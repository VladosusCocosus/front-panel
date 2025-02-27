use std::net::IpAddr;
use local_ip_address::local_ip;

pub fn get_ip() -> IpAddr {
    let my_local_ip = local_ip().unwrap();

    my_local_ip
}