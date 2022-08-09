#[cfg(test)]
mod thread_test {
    use crate::concepts::asynchronous::thread::simple_thread;

    #[tokio::test]
    async fn test_simple_thread() {
        simple_thread();
    }
}
