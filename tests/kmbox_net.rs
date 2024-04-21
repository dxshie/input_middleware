use ::serial_test::{parallel, serial};
// TODO:
// set the kmbox in monitor mode to assert the mouse move and other actions that can be asserted
#[cfg(test)]
#[parallel]
mod parallel_tests {
    use input_middleware::button_state::{ButtonState, MwheelState};
    use input_middleware::devices::kmbox_net::{KMBoxNet, KMBoxNetConfig};
    use input_middleware::keyboardkeys::KeyboardKey;
    use input_middleware::{InputDevice, InputMiddleware};

    /// The UUID of the KMBox Net device. Set your environment variable to the UUID of your KMBox Net device before running the tests.
    /// PS: $Env:KMBOX_UUID = "XXXXXXX"
    /// LINUX: exports KMBOX_UUID="XXXXXXX"
    const UUID: &'static str = env!("KMBOX_UUID");

    #[test]
    fn connect_fail_wrong_uuid() {
        let km = KMBoxNet::new(KMBoxNetConfig::default_with_uuid("XXXXXXXX"));
        if let Err(e) = km {
            assert!(e.to_string().contains("10060"))
        }
    }

    #[test]
    fn monitor() {
        simple_logger::init_with_level(log::Level::Debug).unwrap();
        if let Ok(km) = KMBoxNet::new(KMBoxNetConfig::default_with_uuid(UUID)) {
            if let Ok(mut km_monitor) = km.into_monitor() {
                if let Ok(_) = km_monitor.bind() {
                    let time_to_stop =
                        std::time::Instant::now() + std::time::Duration::from_secs(10);
                    while std::time::Instant::now() < time_to_stop {
                        if let Ok(data) = km_monitor.recv_monitor_data() {
                            println!("{:?}", data);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn move_the_mouse_1px() {
        let km = KMBoxNet::new(KMBoxNetConfig::default_with_uuid(UUID));
        match km {
            Ok(mut km) => {
                km.mouse_move([1, 1]).unwrap();
            }
            Err(_) => println!("Failed to connect to KMBox Net"),
        }
    }

    #[test]
    fn mouse_left_click() {
        let km = KMBoxNet::new(KMBoxNetConfig::default_with_uuid(UUID));
        match km {
            Ok(mut km) => {
                km.mouse_left_click(ButtonState::Pressed).unwrap();
                km.mouse_left_click(ButtonState::Released).unwrap();
            }
            Err(_) => println!("Failed to connect to KMBox Net"),
        }
    }

    #[test]
    fn mouse_right_click() {
        let km = KMBoxNet::new(KMBoxNetConfig::default_with_uuid(UUID));
        match km {
            Ok(mut km) => {
                km.mouse_right_click(ButtonState::Pressed).unwrap();
                km.mouse_right_click(ButtonState::Released).unwrap();
            }
            Err(_) => println!("Failed to connect to KMBox Net"),
        }
    }

    #[test]
    fn mouse_wheel() {
        let km = KMBoxNet::new(KMBoxNetConfig::default_with_uuid(UUID));
        match km {
            Ok(mut km) => {
                km.mouse_wheel(MwheelState::Up(5)).unwrap();
                km.mouse_wheel(MwheelState::Down(10)).unwrap();
            }
            Err(_) => println!("Failed to connect to KMBox Net"),
        }
    }

    #[test]
    fn keyboard_keyevent() {
        let km = KMBoxNet::new(KMBoxNetConfig::default_with_uuid(UUID));
        match km {
            Ok(mut km) => {
                km.keyboard_keydown(KeyboardKey::KEY_A).unwrap();
                km.keyboard_keyup(KeyboardKey::KEY_A).unwrap();
            }
            Err(_) => println!("Failed to connect to KMBox Net"),
        }
    }

    #[test]
    fn move_the_mouse_50px_trait_abstr() {
        let config = KMBoxNetConfig::default_with_uuid(UUID);
        let input_device = InputMiddleware::new(InputDevice::KMBoxNet(config));
        match input_device {
            Ok(mut input_device) => {
                input_device.mouse_move([50, 50]).expect("mouse to move");
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}

#[cfg(test)]
#[serial]
mod serial_test {
    use input_middleware::devices::kmbox_net::{KMBoxNet, KMBoxNetConfig};
    const UUID: &'static str = env!("KMBOX_UUID");
    #[test]
    fn reboot() {
        let km = KMBoxNet::new(KMBoxNetConfig::default_with_uuid(UUID));
        match km {
            Ok(mut km) => {
                km.reboot().unwrap();
            }
            Err(_) => println!("Failed to connect to KMBox Net"),
        }
    }
}
