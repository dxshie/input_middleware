use input_middleware::{
    devices::qmp::{QMPConfig, QMPConnection},
    InputMiddlewareDeviceAction,
};

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    let mut qmp = QMPConnection::new(QMPConfig {
        host: "localhost".to_string(),
        port: 8080,
    })
    .unwrap();
    qmp.mouse_move([1, 1]).unwrap();
}
