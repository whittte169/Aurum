
use serde::Deserialize;
use wmi::WMIConnection;

pub fn get_cpu_model() -> String {
    let Ok(sys) = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::nothing()
            .with_cpu(sysinfo::CpuRefreshKind::everything()),
    ) else {
        return "Unknown CPU".to_string();
    };

    sys.cpus()
        .first()
        .map(|cpu| cpu.brand().to_string())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| "Unknown CPU".to_string())
}

pub fn get_cpu_usage() -> f32 {
    let Ok(mut sys) = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::nothing()
            .with_cpu(sysinfo::CpuRefreshKind::everything()),
    ) else {
        return 0.0;
    };

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_usage();

    sys.global_cpu_usage()
}

pub fn get_total_ram() -> String {
    let Ok(sys) = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::nothing()
            .with_memory(sysinfo::MemoryRefreshKind::everything()),
    ) else {
        return "Unknown".to_string();
    };

    format!(
        "{:.2} GB",
        sys.total_memory() as f64 / 1_073_741_824.0
    )
}

pub fn get_ram_use() -> String {
    let Ok(sys) = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::nothing()
            .with_memory(sysinfo::MemoryRefreshKind::everything()),
    ) else {
        return "Unknown".to_string();
    };

    format!(
        "{:.2} GB",
        sys.used_memory() as f64 / 1_073_741_824.0
    )
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PhysicalMemory {
    manufacturer: Option<String>,
    part_number: Option<String>,
    capacity: Option<u64>,
    speed: Option<u32>,
}

pub fn get_ram_model() -> Vec<String> {
    let Ok(wmi) = WMIConnection::new() else {
        return vec!["Unable to connect to WMI".to_string()];
    };

    let results: Vec<PhysicalMemory> = wmi
        .raw_query(
            "SELECT Manufacturer, PartNumber, Capacity, Speed FROM Win32_PhysicalMemory",
        )
        .unwrap_or_default();

    if results.is_empty() {
        return vec!["No memory modules found".to_string()];
    }

    results
        .into_iter()
        .map(|ram| {
            let vendor = ram
                .manufacturer
                .unwrap_or_default()
                .trim()
                .to_string();

            let model = ram
                .part_number
                .unwrap_or_else(|| "Unknown".to_string())
                .trim()
                .to_string();

            let capacity =
                ram.capacity.unwrap_or(0) as f64 / 1_073_741_824.0;

            let speed = ram.speed.unwrap_or(0);

            format!("{vendor} {model} ({capacity:.0} GB, {speed} MHz)")
        })
        .collect()
}

pub fn get_gpu_model() -> String {
    let Ok(gpu) = gfxinfo::active_gpu() else {
        return "Unknown GPU".to_string();
    };

    format!("{} {}", gpu.vendor(), gpu.model())
}

pub fn get_gpu_usage() -> u32 {
    let Ok(gpu) = gfxinfo::active_gpu() else {
        return 0;
    };

    gpu.info().load_pct()
}

pub fn get_disks_info() -> Vec<String> {
    let Ok(disks) = sysinfo::Disks::new_with_refreshed_list() else {
        return vec!["Unable to retrieve disk information".to_string()];
    };

    disks
        .iter()
        .map(|disk| {
            let mount = disk.mount_point().to_string_lossy();
            let total = disk.total_space() as f64 / 1_073_741_824.0;
            let used = (disk.total_space() - disk.available_space()) as f64
                / 1_073_741_824.0;

            let percent = if total > 0.0 {
                used / total * 100.0
            } else {
                0.0
            };

            format!("{mount} {used:.2} GB / {total:.2} GB ({percent:.1}%)")
        })
        .collect()
}

pub fn get_os_name() -> String {
    sysinfo::System::long_os_version()
        .unwrap_or_else(|_| "Unknown OS".to_string())
}
