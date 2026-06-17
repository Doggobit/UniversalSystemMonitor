use crate::run::ui_definitions::MainWindow;

pub mod run;

//main

fn main(){

    let ui = MainWindow::new().unwrap();

    run::run_app(ui);

}