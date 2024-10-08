use ::serial_test::parallel;

#[cfg(test)]
#[parallel]
#[cfg(feature = "qmp")]
mod parallel_tests {
    use input_middleware::{
        devices::qmp::{QMPConfig, QMPConnection},
        InputMiddlewareDeviceAction,
    };
    use log::debug;
    use tokio::io::AsyncReadExt;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn qmp_send_mouse_move() {
        simple_logger::init_with_level(log::Level::Debug).unwrap();

        // Create a TCP listener on localhost:8080
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

        // Spawn a task to handle incoming connections
        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = [0; 1024];
            let n = socket.read(&mut buffer).await.unwrap();

            // Assert the received message (you can modify this as per your protocol)
            assert!(n > 0);
            let received_message = String::from_utf8_lossy(&buffer[..n]);
            debug!("Received message: {}", received_message);
            assert!(received_message.contains("1"));
        });

        // Now create a QMPConnection to send a mouse move
        let mut qmp = QMPConnection::new(QMPConfig {
            host: "localhost".to_string(),
            port: 8080,
        })
        .unwrap();

        // Send a mouse move command
        qmp.mouse_move([1, 1]).unwrap();

        // Sleep for a short duration to ensure the listener has time to process
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}
