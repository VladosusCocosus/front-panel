mod disk;
mod utils;

use disk::write_disk_info;
use serde_json::to_string_pretty;

fn main() {
    let disk_data = write_disk_info();
    let json_data = to_string_pretty(&disk_data).unwrap();
    println!("{}", json_data);
}
