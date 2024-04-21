# Input Middlware

## Synopsis
Input Middlware is a crate that implements various hardware devices that act as a middleman between input devices and the computer.
Each device may have it's own way of connecting and communicating which is abstracted away in this crate.

```toml
input_middleware = { git = "https://github.com/dxshie/input_middleware" }
```

# Supported Devices

- ✅ KMBox_Net (SOCKET)
- ❌ KMBox_B (COM) (i don't own that one feel free to do a PR)
- ... open a issue if you want more support or create a PR

# Example

## InputMiddleware dyn trait Abstraction

```rust
use input_middleware::devices::kmbox_net::KMBoxNetConfig;
use input_middleware::{InputDevice, InputMiddleware};

fn main() {
    let uuid = env!("KMBOX_UUID");
    let config = KMBoxNetConfig::default_with_uuid(uuid);
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
```

## Direct device usage

```rust
use input_middleware::devices::kmbox_net::{KMBoxNet, KMBoxNetConfig};

fn main() {
    let uuid = env!("KMBOX_UUID");
    let config = KMBoxNetConfig::default_with_uuid(uuid);
    let km = KMBoxNet::new(config);
    match km {
        Ok(mut km) => {
            km.mouse_move([1, 1]);
        }
        Err(_) => println!("Failed to connect to KMBox Net"),
    }
}
```
