use sysinfo::{System, RefreshKind, CpuRefreshKind, Cpu};

pub fn get_system_cpu_refreshed () -> System {
    let mut system = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
    );

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    system.refresh_cpu_all();


    system
}