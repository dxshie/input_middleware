use std::time::Duration;

use input_middleware::devices::kmbox_net::{KMBoxNet, KMBoxNetConfig};
use rand::{thread_rng, Rng};
use simple_logger::SimpleLogger;

const UUID: &'static str = env!("KMBOX_UUID");

fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();
    let km = KMBoxNet::new(KMBoxNetConfig::default_with_uuid(UUID));
    match km {
        Ok(mut km) => {
            let mut now = std::time::Instant::now();
            loop {
                let wait = thread_rng().gen_range(100..500);
                if now.elapsed().as_millis() > wait {
                    let x = thread_rng().gen_range(-2..2);
                    let y = thread_rng().gen_range(-2..2);
                    km.mouse_move([x, y]).unwrap();
                    log::info!("Mouse moved to x: {}, y: {}", x, y);
                    now = std::time::Instant::now();
                }
                std::thread::sleep(Duration::from_millis(1));
            }
        }
        Err(e) => log::error!("Failed to connect to KMBox Net: {e}"),
    }
}
