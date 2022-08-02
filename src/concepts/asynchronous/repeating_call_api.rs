use crate::concepts::asynchronous::sample_api::get_page_random_failed;
use crate::models::error::Error;
use futures::StreamExt;

async fn call_api_by_vec(pages: Vec<i32>) -> Result<Vec<usize>, Error> {
    let mut temp: Vec<usize> = vec![];
    for page in pages {
        let mut result = get_page_random_failed(page.clone() as usize)
            .await
            .map_err(|_| {
                let msg = format!("page {} error", page);
                Error::from(msg.as_str())
            })?;

        temp.append(&mut result)
    }

    Ok(temp)
}

pub async fn repeating_call_api() {
    let pages: Vec<i32> = (1..100).collect();
    let handler = pages
        .chunks(10)
        .into_iter()
        .map(|v| async move { call_api_by_vec(Vec::from(v)).await });

    let stream = futures::stream::iter(handler).buffer_unordered(2);
    let results: Vec<Result<Vec<usize>, Error>> = stream.collect().await;

    println!("{:#?}", results)
}
