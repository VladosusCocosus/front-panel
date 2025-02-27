use serde_json::{json, Value};
use sysinfo::{System, RefreshKind, CpuRefreshKind};

pub fn get_cpu_info () -> Vec<Value> {
    let mut cpus = Vec::new();
    let mut system = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
    );

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    system.refresh_cpu_all();

    for cpu in system.cpus() {
        cpus.push(json!({
            "brand": cpu.brand(),
            "name":  cpu.name(),
            "usage": cpu.cpu_usage()
        }));
    }
    
    return cpus
}