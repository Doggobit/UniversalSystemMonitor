pub mod run;

pub use crate::run::ui_definitions::MainWindow;
pub use std::time::Duration;
use slint::ComponentHandle;

//Android main
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: i_slint_backend_android_activity::AndroidApp) {

    slint::android::init(app).unwrap();

    let ui = MainWindow::new().unwrap();

    let ui_weak = ui.as_weak();

    let timer = slint::Timer::default();

    timer.start(slint::TimerMode::Repeated, Duration::from_millis(1000), move || {

        let ui = ui_weak.upgrade().unwrap();
        ui.invoke_refresh_clicked();
    
    });

    run::run_app(ui);

}