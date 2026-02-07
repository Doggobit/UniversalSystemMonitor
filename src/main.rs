use sysinfo::{Components, Disks, Networks, System, Pid};
use std::{env, process, str};


fn kb_to_gb(kb: u64) -> u64 {
    return (kb) / (1024 * 1024);
}

//fn for RAM infos

struct RamInfo {
    total_gb: u64,
    used_gb: u64,
    total_swap_gb: u64,
    used_swap_gb: u64,
}

fn raminfo(sys: &System) -> RamInfo {
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

struct CpuInfo {
    usage_percent: f32,
    num_cpus: usize,
    frequency_mhz: u64,
}

fn cpuinfo(sys: &System) -> CpuInfo {
    let usage = sys.global_cpu_info().cpu_usage();
    let cpus = sys.cpus().len();
    let frequency = sys.global_cpu_info().frequency();

    return CpuInfo {
        usage_percent: usage,
        num_cpus: cpus,
        frequency_mhz: frequency,
    };
}

//Process infos

struct ProcessInfo {
    pid: Pid,
    name: String,
    cpu_usage: f32,
    memory_kb: u64,
}

fn processinfo(sys: &System, pid: Pid) -> Option<ProcessInfo> {
    if let Some(process) = sys.process(pid) {

        return Some(ProcessInfo {
            pid,
            name: process.name().to_string(),
            cpu_usage: process.cpu_usage(),
            memory_kb: process.memory(),
        });
    }
    None
}

fn main(){
    let mut sys = System::new_all();

    sys.refresh_all();

    //RAM

    raminfo(&sys);

    let ram = RamInfo {
        total_gb: raminfo(&sys).total_gb,
        used_gb: raminfo(&sys).used_gb,
        total_swap_gb: raminfo(&sys).total_swap_gb,
        used_swap_gb: raminfo(&sys).used_swap_gb,
    };

    println!("Total RAM: {} GB", ram.total_gb);
    println!("Used RAM: {} GB", ram.used_gb);
    println!("Total Swap: {} GB", ram.total_swap_gb);
    println!("Used Swap: {} GB", ram.used_swap_gb);

    //CPU

    let cpu = CpuInfo {
        usage_percent: cpuinfo(&sys).usage_percent,
        num_cpus: cpuinfo(&sys).num_cpus,
        frequency_mhz: cpuinfo(&sys).frequency_mhz,
    };

    println!("CPU Usage: {:.2}%", cpu.usage_percent);
    println!("Number of CPUs: {}", cpu.num_cpus);
    println!("CPU Frequency: {} MHz", cpu.frequency_mhz);

    //disks

    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
    println!("{disk:?}");
    }

    //Processes

    println!("Number of processes: {}", sys.processes().len());

    let pids: Vec<_> = sys.processes().keys().copied().collect();
    
    for pid in &pids {
        sys.refresh_process(*pid);
    }
    
    // Display process information
    for pid in &pids {
        sys.refresh_process(*pid);
        
        if let Some(process) = sys.process(*pid) {
            
            println!("PID:{}\tName:{} :\n CPU:{}%\tRAM:{} KB", 
                     pid, process.name(), process.cpu_usage(), process.memory());
        }
        
    }
    process::exit(0);
}
