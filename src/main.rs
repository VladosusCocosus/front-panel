mod disk;
mod utils;
mod cpu;
mod local_ip;
mod serial_communications;

use std::thread::sleep;
use std::time::Duration;
use disk::disk::write_disk_info;
use cpu::cpu::get_cpu_info;
use local_ip::local::get_ip;
use serial_communications::port::connect_to_controller;
use crate::serial_communications::port;

enum State {
    CPU,
    Disks,
    LocalIp,
}

fn main() {
    let mut state = State::LocalIp;

    let mut controller = connect_to_controller();

    loop {
        match state {
            State::LocalIp => {
                println!("{}", get_ip())
            },
            State::CPU => {
                println!("{:?}", get_cpu_info())
            }
            State::Disks => {
                println!("{:?}", write_disk_info())
            }
        }

        match port::read_from_controller(&mut controller) {
            Ok((bytes, data)) => {
                let received_str = String::from_utf8_lossy(&data[..bytes]); // Convert bytes to string
                println!("Read {} bytes: {}", bytes, received_str);
            }
            Err(e) => println!("Error reading from controller: {}", e),
        }

        sleep(Duration::from_secs(5));
    }
}
