use tokio::task::{JoinHandle, JoinSet};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut set = JoinSet::new();
    // set.spawn already spawns a tokio::task under the hood!
    set.spawn(run1());
    set.spawn(run2());

    // On first error we shut down all tasks
    while let Some(task_return) = set.join_next().await {
        // Common task top-level logging
        match task_return {
            Ok(res) => {
                println!("success: {:?}", res);
            }
            Err(res) => {
                println!("join error: {:?}", res);
            }
        }
        set.abort_all();
    }

    // As you can see all tasks have been stopped! 😲
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
}

async fn run1() {
    // log timestamp every 1 sec
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("timestamp: {}", chrono::Utc::now());
    }
}
async fn run2() {
    // sleep for 3 sec then panic
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    panic!("OMG")
}
