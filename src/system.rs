use sysinfo;
use gfxinfo;
use std::thread;
use serde::Deserialize;
use wmi::WMIConnection;

pub fn get_cpu_model() -> String {
    let sys = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::nothing().with_cpu(sysinfo::CpuRefreshKind::everything()
    )).expect("Cannot get a System class from OS");
    let cpu_brand: String = sys.cpus().first().map(|cpu| cpu.brand().to_string()).unwrap_or_else(|| "Unknown CPU".to_string());
    cpu_brand
}

pub fn get_cpu_usage() -> f32 {
    let mut sys = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::nothing().with_cpu(sysinfo::CpuRefreshKind::everything()),
    ).expect("Cannot get a System class from OS");
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_usage();
    sys.global_cpu_usage()
}

pub fn get_total_ram() -> String {
    let sys = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::nothing()
            .with_memory(sysinfo::MemoryRefreshKind::everything())
    ).expect("Cannot get a System class from OS");
    let total_bytes = sys.total_memory();
    let total_gb = total_bytes as f64 / 1_073_741_824.0;
    let memory_info = format!("{:.2} GB", total_gb);
    memory_info
}

pub fn get_ram_use() -> String {
    let sys = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::nothing()
            .with_memory(sysinfo::MemoryRefreshKind::everything())
    ).expect("Cannot get a System class from OS");
    let used_bytes = sys.used_memory();
    let used_gb = used_bytes as f64 / 1_073_741_824.0;
    let memory_info = format!("{:.2} GB", used_gb);
    memory_info
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct PhysicalMemory {
    manufacturer: Option<String>,
    part_number: Option<String>,
    capacity: Option<u64>,
    speed: Option<u32>,
}

pub fn get_ram_model() -> Vec<String> {
    let Ok(wmi_con) = WMIConnection::new() else {
        return vec!["Error with connection to WMI".to_string()];
    };
    let results: Vec<PhysicalMemory> = wmi_con
        .raw_query("SELECT Manufacturer, PartNumber, Capacity, Speed FROM Win32_PhysicalMemory")
        .unwrap_or_default();
    results
        .into_iter()
        .map(|ram| {
            let vendor = ram.manufacturer.unwrap_or_default().trim().to_string();
            let model = ram.part_number.unwrap_or_else(|| "Unknown".to_string()).trim().to_string();
            let gb = ram.capacity.unwrap_or(0) / 1_073_741_824;
            let mhz = ram.speed.unwrap_or(0);
            format!("{} {} ({} GB, {} MHz)", vendor, model, gb, mhz)
        })
        .collect()
}

pub fn get_gpu_model() -> String {
    let Ok(gpu) = gfxinfo::active_gpu() else {
        return "Unknown GPU".to_string();
    };
    let model = format!("{} {}", gpu.vendor(), gpu.model());
    model
}

pub fn get_gpu_usage() -> u32 {
    let Ok(gpu) = gfxinfo::active_gpu() else {
        return 0;
    };
    let load = gpu.info().load_pct();
    load
}

pub fn get_disks_info() -> Vec<String> {
    let disks = sysinfo::Disks::new_with_refreshed_list()
        .expect("Cannot get Disks");
    disks
        .iter()
        .map(|disk| {
            let mount = disk.mount_point().to_string_lossy();
            let total_gb = disk.total_space() as f64 / 1_073_741_824.0;
            let used_gb = (disk.total_space() - disk.available_space()) as f64 / 1_073_741_824.0;
            let percent = if total_gb > 0.0 {
                (used_gb / total_gb) * 100.0
            } else {
                0.0
            };
            format!("{} {:.2} GB / {:.2} GB ({:.1}%)", mount, used_gb, total_gb, percent)
        })
        .collect()
}

pub fn get_os_name() -> String {
    let os_name = sysinfo::System::long_os_version().unwrap_or_else(|_| "Unknown OS".to_string());
    os_name
}