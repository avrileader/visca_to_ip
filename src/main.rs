#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::UdpSocket;
use visca_to_ip::{config, separator};

const HEAD: [u8; 8]= [01,00,00,09,00,00,00,01];
const CLEAR: [u8; 9]= [02,00,00,01,00,00,00,00,01];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let separator = separator();
    let config = config("config.yaml".to_string());
    let recv_socket = UdpSocket::bind(config.local).expect("bind failed");
    let send_socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");
    loop{
        let mut buf = [0u8; 9];
        let (amt , src)= recv_socket.recv_from(&mut buf).expect("recv failed");
        let buf = &mut buf[..amt];
        if buf[4] == 2 {
            buf[4] = 33;
        } else if buf[4] == 3 {
            buf[4] = 23;
        }
        let merged = [&HEAD[..], &buf[..]].concat();
        println!("source port: {}\ndata stream: {:?}\n{}", src, merged, separator);
        send_socket.send_to(&CLEAR, config.remote.clone()).expect("send failed");
        send_socket.send_to(&merged, config.remote.clone()).expect("send failed");
    }
}