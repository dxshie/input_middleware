#[cfg(test)]
mod test {
    use input_middleware::devices::kmbox_net::{KMBoxNet, KMBoxNetConfig};
    use input_middleware::{InputDevice, InputMiddleware};

    #[test]
    fn test() {
        let km = KMBoxNet::new(KMBoxNetConfig::default_with_uuid("XXXXXXX"));
        match km {
            Ok(mut km) => {
                km.mouse_move([1, 1]).unwrap();
            }
            Err(_) => println!("Failed to connect to KMBox Net"),
        }
    }

    #[test]
    fn test_with_dyn_abstr() {
        let config = KMBoxNetConfig::default_with_uuid("XXXXXXX");
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
