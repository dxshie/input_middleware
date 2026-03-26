use ::serial_test::parallel;

pub fn connection_fail_assert(e: std::io::Error) {
    println!("Failed to connect to KMBox Net: {e}");
    assert!(false)
}

pub fn assert_send_sync<T: Send + Sync>(_: &T) {
    println!("Type is Send and Sync");
}

// TODO:
// set the kmbox in monitor mode to assert the mouse move and other actions that can be asserted
#[cfg(test)]
#[parallel]
#[cfg(feature = "kmbox_net")]
mod parallel_tests {
    use input_middleware::devices::kmbox_net::KMBoxNetConfig;
    use input_middleware::{InputDevice, InputMiddleware};
    use tokio::runtime::Runtime;

    use crate::{assert_send_sync, connection_fail_assert};

    /// The UUID of the KMBox Net device. Set your environment variable to the UUID of your KMBox Net device before running the tests.
    /// PS: $Env:KMBOX_UUID = "XXXXXXX"
    /// LINUX: exports KMBOX_UUID="XXXXXXX"
    const UUID: &'static str = env!("KMBOX_UUID");

    #[test]
    fn move_the_mouse_50px_trait_abstr() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let config = KMBoxNetConfig::default_with_uuid(UUID);
            let input_device = InputMiddleware::new(InputDevice::KMBoxNet(config));
            match input_device {
                Ok(mut input_device) => {
                    // Check for Send and Sync traits
                    assert_send_sync(&input_device);

                    input_device.mouse_move([50, 50]).expect("mouse to move");
                }
                Err(e) => connection_fail_assert(e.0),
            }
        });
    }
}
