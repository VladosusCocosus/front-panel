mod disk;
mod utils;
mod cpu;

use disk::disk::write_disk_info;
use cpu::cpu::get_cpu_info;
use serde_json::json;

fn main() {
    println!("{}", json!({
        "disks": write_disk_info(),
        "cpus": get_cpu_info(),
    }));
}
