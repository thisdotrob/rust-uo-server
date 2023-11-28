use byteorder::{BigEndian, ByteOrder};
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;

pub fn start() {
    let listener = TcpListener::bind("127.0.0.1:2593").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        parse_packets(buffer, &mut stream);
    }
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

fn send_server_list_packet(stream: &mut TcpStream) {
    let mut buffer: [u8; 46] = [0; 46];

    buffer[0] = 0xA8; // packet ID

    buffer[2] = 0x2E; // packet length

    buffer[3] = 0x00; // flags (unused, ServUO uses 0x5D

    BigEndian::write_u16(&mut buffer[4..6], 1); // server count

    BigEndian::write_u16(&mut buffer[6..8], 0); // server index

    buffer[8..16].copy_from_slice("My Shard".as_bytes()); // server name

    buffer[37] = 0x00; // server percent full

    // server timezone
    buffer[38] = 0x00;
    buffer[39] = 0x00;
    buffer[40] = 0x00;
    buffer[41] = 0x00;

    // server address
    buffer[42] = 0x01;
    buffer[43] = 0x00;
    buffer[44] = 0x00;
    buffer[45] = 0x7F;

    stream.write_all(&buffer).unwrap();
    stream.flush().unwrap();

    println!("\nSent Server List packet: {:X?}", buffer);
}

fn send_server_redirect_packet(stream: &mut TcpStream) {
    // 8c 7f 00 00 01 0a 21 43

    let mut buffer: [u8; 11] = [0; 11];

    buffer[0] = 0x8C; // packet ID

    // server address
    buffer[1] = 0x7F; // 127;
    buffer[2] = 0x00; // 0;
    buffer[3] = 0x00; // 0;
    buffer[4] = 0x01; // 1;

    // server port
    buffer[5] = 0x0A; // 10;
    buffer[6] = 0x21; // 33;

    // encryption key
    buffer[7] = 0x43; // copied from a ServUO sample packet
    buffer[8] = 0x2F;
    buffer[9] = 0x3F;
    buffer[10] = 0xF0;

    stream.write_all(&buffer).unwrap();
    stream.flush().unwrap();

    println!("\nSent Server Redirect packet: {:X?}", buffer);
}

fn send_features_packet(stream: &mut TcpStream) {
    let mut buffer: [u8; 3] = [0; 3];

    buffer[0] = 0xB9; // packet ID

    // flags
    buffer[1] = 0x00;
    buffer[2] = 0x00;

    stream.write_all(&buffer).unwrap();
    stream.flush().unwrap();

    println!("\nSent Features packet: {:X?}", buffer);
}

fn send_character_list_packet(stream: &mut TcpStream) {
    let mut buffer: [u8; 6] = [0; 6];

    buffer[0] = 0xA9; // packet ID
                      //
    buffer[1] = 6; // packet size

    buffer[2] = 0x00; // number of characters

    buffer[3] = 0x00; // number of cities

    // flags
    buffer[4] = 0x00;
    buffer[5] = 0x00;

    stream.write_all(&buffer).unwrap();
    stream.flush().unwrap();

    println!("\nSent Character List packet: {:X?}", buffer);
}

fn parse_packets(buffer: [u8; 1024], mut stream: &mut TcpStream) {
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
                send_server_list_packet(&mut stream);
            }
            0xA0 => {
                handle_server_select_packet(&mut buffer_slice);
                send_server_redirect_packet(&mut stream);
            }
            0x91 => {
                handle_post_login_packet(&mut buffer_slice);
                send_features_packet(&mut stream);
                send_character_list_packet(&mut stream);
            }
            0x73 => continue,
            _ => continue,
        }
    }

    println!("\n======== Finished parsing packet. ========\n");
}
