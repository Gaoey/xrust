use crate::models::error::Error;
use lazy_static::lazy_static;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::time::Duration;
use tokio::time::{sleep, Instant};

lazy_static! {
    static ref START_TIME: Instant = Instant::now();
}

// following this instruction
// https://gendignoux.com/blog/2021/04/01/rust-async-streams-futures-part1.html?fbclid=IwAR25rfqfygdEY0t8fNdDR2d2x4q5_oUqtRfSr4kUdrt_URTIHf5E9JyTK4k
pub async fn get_page(i: usize) -> Vec<usize> {
    let millis = Uniform::from(0..10).sample(&mut rand::thread_rng());
    println!(
        "[{}] # get_page({}) will complete in {} ms",
        START_TIME.elapsed().as_millis(),
        i,
        millis
    );

    sleep(Duration::from_millis(millis)).await;
    println!(
        "[{}] # get_page({}) completed",
        START_TIME.elapsed().as_millis(),
        i
    );

    (10 * i..10 * (i + 1)).collect()
}

pub async fn get_page_random_failed(i: usize) -> Result<Vec<usize>, Error> {
    let mut rng = rand::thread_rng();
    let millis = Uniform::from(0..10).sample(&mut rng);
    // println!(
    //     "[{}] # get_page({}) will complete in {} ms",
    //     START_TIME.elapsed().as_millis(),
    //     i,
    //     millis
    // );

    sleep(Duration::from_millis(millis)).await;
    let is_failed = rng.gen::<bool>();
    if is_failed {
        println!(
            "[{}] # get_page({}) failed",
            START_TIME.elapsed().as_millis(),
            i
        );
        return Err(Error::from("get page failed"));
    }

    // println!(
    //     "[{}] # get_page({}) completed",
    //     START_TIME.elapsed().as_millis(),
    //     i
    // );

    let result = vec![i];
    Ok(result)
}
