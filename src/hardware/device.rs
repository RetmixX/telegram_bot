use std::thread::sleep;
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};
use crate::hardware::camera::human_from_camera;

enum ActivationFlags {
    Activate,
    Deactivate,
    Error,
}

pub fn handle_device_input(path: &str) -> bool {
    let mut activate_flag = false;
    let device = DeviceState::new();
    loop {
        let keys = device.get_keys();
        //let d = device.get_mouse();
        if !keys.is_empty() {
            match define_activate_combination(&keys) {
                ActivationFlags::Activate => {
                    activate_flag = true;
                    sleep(Duration::from_secs(2));
                    continue;
                }
                ActivationFlags::Deactivate => {
                    activate_flag = false;
                    sleep(Duration::from_secs(2));
                    continue;
                }
                ActivationFlags::Error => {
                    if activate_flag {
                        human_from_camera(path);
                        return true;
                    }
                }
            }
        }

        sleep(Duration::from_millis(100));
    }
}

fn define_activate_combination(keys_enter: &Vec<Keycode>) -> ActivationFlags {
    return if keys_enter.contains(&Keycode::LAlt) && keys_enter.contains(&Keycode::I) {
        ActivationFlags::Activate
    } else if keys_enter.contains(&Keycode::LAlt) && keys_enter.contains(&Keycode::O) {
        ActivationFlags::Deactivate
    } else {
        ActivationFlags::Error
    };
}