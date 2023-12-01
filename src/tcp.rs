use byteorder::{BigEndian, ByteOrder};
use std::str;

use async_std::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};

use crate::huffman::{Huffman, HuffmanTable, TerminalCode};

mod packets;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const SERVUO_HUFFMAN_TABLE_VALUES: [u32; 256] = [
    0x000, 0x01F, 0x022, 0x034, 0x075, 0x028, 0x03B, 0x032, 0x0E0, 0x062, 0x056, 0x079, 0x19D,
    0x097, 0x02A, 0x057, 0x071, 0x05B, 0x1CC, 0x0A7, 0x025, 0x04F, 0x066, 0x07D, 0x191, 0x1CE,
    0x03F, 0x090, 0x059, 0x07B, 0x091, 0x0C6, 0x02D, 0x186, 0x06F, 0x093, 0x1CC, 0x05A, 0x1AE,
    0x1C0, 0x148, 0x14A, 0x082, 0x19F, 0x171, 0x120, 0x0E7, 0x1F3, 0x14B, 0x100, 0x190, 0x013,
    0x161, 0x125, 0x133, 0x195, 0x173, 0x1CA, 0x086, 0x1E9, 0x0DB, 0x1EC, 0x08B, 0x085, 0x00A,
    0x096, 0x09C, 0x1C3, 0x19C, 0x08F, 0x18F, 0x091, 0x087, 0x0C6, 0x177, 0x089, 0x0D6, 0x08C,
    0x1EE, 0x1EB, 0x084, 0x164, 0x175, 0x1CD, 0x05E, 0x088, 0x12B, 0x172, 0x10A, 0x08D, 0x13A,
    0x11C, 0x1E1, 0x1E0, 0x187, 0x1DC, 0x1DF, 0x074, 0x19F, 0x08D, 0x0E4, 0x079, 0x0EA, 0x0E1,
    0x040, 0x041, 0x10B, 0x0B0, 0x06A, 0x0C1, 0x071, 0x078, 0x0B1, 0x14C, 0x043, 0x076, 0x066,
    0x04D, 0x08A, 0x02F, 0x0C9, 0x0CE, 0x149, 0x160, 0x1BA, 0x19E, 0x39F, 0x0E5, 0x194, 0x184,
    0x126, 0x030, 0x06C, 0x121, 0x1E8, 0x1C1, 0x11D, 0x163, 0x385, 0x3DB, 0x17D, 0x106, 0x397,
    0x24E, 0x02E, 0x098, 0x33C, 0x32E, 0x1E9, 0x0BF, 0x3DF, 0x1DD, 0x32D, 0x2ED, 0x30B, 0x107,
    0x2E8, 0x3DE, 0x125, 0x1E8, 0x0E9, 0x1CD, 0x1B5, 0x165, 0x232, 0x2E1, 0x3AE, 0x3C6, 0x3E2,
    0x205, 0x29A, 0x248, 0x2CD, 0x23B, 0x3C5, 0x251, 0x2E9, 0x252, 0x1EA, 0x3A0, 0x391, 0x23C,
    0x392, 0x3D5, 0x233, 0x2CC, 0x390, 0x1BB, 0x3A1, 0x3C4, 0x211, 0x203, 0x12A, 0x231, 0x3E0,
    0x29B, 0x3D7, 0x202, 0x3AD, 0x213, 0x253, 0x32C, 0x23D, 0x23F, 0x32F, 0x11C, 0x384, 0x31C,
    0x17C, 0x30A, 0x2E0, 0x276, 0x250, 0x3E3, 0x396, 0x18F, 0x204, 0x206, 0x230, 0x265, 0x212,
    0x23E, 0x3AC, 0x393, 0x3E1, 0x1DE, 0x3D6, 0x31D, 0x3E5, 0x3E4, 0x207, 0x3C7, 0x277, 0x3D4,
    0x0C0, 0x162, 0x3DA, 0x124, 0x1B4, 0x264, 0x33D, 0x1D1, 0x1AF, 0x39E, 0x24F, 0x373, 0x249,
    0x372, 0x167, 0x210, 0x23A, 0x1B8, 0x3AF, 0x18E, 0x2EC, 0x062,
];

const SERVUO_HUFFMAN_TABLE_BIT_COUNTS: [u8; 256] = [
    2, 5, 6, 7, 7, 6, 6, 7, 8, 8, 7, 8, 9, 8, 6, 7, 8, 8, 9, 8, 7, 7, 8, 8, 9, 9, 7, 9, 8, 8, 8, 8,
    6, 9, 8, 9, 10, 8, 10, 10, 9, 9, 9, 10, 9, 9, 9, 10, 9, 9, 9, 6, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 5, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 8, 9, 9, 9, 9, 9, 9, 9, 10,
    10, 9, 10, 10, 7, 9, 8, 8, 7, 9, 9, 8, 7, 9, 9, 8, 8, 7, 7, 8, 9, 7, 8, 7, 7, 9, 6, 8, 9, 9, 9,
    10, 10, 10, 9, 9, 9, 9, 7, 8, 9, 9, 10, 10, 10, 10, 10, 10, 10, 10, 10, 7, 8, 10, 10, 10, 9,
    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 9, 10, 10, 9, 10, 10, 11, 11, 11, 10, 10, 10, 10, 10,
    11, 10, 10, 10, 9, 11, 11, 10, 11, 11, 10, 10, 11, 10, 11, 11, 10, 10, 9, 10, 11, 10, 11, 10,
    11, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 11, 10, 10, 10, 10, 10, 10, 10, 10,
    11, 11, 11, 10, 11, 10, 11, 11, 10, 11, 10, 11, 8, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 11,
    10, 11, 9, 10, 10, 10, 11, 10, 10, 7,
];

const UO_TERMINAL_CODE_BIT_COUNT: u8 = 4;

const UO_TERMINAL_CODE_VALUE: u32 = 0xD;

async fn connection_loop(mut stream: TcpStream) -> Result<()> {
    let table = HuffmanTable {
        values: SERVUO_HUFFMAN_TABLE_VALUES,
        bit_counts: SERVUO_HUFFMAN_TABLE_BIT_COUNTS,
    };

    let terminal_code = TerminalCode {
        bit_count: UO_TERMINAL_CODE_BIT_COUNT,
        value: UO_TERMINAL_CODE_VALUE,
    };

    let mut huffman = Huffman::new(table, Some(terminal_code));

    let mut buffer = [0; 1024];

    while let Ok(received) = stream.read(&mut buffer).await {
        if received == 0 {
            let addr = stream.peer_addr().unwrap();
            println!("Connection closed by: {}", addr);
            break;
        } else {
            parse_packets(buffer, &mut stream, &mut huffman).await?;
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
    buffer[42] = 0x7F;
    buffer[43] = 0x00;
    buffer[44] = 0x00;
    buffer[45] = 0x01;

    stream.write_all(&buffer).await?;
    stream.flush().await?;

    println!("\nSent Server List packet: {:X?}", buffer);

    Ok(())
}

async fn send_server_redirect_packet(stream: &mut TcpStream) -> Result<()> {
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

    stream.write_all(&buffer).await?;
    stream.flush().await?;

    println!("\nSent Server Redirect packet: {:X?}", buffer);

    Ok(())
}

async fn send_features_packet(stream: &mut TcpStream, huffman: &mut Huffman) -> Result<()> {
    let src = vec![
        0xB9, // packet ID
        0x00, 0xFF, 0x92, 0xDB, // flags
    ];

    let mut output = Vec::new();

    println!("\nCompressing Features packet: {:X?}", src);
    huffman.compress(src, &mut output);

    stream.write_all(&output).await?;
    stream.flush().await?;

    println!("\nSent compressed Features packet: {:X?}", output);

    Ok(())
}

async fn send_character_list_packet(stream: &mut TcpStream, huffman: &mut Huffman) -> Result<()> {
    let src = packets::character_list_packet();

    let mut output = Vec::new();

    println!("\nCompressing Character List packet: {:02X?}", src);
    huffman.compress(src, &mut output);

    stream.write_all(&output).await?;
    stream.flush().await?;

    println!("\nSent compressed Character List packet: {:X?}", output);

    Ok(())
}

async fn parse_packets(
    buffer: [u8; 1024],
    mut stream: &mut TcpStream,
    huffman: &mut Huffman,
) -> Result<()> {
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
                send_features_packet(&mut stream, huffman).await?;
                send_character_list_packet(&mut stream, huffman).await?;
            }
            0x73 => continue,
            _ => continue,
        }
    }

    println!("\n======== Finished parsing packet. ========\n");

    Ok(())
}
