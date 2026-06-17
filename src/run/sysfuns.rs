use sysinfo::{Components, Disks, MINIMUM_CPU_UPDATE_INTERVAL, Networks, Pid, ProcessRefreshKind, System};
use std::thread;

//conversions
pub fn kb_to_gb(kb: u64) -> u64 {
    return (kb) / (1024 * 1024);
}

pub fn b_to_gb(b: u64) -> u64 {
    return (b) / (1024 * 1024 * 1024);
}

//Data to string

pub fn format_bytes(b: u64) -> String {
    if b < 1024             { format!("{} B", b) }
    else if b < 1<<20       { format!("{:.1} KB", b as f64 / 1024.0) }
    else if b < 1<<30       { format!("{:.1} MB", b as f64 / 1_048_576.0) }
    else                    { format!("{:.2} GB", b as f64 / 1_073_741_824.0) }
}

//components infos

pub struct ComponentInfo{
    pub label: String,
    pub temperature: f32,
}

pub fn componentinfo() -> Vec<ComponentInfo> {

    let components = Components::new_with_refreshed_list();
    
    let mut components_info = Vec::new();

    for component in components.list() {
        let label = component.label().to_string();
        let temperature = component.temperature();

        components_info.push(ComponentInfo {
            label,
            temperature,
        });
    }

    return components_info
    
}

//RAM infos

pub struct RamInfo {
    pub total_gb: u64,
    pub used_gb: u64,
    pub total_swap_gb: u64,
    pub used_swap_gb: u64,
}

pub fn raminfo(sys: &System) -> RamInfo {
    let totalgb = kb_to_gb(sys.total_memory());
    let usedgb = kb_to_gb(sys.used_memory());
    let totalswapgb = kb_to_gb(sys.total_swap());
    let usedswapgb = kb_to_gb(sys.used_swap());

    return RamInfo {
        total_gb: totalgb,
        used_gb: usedgb,
        total_swap_gb: totalswapgb,
        used_swap_gb: usedswapgb,
    };

}

//CPU infos

pub struct CpuInfo {
    pub usage_percent: f32,
    pub num_cpus: usize,
    pub frequency_mhz: u64,
}

pub fn cpuinfo(sys: &System) -> CpuInfo {
    let usage = sys.global_cpu_info().cpu_usage();
    let cpus = sys.cpus().len();
    let frequency = sys.cpus().first().map(|c| c.frequency()).unwrap_or(0);

    return CpuInfo {
        usage_percent: usage,
        num_cpus: cpus,
        frequency_mhz: frequency,
    };
}

//disks infos

pub struct DiskInfo {
    pub name: String,
    pub total_space_gb: u64,
    pub available_space_gb: u64,
}

pub fn diskinfo() -> Vec<DiskInfo> {
    let mut disks_info = Vec::new();

    let disks = Disks::new_with_refreshed_list();

    for disk in disks.list() {
        let name = disk.name().to_string_lossy().into_owned();
        let total_space_gb = b_to_gb(disk.total_space());
        let available_space_gb = b_to_gb(disk.available_space());

        disks_info.push(DiskInfo {
            name,
            total_space_gb,
            available_space_gb,
        });
    }

    return disks_info;
}

//Networks infos

pub struct NetworkInfo {
    pub interface_name: String,
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
}

pub fn networkinfo() -> Vec<NetworkInfo> {
    let mut networks = Networks::new_with_refreshed_list();

    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    networks.refresh();

    let net_infos: Vec<NetworkInfo> = networks.list().iter().map(|(name, data)| {
        NetworkInfo {
            interface_name: name.to_string(),
            received_bytes: data.received(),
            transmitted_bytes: data.transmitted(),
        }
    }).collect();

    return net_infos;
    
}

//process infos

pub struct ProcessInfo {
    pub pid: Pid,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
}

pub fn processinfo(sys: &mut System) -> Vec<ProcessInfo> {

    let refresh = ProcessRefreshKind::new().with_cpu().with_memory().without_cmd().without_cwd().without_disk_usage().without_exe().without_user();

    sys.refresh_processes_specifics(refresh);
    thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_processes_specifics(refresh);
    

    let num_cpus = cpuinfo(sys).num_cpus as f32;

    let processes = sys.processes().iter().map(|(pid, process)| ProcessInfo {
        pid: *pid,
        name: process.name().to_string(),
        cpu_usage: (process.cpu_usage() / num_cpus),
        memory: process.memory(),
    }).collect();
    
    return processes;
}

//search process by name

pub fn search_process(name: String) -> ProcessInfo {
    let mut sys = System::new_all();
    let target = processinfo(&mut sys).into_iter().find(|p| p.name == name).expect("PROCESS NOT FOUND!\n");

    return target;

}

//look for a process by a keyword

pub fn lookfor_process(word: String) -> Vec<ProcessInfo> {
    let mut sys = System::new_all();
    let process_list = processinfo(&mut sys);
    let mut target_list: Vec<ProcessInfo> = Vec::new();
    for process in process_list {
        if process.name.contains(&word){
            target_list.push(process);
        }
        continue;
    }
    return target_list;
}
