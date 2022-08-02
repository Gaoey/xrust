#[cfg(test)]
mod sample_api_test {
    use crate::concepts::asynchronous::sample_api::get_page;

    #[tokio::test]
    async fn test_get_page() {
        let page = get_page(42).await;
        println!("Page #42: {:?}", page);
    }
}
