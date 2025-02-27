use std::io::Read;
use serialport::{SerialPort};

pub fn connect_to_controller () -> Option<Box<dyn SerialPort>> {
    match serialport::available_ports() {
        Ok(ports) => {
            for p in ports {
                if p.port_name != "/dev/ttyUSB0" {
                    continue;
                }
                let port = serialport::new(p.port_name, 115_200)
                    .timeout(std::time::Duration::from_millis(10))
                    .open();

                return port.ok(); // Convert Result to Option
            }
        }
        Err(_) => {}
    }
    None // Ensure function returns None if no port is found
}

pub fn read_from_controller(port: &mut Option<Box<dyn SerialPort>>) -> Result<(usize, Vec<u8>), std::io::Error> {
    let mut serial_buf: Vec<u8> = vec![0; 32];

    if let Some(p) = port.as_mut() {
        let bytes_read = p.read(serial_buf.as_mut_slice())?;
        Ok((bytes_read, serial_buf))
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "No serial port connected"))
    }
}
