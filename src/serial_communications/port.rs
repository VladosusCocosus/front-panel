use serialport::{SerialPort};
use std::io::{self};
use std::thread::sleep;
use std::time::Duration;

pub fn connect_to_controller () -> Option<Box<dyn SerialPort>> {
    match serialport::available_ports() {
        Ok(ports) => {
            for p in ports {
                println!("{:?}", p.port_name);
                if p.port_name != "/dev/cu.usbserial-0001" {
                    continue;
                }
                println!("Opening serial port: {}", p.port_name);
                let port = serialport::new(p.port_name, 115_200)
                    .timeout(std::time::Duration::from_millis(10))
                    .open();

                return port.ok(); // Convert Result to Option
            }
        }
        Err(e) => {
            println!("Error opening serial port: {:?}", e);
        }
    }
    None // Ensure function returns None if no port is found
}

pub fn read_from_controller(port: &mut dyn SerialPort) -> io::Result<String> {
    let mut serial_buf = vec![0; 32]; // Buffer to store read data
    let bytes_read = port.read(&mut serial_buf)?; // Read from serial port

    // Convert only the received part of buffer to String
    Ok(String::from_utf8_lossy(&serial_buf[..bytes_read]).to_string())
}


pub fn send_data(port: &mut dyn SerialPort, label: &str, data: String) -> std::io::Result<()> {
    let formatted_data = format!("{}: {}\n", label, data);
    port.write_all(formatted_data.as_bytes())?;
    port.flush()?;
    Ok(())
}

pub fn connect_controller() -> Box<dyn SerialPort> {
    loop {
        println!("Retrying connection...");
        if let Some(port) = connect_to_controller() {
            return port;
        }
        sleep(Duration::from_secs(5));
    }
}

