#[cfg(test)]
mod test {
    use input_middleware::devices::kmbox_net::KMBoxNet;

    #[test]
    fn test() {
        simple_logger::init_with_level(log::Level::Debug).unwrap();
        let km = KMBoxNet::new("192.168.2.188".into(), 16824, "XXXXXXXX".into());
        match km {
            Ok(mut km) => {
                km.mouse_move([1, 1]);
            }
            Err(_) => println!("Failed to connect to KMBox Net"),
        }
    }
}
