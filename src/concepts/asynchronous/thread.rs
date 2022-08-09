use chrono::Local;
use std::thread;
use std::time::Duration;

pub fn simple_thread() {
    let handle = thread::spawn(|| {
        for i in 1..100 {
            println!("start: {:?}", Local::now());
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(100));
            println!("end: {:?}", Local::now());
            println!("==========================")
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
