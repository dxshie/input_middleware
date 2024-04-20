// TODO:
// set the kmbox in monitor mode to assert the mouse move and other actions that can be asserted
#[cfg(test)]
mod test {
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

    // #[test]
    // fn reboot() {
    //     let km = KMBoxNet::new(KMBoxNetConfig::default_with_uuid(UUID));
    //     match km {
    //         Ok(mut km) => {
    //             km.reboot();
    //         }
    //         Err(_) => println!("Failed to connect to KMBox Net"),
    //     }
    // }

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
