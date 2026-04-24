use std::net::IpAddr;
use std::net::TcpStream;
use std::time::Duration;

pub fn scan_port(ip:IpAddr) {
    let timeout = Duration::from_millis(100);
    for port in 1..=65535 {
        let listener = format!("{}:{}", ip , port);
        
        match TcpStream::connect_timeout(&listener.parse().unwrap(), timeout){ Ok(_) => println!("Port {} is OPEN", port) , Err(_) => {}}
        
    }
    println!("Done");
}