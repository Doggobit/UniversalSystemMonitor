use slint::{ComponentHandle};

use crate::run::ui_definitions::MainWindow;
use std::time::Duration;

pub mod run;

//main

fn main(){

    let ui = MainWindow::new().unwrap();

    let ui_weak = ui.as_weak();

    let timer = slint::Timer::default();

    timer.start(slint::TimerMode::Repeated, Duration::from_millis(2500), move || {

        let ui = ui_weak.upgrade().unwrap();
        ui.invoke_refresh_clicked();
        let text = ui.get_search_text();
        ui.invoke_search_requested(text);
    });

    run::run_app(ui);

}