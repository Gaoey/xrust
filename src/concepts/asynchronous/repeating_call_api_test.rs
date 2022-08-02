#[cfg(test)]
mod repeating_call_api_test {
    use crate::concepts::asynchronous::repeating_call_api::repeating_call_api;

    #[tokio::test]
    async fn test_repeating_call_api() {
        repeating_call_api().await;
    }
}
