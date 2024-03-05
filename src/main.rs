use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut set = JoinSet::new();
    set.spawn(run1());
    set.spawn(run2());

    while let Some(task_return) = set.join_next().await {
        println!("ERROR - task panicked! {task_return:?}");
        set.abort_all();
    }

    // As you can see the run1() task just keeps on printing without an intention of stopping 😲
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
}

async fn run1() {
    match tokio::spawn(async {
        // log timestamp every 1 sec
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            println!("timestamp: {}", chrono::Utc::now());
        }
    })
    .await
    {
        Ok(succsess) => {
            println!("success: {:?}", succsess);
        }
        Err(err) => {
            println!("error: {:?}", err);
        }
    }
}
async fn run2() {
    match tokio::spawn(async {
        // sleep for 3 sec then panic
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        panic!("OMG")
    })
    .await
    {
        Ok(succsess) => {
            println!("success: {:?}", succsess);
        }
        Err(err) => {
            println!("error: {:?}", err);
        }
    }
}
