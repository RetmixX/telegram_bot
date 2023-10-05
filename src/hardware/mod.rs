pub mod device;
pub mod camera;

pub fn shutdown_device() {
    std::process::Command::new("shutdown")
        .args(["-h", "now"]).spawn().unwrap();
}

pub fn send_message_box(type_window: &str, message: &str) {
    std::process::Command::new("zenity")
        .args([type_window, "--text", message]).spawn().unwrap();
}

