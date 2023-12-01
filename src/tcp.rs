use std::str;

use async_std::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};

use crate::huffman;

mod packets;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn connection_loop(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];

    while let Ok(received) = stream.read(&mut buffer).await {
        if received == 0 {
            let addr = stream.peer_addr().unwrap();
            println!("Connection closed by: {}", addr);
            break;
        } else {
            parse_packets(buffer, &mut stream).await?;
            buffer = [0; 1024];
        }
    }
    Ok(())
}

async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        let addr = stream.peer_addr()?;
        println!("Connection received from: {}", addr);
        task::spawn(connection_loop(stream));
    }
    Ok(())
}

pub fn start() -> Result<()> {
    let fut = accept_loop("127.0.0.1:2593");
    task::block_on(fut)
}

fn read_u8(input: &mut &[u8]) -> u8 {
    let (int_bytes, rest) = input.split_at(1);
    *input = rest;
    return *&int_bytes[0];
}

fn read_u16(input: &mut &[u8]) -> u16 {
    let (int_bytes, rest) = input.split_at(2);
    *input = rest;
    u16::from_be_bytes(
        int_bytes
            .try_into()
            .expect("int_bytes should always be two bytes long"),
    )
}

fn read_u32(input: &mut &[u8]) -> u32 {
    let (int_bytes, rest) = input.split_at(4);
    *input = rest;
    u32::from_be_bytes(int_bytes.try_into().unwrap())
}

fn read_string<'a>(input: &'a mut &[u8], length: u8) -> &'a str {
    let (string_bytes, rest) = input.split_at(length.into());
    *input = rest;
    return str::from_utf8(string_bytes).unwrap();
}

fn handle_encrypted_login_seed_packet(buffer_slice: &mut &[u8]) {
    println!("\nEncrypted Login Seed packet received:");
    let packet_length = 20;
    let (mut bytes, rest) = buffer_slice.split_at(packet_length);
    *buffer_slice = rest;
    let seed = read_u32(&mut bytes);
    println!("seed: {}", seed);
    let major = read_u32(&mut bytes);
    let minor = read_u32(&mut bytes);
    let revision = read_u32(&mut bytes);
    let patch = read_u32(&mut bytes);
    println!("client version: {}.{}.{}.{}", major, minor, revision, patch);
}

fn handle_account_login_request_packet(buffer_slice: &mut &[u8]) {
    println!("\nAccount Login Request packet received:");
    let packet_length = 61;
    let (mut bytes, rest) = buffer_slice.split_at(packet_length);
    *buffer_slice = rest;
    let username = read_string(&mut bytes, 30);
    println!("username: {}", username);
    let password = read_string(&mut bytes, 30);
    println!("password: {}", password);
}

fn handle_server_select_packet(buffer_slice: &mut &[u8]) {
    println!("\nServer Select packet received:");
    let packet_length = 2;
    let (mut bytes, rest) = buffer_slice.split_at(packet_length);
    *buffer_slice = rest;
    let server_index = read_u16(&mut bytes);
    println!("server_index: {}", server_index);
}

fn handle_post_login_packet(buffer_slice: &mut &[u8]) {
    println!("Post Login packet received:");
    let packet_length = 64;
    let (mut bytes, rest) = buffer_slice.split_at(packet_length);
    *buffer_slice = rest;
    let encryption_key = read_u32(&mut bytes);
    println!("encryption_key: {}, ", encryption_key);
    let username = read_string(&mut bytes, 30);
    println!("username: {}, ", username);
    let password = read_string(&mut bytes, 30);
    println!("password: {}", password);
}

async fn send_server_list_packet(stream: &mut TcpStream) -> Result<()> {
    let buffer = packets::server_list_packet();

    stream.write_all(&buffer).await?;
    stream.flush().await?;

    println!("\nSent Server List packet: {:X?}", buffer);

    Ok(())
}

async fn send_server_redirect_packet(stream: &mut TcpStream) -> Result<()> {
    let buffer = packets::server_redirect_packet();

    stream.write_all(&buffer).await?;
    stream.flush().await?;

    println!("\nSent Server Redirect packet: {:X?}", buffer);

    Ok(())
}

async fn send_features_packet(stream: &mut TcpStream) -> Result<()> {
    let src = packets::features_packet();

    let mut output = Vec::new();

    println!("\nCompressing Features packet: {:X?}", src);
    huffman::compress(src, &mut output);

    stream.write_all(&output).await?;
    stream.flush().await?;

    println!("\nSent compressed Features packet: {:X?}", output);

    Ok(())
}

async fn send_character_list_packet(stream: &mut TcpStream) -> Result<()> {
    let src = packets::character_list_packet();

    let mut output = Vec::new();

    println!("\nCompressing Character List packet: {:02X?}", src);
    huffman::compress(src, &mut output);

    stream.write_all(&output).await?;
    stream.flush().await?;

    println!("\nSent compressed Character List packet: {:X?}", output);

    Ok(())
}

async fn parse_packets(buffer: [u8; 1024], mut stream: &mut TcpStream) -> Result<()> {
    let mut buffer_slice = &buffer[..];

    println!("\n============= Parsing packet =============\n");

    while buffer_slice.len() > 0 {
        let packet_id = read_u8(&mut buffer_slice);

        match packet_id {
            0xEF => {
                handle_encrypted_login_seed_packet(&mut buffer_slice);
            }
            0x80 => {
                handle_account_login_request_packet(&mut buffer_slice);
                send_server_list_packet(&mut stream).await?;
            }
            0xA0 => {
                handle_server_select_packet(&mut buffer_slice);
                send_server_redirect_packet(&mut stream).await?;
            }
            0x91 => {
                handle_post_login_packet(&mut buffer_slice);
                send_features_packet(&mut stream).await?;
                send_character_list_packet(&mut stream).await?;
            }
            0x73 => continue,
            _ => continue,
        }
    }

    println!("\n======== Finished parsing packet. ========\n");

    Ok(())
}
