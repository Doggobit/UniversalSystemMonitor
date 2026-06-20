pub mod sysfuns;
pub mod ui_definitions;

use crate::{MainWindow};
use crate::run::ui_definitions::{DiskData, NetworkData, ComponentData, ProcessData};
use slint::{ComponentHandle, ModelRc};
use sysinfo::{System, Pid};
use std::{cell::RefCell, rc::Rc, thread};

#[cfg(target_os = "android")]
use android_clipboard::{self, set_text};

#[cfg(target_os = "android")]
fn copy_to_clipboard_android(str: String) -> () {
   set_text(str);
}

#[cfg(not(target_os = "android"))]
use arboard::{Clipboard};

#[cfg(not(target_os = "android"))]
fn copy_to_clipboard(str: String) -> () {
   let mut clipboard = Clipboard::new().unwrap();
   let _set = clipboard.set_text(str);
}

pub fn run_app(ui: MainWindow) {
    let mut sys = System::new_all();

    sys.refresh_all();
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_all();

    // RAM
    let ram = sysfuns::raminfo(&sys);
    ui.set_total_ram_gb(ram.total_gb as i32);
    ui.set_used_ram_gb(ram.used_gb as i32);
    ui.set_total_swap_gb(ram.total_swap_gb as i32);
    ui.set_used_swap_gb(ram.used_swap_gb as i32);

    // CPU
    let cpu = sysfuns::cpuinfo(&sys);
    ui.set_cpu_usage(cpu.usage_percent);
    ui.set_num_cpus(cpu.num_cpus as i32);
    ui.set_cpu_frequency_mhz(cpu.frequency_mhz as i32);

    // Components
    ui.set_components(Rc::new(slint::VecModel::from(
        sysfuns::componentinfo().into_iter().map(|c| ComponentData {
            label:c.label.into(),
            temperature:c.temperature,
        }).collect::<Vec<_>>()
    )).into());

    // Disks

    let disk_vec: ModelRc<DiskData> = Rc::new(slint::VecModel::from(
        sysfuns::diskinfo().into_iter().map(|d| DiskData {
            name:d.name.into(),
            total_gb:d.total_space_gb as i32,
            available_gb:d.available_space_gb as i32,
        }).collect::<Vec<_>>()
    )).into();

    ui.set_disks(disk_vec);

    // Networks
    ui.set_networks(Rc::new(slint::VecModel::from(
        sysfuns::networkinfo().into_iter().map(|n| NetworkData {
            interface_name:n.interface_name.into(),
            received:sysfuns::format_bytes(n.received_bytes).into(),
            transmitted:sysfuns::format_bytes(n.transmitted_bytes).into(),
        }).collect::<Vec<_>>()
    )).into());

    // Processes
    let all_procs_shared: Rc<RefCell<Vec<ProcessData>>> = Rc::new(RefCell::new(
        sysfuns::processinfo(&mut sys).into_iter().map(|p| ProcessData {
            pid:       p.pid.as_u32() as i32,
            name:      p.name.into(),
            cpu_usage: p.cpu_usage,
            memory:    sysfuns::format_bytes(p.memory).into(),
        }).collect()
    ));

    ui.set_processes(
        Rc::new(slint::VecModel::from(all_procs_shared.borrow().clone())).into()
    );

    //Kill process request
    ui.on_kill_requested({
            |pid| {
                let s = System::new_all();
                if let Some(process) = s.process(Pid::from_u32(pid as u32)) {
                    sysinfo::Process::kill(process);
            }
        }
    });

    //ui.on_cwd_copy_requested({
    //    |pid| {
    //        let sys = System::new_all();
    //        let cwd = sysfuns::process_cwd_string(Pid::from_u32(pid as u32), &sys).unwrap();
    //        let mut clipboard = Clipboard::new().unwrap();
    //        let _res = clipboard.set_text(cwd);
    //    }
    //});

    let ui_weak_copy = ui.as_weak();

    ui.on_cwd_copy_requested({ move
        |pid| {
            let ui_copy = ui_weak_copy.upgrade().unwrap();
            let sys = System::new_all();
            let cwd = sysfuns::process_cwd_string(Pid::from_u32(pid as u32), &sys);
            let _res = match cwd {
                Some(cwd) => {
                    let copy = cwd.clone();
                    let popup_string = String::from(cwd + " copied!");
                    ui_copy.set_copy_txt(popup_string.into());
                    #[cfg(target_os = "android")]
                    copy_to_clipboard_android(copy);
                    
                    #[cfg(not(target_os = "android"))]
                    copy_to_clipboard(copy);
                },
                None => {
                    let err_txt: String = String::from("UNABLE TO COPY CWD");
                    ui_copy.set_copy_txt(err_txt.into());
                }
            };
        }
    });

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
        let ram2 = sysfuns::raminfo(&sys);
        ui.set_total_ram_gb(ram2.total_gb as i32);
        ui.set_used_ram_gb(ram2.used_gb as i32);
        ui.set_total_swap_gb(ram2.total_swap_gb as i32);
        ui.set_used_swap_gb(ram2.used_swap_gb as i32);

        //refresh CPU
        let cpu2 = sysfuns::cpuinfo(&sys);
        ui.set_cpu_usage(cpu2.usage_percent);
        ui.set_num_cpus(cpu2.num_cpus as i32);
        ui.set_cpu_frequency_mhz(cpu2.frequency_mhz as i32);

        //refresh networks
        ui.set_networks(Rc::new(slint::VecModel::from(
            sysfuns::networkinfo().into_iter().map(|n| NetworkData {
                interface_name: n.interface_name.into(),
                received:sysfuns::format_bytes(n.received_bytes).into(),
                transmitted:sysfuns::format_bytes(n.transmitted_bytes).into(),
        }).collect::<Vec<_>>()
        )).into());

        // refresh processes
        let new_procs: Vec<ProcessData> = sys.processes().iter().map(|(pid, p)| ProcessData {
            pid:pid.as_u32() as i32,
            name:p.name().to_string().into(),
            cpu_usage:p.cpu_usage() / sysfuns::cpuinfo(&sys).num_cpus as f32,
            memory:sysfuns::format_bytes(p.memory()).into(),
        }).collect();

        *all_procs_shared.borrow_mut() = new_procs.clone();
        ui.set_processes(Rc::new(slint::VecModel::from(new_procs)).into());
    });

    ui.run().unwrap();  // blocks until window is closed 
}