pub mod run;

pub use crate::run::ui_definitions::MainWindow;


//Android main
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: i_slint_backend_android_activity::AndroidApp) {

    slint::android::init(app).unwrap();

    let ui = MainWindow::new().unwrap();

    run::run_app(ui);

}