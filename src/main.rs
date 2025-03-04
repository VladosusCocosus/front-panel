mod disk;
mod utils;
mod cpu;
mod local_ip;
mod serial_communications;

use std::thread::sleep;
use std::time::Duration;
use disk::disk::write_disk_info;
use cpu::cpu::get_system_cpu_refreshed;
use local_ip::local::get_ip;
use serial_communications::port::{connect_controller, send_data, read_from_controller};
use serialport::SerialPort;

enum State {
    LocalIp,
    CPU,
    Disks,
}



fn main() {
    let mut state = State::CPU;
    let mut port = connect_controller();

    loop {
        match state {
            State::LocalIp => {
                send_data(&mut *port, "IP", get_ip()).unwrap();
            }
            State::CPU => {
                let system_info = get_system_cpu_refreshed();
                let cpus = system_info.cpus();

                let core_info_string = cpus.iter()
                    .map(|core| format!("{} - {}%", core.name(), core.cpu_usage().round()))
                    .collect::<Vec<_>>() // Collect into a vector of strings
                    .join("|"); // Use "|" as delimiter instead of "\n"

                send_data(&mut *port, "CPU", core_info_string).unwrap();
            }
            State::Disks => {
                let disk_info = format!("{:?}", write_disk_info());
                send_data(&mut *port, "DISK", disk_info).unwrap();
            }
        }

        // Read response from ESP (optional)
        match read_from_controller(&mut *port) {
            Ok(response) => {
                println!("ESP Response: {}", response);
                if response.contains("GET_CPU") {
                    println!("Sending CPU");
                    state = State::CPU;
                } else if response.contains("GET_IP") {
                    println!("Sending IP");
                    state = State::LocalIp;
                }
            },
            Err(e) => println!("Error reading from port: {}", e),
        }

        sleep(Duration::from_secs(1));
    }
}
