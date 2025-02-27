mod disk;
mod utils;
mod cpu;
mod local_ip;

use disk::disk::write_disk_info;
use cpu::cpu::get_cpu_info;
use serde_json::json;
use local_ip::local::get_ip;

fn main() {
    println!("{}", json!({
        "disks": write_disk_info(),
        "cpus": get_cpu_info(),
        "local_ip": get_ip()
    }));
}
