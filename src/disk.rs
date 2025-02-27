
use sysinfo::{Disks};
use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::utils::convertor::bytes_to_gb;

pub fn get_physical_disks() -> serde_json::Value {
    let disks = Disks::new_with_refreshed_list();
    let mut real_disks = Vec::new();

    for disk in disks.list() {
        let mount_point = disk.mount_point().to_string_lossy().to_string();
        let device_name = disk.name().to_string_lossy().to_string();
        let total_size = disk.total_space();
        let used_space = total_size - disk.available_space();
        let filesystem = disk.file_system().to_string_lossy().to_string();

        // Exclude system-related and virtual mounts
        if mount_point.starts_with("/dev/loop") || mount_point.contains("/System/Volumes/") {
            continue;
        }

        if (["overlay", "vfat"].contains(&&*filesystem)) {
            continue;
        }

        real_disks.push(json!({
            "device": device_name,
            "mountpoint": mount_point,
            "filesystem": filesystem,
            "total_size": bytes_to_gb(total_size),
            "used": bytes_to_gb(used_space),
            "free": bytes_to_gb(disk.available_space()),
            "percent": (used_space as f64 / total_size as f64) * 100.0
        }));
    }

    json!(real_disks)
}

fn determine_log_path() -> String {
    let log_path = "/var/log/disk_info.json";
    if Path::new("/var/log").exists() && std::fs::metadata("/var/log").is_ok() {
        log_path.to_string()
    } else {
        "/tmp/disk_info.json".to_string()
    }
}

pub(crate) fn write_disk_info() {
    let log_path = determine_log_path();
    let disk_data = get_physical_disks();

    match File::create(&log_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(disk_data.to_string().as_bytes()) {
                eprintln!("Error writing to {}: {}", log_path, e);
            } else {
                println!("Disk information written to {}", log_path);
            }
        }
        Err(_) => {
            eprintln!("Permission denied: Cannot write to {}", log_path);
        }
    }
}
