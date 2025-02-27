use sysinfo::{Disks};
use serde_json::{json, Value};
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

        if ["overlay", "vfat"].contains(&&*filesystem) {
            continue;
        }

        if mount_point.starts_with("/dev/loop") || mount_point.contains("/System/Volumes/") {
            continue;
        }

        real_disks.push(json!({
            "device": device_name,
            "mountpoint": mount_point,
            "filesystem": filesystem,
            "total_size": bytes_to_gb(total_size).round(),
            "used": bytes_to_gb(used_space).round(),
            "free": bytes_to_gb(disk.available_space()).round(),
            "percent": ((used_space as f64 / total_size as f64) * 100.0).round()
        }));
    }

    json!(real_disks)
}

pub(crate) fn write_disk_info() -> Value {
    get_physical_disks()
}
