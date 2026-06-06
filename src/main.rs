use sysinfo::{Components, Disks, Networks, System, Pid};
use std::{env::args, process::{self, exit}};

//conversions
fn kb_to_gb(kb: u64) -> u64 {
    return (kb) / (1024 * 1024);
}

fn b_to_gb(b: u64) -> u64 {
    return (b) / (1024 * 1024 * 1024);
}

//components infos

struct ComponentInfo{
    label: String,
    temperature: f32,
}

fn componentinfo() -> Vec<ComponentInfo> {

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

//disks infos

struct DiskInfo {
    name: String,
    total_space_gb: u64,
    available_space_gb: u64,
}

fn diskinfo() -> Vec<DiskInfo> {
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

struct NetworkInfo {
    interface_name: String,
    received_bytes: u64,
    transmitted_bytes: u64,
}

fn networkinfo() -> Vec<NetworkInfo> {
    let networks = Networks::new_with_refreshed_list();

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

struct ProcessInfo {
    pid: Pid,
    name: String,
    cpu_usage: f32,
    memory: u64,
}

fn processinfo(sys: &mut System) -> Vec<ProcessInfo> {
    let pids: Vec<_> = sys.processes().keys().copied().collect();
    
    for pid in &pids {
        sys.refresh_process(*pid);
    }

    let processes: Vec<ProcessInfo> = pids.iter()
        .filter_map(|pid| {
            sys.process(*pid).map(|process| ProcessInfo {
                pid: *pid,
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory: process.memory(),
            })
        })
        .collect();
    
    return processes;
}

//search process by name

fn search_process(name: String) -> ProcessInfo {
    let mut sys = System::new_all();
    let target = processinfo(&mut sys).into_iter().find(|p| p.name == name).expect("PROCESS NOT FOUND!\n");

    return target;

}

//look for a process by a keyword

fn lookfor_process(word: String) -> Vec<ProcessInfo> {
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

//main

fn main(){

    //system refresh

    let mut sys = System::new_all();

    sys.refresh_all();

    //args

    let args: Vec<String> = args().collect();

    if args.len() > 1 {

        if args[1] == "-s"{
            let process = search_process(args[2].clone());
            println!("PROCESS FOUND!");
            println!("PID: {}, name: {}, cpu_usage: {} %, ram_usage: {} bytes", process.pid, process.name, process.cpu_usage, process.memory);
            exit(0);
        }

        else if args[1] == "-lf" {
            let process_list = lookfor_process(args[2].clone());
            if process_list.is_empty() {
                println!("NO PROCESS FOUND WITH THIS WORD: {}", args[2]);
                exit(1);
            }
            for process in process_list{
                println!("PID: {}", process.pid);
                println!("CPU Usage: {:.2}%", process.cpu_usage);
                println!("Name: {}", process.name);
                println!("Memory: {} KB", process.memory);
                println!("*****************************");
            }
            exit(0);
        }

        
    }



    //components

    println!("Components!!!!");

    let components = componentinfo();

    if components.is_empty() {
        println!("Cannot access data, are running as an administrator?");
    }

    else{
        for component in components {
            println!("{}", component.label);
            println!("Temperature: {} °C", component.temperature);
            println!("-----------------------------");
        }
    }

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
    println!("-----------------------------");

    //CPU

    let cpu = CpuInfo {
        usage_percent: cpuinfo(&sys).usage_percent,
        num_cpus: cpuinfo(&sys).num_cpus,
        frequency_mhz: cpuinfo(&sys).frequency_mhz,
    };

    println!("CPU Usage: {:.2}%", cpu.usage_percent);
    println!("Number of CPUs: {}", cpu.num_cpus);
    println!("CPU Frequency: {} MHz", cpu.frequency_mhz);
    println!("-----------------------------");

    //disks

    let disks = diskinfo();
    for disk in disks {
        println!("Disk Name: {}", disk.name);
        println!("Total Space: {} GB", disk.total_space_gb);
        println!("Available Space: {} GB", disk.available_space_gb);
        println!("-----------------------------");
    }

    //networks
    let networks = networkinfo();
    for net in networks {
        println!("Interface: {}", net.interface_name);
        println!("Received Bytes: {} bytes", net.received_bytes);
        println!("Transmitted Bytes: {} bytes", net.transmitted_bytes);
        println!("-----------------------------");
    }

    //Processes

    println!("Number of processes: {}", sys.processes().len());

    let processes = processinfo(&mut sys);

    for process in processes {
        println!("PID: {}", process.pid);
        println!("Name: {}", process.name);
        println!("CPU Usage: {:.2}%", process.cpu_usage);
        println!("Memory: {} KB", process.memory);
        println!("*****************************");
    }

    process::exit(0);
}
