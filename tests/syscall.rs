use ::serial_test::parallel;

#[cfg(test)]
#[parallel]
#[cfg(feature = "syscall")]
mod parallel_tests {

    #[tokio::test]
    async fn syscall_send_mouse_move() {}
}
