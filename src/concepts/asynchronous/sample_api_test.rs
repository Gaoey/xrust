#[cfg(test)]
mod sample_api_test {
    use crate::concepts::asynchronous::sample_api::{get_page, get_page_random_failed};

    #[tokio::test]
    async fn test_get_page() {
        let page = get_page(42).await;
        println!("Page #42: {:?}", page);
    }

    #[tokio::test]
    async fn test_get_page_random_failed() {
        let pages = 1..20;

        for page in pages {
            let result = get_page_random_failed(page.clone()).await;
            match result {
                Ok(_) => println!("page {} pass", page),
                Err(_) => println!("page {} failed", page),
            }
        }
    }
}
