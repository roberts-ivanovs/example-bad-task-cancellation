use tokio::task::{JoinHandle, JoinSet};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let mut set = JoinSet::new();
    let h1 = run1();
    let h2 = run2();
    let abort_handles = [h1.abort_handle(), h2.abort_handle()];
    set.spawn(h1);
    set.spawn(h2);

    while let Some(task_return) = set.join_next().await {
        match task_return {
            Ok(res) => match res {
                Ok(succsess) => {
                    println!("success: {:?}", succsess);
                }
                Err(err) => {
                    println!("error: {:?}", err);
                }
            },
            Err(res) => {
                println!("set-level join error: {:?}", res);
            }
        }
        set.abort_all();
    }

    for (idx, handle) in abort_handles.iter().enumerate() {
        println!("aborting task {idx:}");
        handle.abort();
    }

    // As you can see the run1() task just keeps on printing without an intention of stopping 😲
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
}

fn run1() -> JoinHandle<()> {
    tokio::spawn(async {
        // log timestamp every 1 sec
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            println!("timestamp: {}", chrono::Utc::now());
        }
    })
}
fn run2() -> JoinHandle<()> {
    tokio::spawn(async {
        // sleep for 3 sec then panic
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        panic!("OMG")
    })
}
