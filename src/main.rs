mod components;

use std::net::{IpAddr, TcpListener, TcpStream};
use std::io::{Result, Error};
use std::time::Duration;
use local_ip_address::local_ip;
use components::port::scan_port;

fn get_my_ip() -> Result<IpAddr>{
    match local_ip(){
        Ok(ip) => {println!("{}", ip);Ok(ip)},
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
    }
}



fn main() {
    let ip = get_my_ip();
    println!("Scanning {:?}", ip);
    scan_port(ip.unwrap());  
}
