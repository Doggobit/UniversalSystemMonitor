use slint;
use sysinfo::{Components, Disks, MINIMUM_CPU_UPDATE_INTERVAL, Networks, Pid, System};
use std::{env::args, process::exit, thread, {cell::RefCell}, rc::Rc};

slint::include_modules!();

//conversions
fn kb_to_gb(kb: u64) -> u64 {
    return (kb) / (1024 * 1024);
}

fn b_to_gb(b: u64) -> u64 {
    return (b) / (1024 * 1024 * 1024);
}

//Data to string

fn format_bytes(b: u64) -> String {
    if b < 1024             { format!("{} B", b) }
    else if b < 1<<20       { format!("{:.1} KB", b as f64 / 1024.0) }
    else if b < 1<<30       { format!("{:.1} MB", b as f64 / 1_048_576.0) }
    else                    { format!("{:.2} GB", b as f64 / 1_073_741_824.0) }
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
    let frequency = sys.cpus().first().map(|c| c.frequency()).unwrap_or(0);

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

struct ProcessInfo {
    pid: Pid,
    name: String,
    cpu_usage: f32,
    memory: u64,
}

fn processinfo(sys: &mut System) -> Vec<ProcessInfo> {
    sys.refresh_processes();
    thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_processes();

    let processes = sys.processes().iter().map(|(pid, process)| ProcessInfo {
        pid: *pid,
        name: process.name().to_string(),
        cpu_usage: process.cpu_usage() / sys.cpus().len() as f32,
        memory: process.memory(),
    }).collect();
    
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

    sys.refresh_cpu_usage();

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

    println!("WHEN PROGRAM OPENED");

    //UI

    let ui = MainWindow::new().unwrap();

    //Refresh everything

    sys.refresh_all();
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_all();

    // RAM
    let mut ram = raminfo(&sys);
    ui.set_total_ram_gb(ram.total_gb as i32);
    ui.set_used_ram_gb(ram.used_gb as i32);
    ui.set_total_swap_gb(ram.total_swap_gb as i32);
    ui.set_used_swap_gb(ram.used_swap_gb as i32);

    // CPU
    let mut cpu = cpuinfo(&sys);
    ui.set_cpu_usage(cpu.usage_percent);
    ui.set_num_cpus(cpu.num_cpus as i32);
    ui.set_cpu_frequency_mhz(cpu.frequency_mhz as i32);

    // Components
    ui.set_components(Rc::new(slint::VecModel::from(
        componentinfo().into_iter().map(|c| ComponentData {
            label:c.label.into(),
            temperature:c.temperature,
        }).collect::<Vec<_>>()
    )).into());

    // Disks
    ui.set_disks(Rc::new(slint::VecModel::from(
        diskinfo().into_iter().map(|d| DiskData {
            name:d.name.into(),
            total_gb:d.total_space_gb as i32,
            available_gb:d.available_space_gb as i32,
        }).collect::<Vec<_>>()
    )).into());

    // Networks
    ui.set_networks(Rc::new(slint::VecModel::from(
        networkinfo().into_iter().map(|n| NetworkData {
            interface_name:n.interface_name.into(),
            received:format_bytes(n.received_bytes).into(),
            transmitted:format_bytes(n.transmitted_bytes).into(),
        }).collect::<Vec<_>>()
    )).into());

    // Processes
    let all_procs_shared: Rc<RefCell<Vec<ProcessData>>> = Rc::new(RefCell::new(
        processinfo(&mut sys).into_iter().map(|p| ProcessData {
            pid:       p.pid.as_u32() as i32,
            name:      p.name.into(),
            cpu_usage: p.cpu_usage,
            memory:    format_bytes(p.memory).into(),
        }).collect()
    ));

    ui.set_processes(
        Rc::new(slint::VecModel::from(all_procs_shared.borrow().clone())).into()
    );

    // Search/filter callback
    let ui_weak_search = ui.as_weak();
    ui.on_search_requested({
        let all_procs_shared = all_procs_shared.clone();
        move |term| {
            let t = term.to_string();
            let data = all_procs_shared.borrow();
            let filtered: Vec<ProcessData> = if t.is_empty() {
                data.clone()
            } else {
                data.iter()
                    .filter(|p| p.name.as_str().contains(&*t))
                    .cloned()
                    .collect()
            };
            drop(data); // release borrow before touching ui
            if let Some(ui) = ui_weak_search.upgrade() {
                ui.set_processes(Rc::new(slint::VecModel::from(filtered)).into());
            }
        }
    });

    let ui_weak = ui.as_weak();

    ui.on_refresh_clicked(move || {
        //refresh system variables
        sys.refresh_all();
        thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_all();

        let ui = ui_weak.upgrade().unwrap();

        //refresh RAM
        ram = raminfo(&sys);
        ui.set_total_ram_gb(ram.total_gb as i32);
        ui.set_used_ram_gb(ram.used_gb as i32);
        ui.set_total_swap_gb(ram.total_swap_gb as i32);
        ui.set_used_swap_gb(ram.used_swap_gb as i32);

        //refresh CPU
        cpu = cpuinfo(&sys);
        ui.set_cpu_usage(cpu.usage_percent);
        ui.set_num_cpus(cpu.num_cpus as i32);
        ui.set_cpu_frequency_mhz(cpu.frequency_mhz as i32);

        //refresh networks
        ui.set_networks(Rc::new(slint::VecModel::from(
            networkinfo().into_iter().map(|n| NetworkData {
                interface_name: n.interface_name.into(),
                received:format_bytes(n.received_bytes).into(),
                transmitted:format_bytes(n.transmitted_bytes).into(),
        }).collect::<Vec<_>>()
        )).into());

        // refresh processes
        let new_procs: Vec<ProcessData> = sys.processes().iter().map(|(pid, p)| ProcessData {
            pid:pid.as_u32() as i32,
            name:p.name().to_string().into(),
            cpu_usage:p.cpu_usage(),
            memory:format_bytes(p.memory()).into(),
        }).collect();

        *all_procs_shared.borrow_mut() = new_procs.clone();
        ui.set_processes(Rc::new(slint::VecModel::from(new_procs)).into());
    });
    ui.run().unwrap();  // blocks until window is closed
}
