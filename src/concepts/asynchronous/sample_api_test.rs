#[cfg(test)]
mod sample_api_test {
    use crate::concepts::asynchronous::sample_api::{get_page, get_page_random_failed};
    use crate::models::error::Error;
    use futures::future;

    #[tokio::test]
    async fn test_get_page() {
        let page = get_page(42).await;
        println!("Page #42: {:?}", page);
    }

    #[tokio::test]
    async fn test_get_page_random_failed() {
        let pages = 1..20;

        let result = pages
            .into_iter()
            .map(|page| async move { get_page_random_failed(page.clone()).await });

        let resp: Result<Vec<Vec<usize>>, Error> = future::try_join_all(result)
            .await
            .map_err(|e| Error::from("future error"));

        println!("Page #42: {:?}", resp);
    }
}
