use tokio::time::{sleep,Duration};
use tokio::join;

async fn dowork(secs:u64)
{
    println!("working for {}",secs);
    sleep(Duration::from_secs(secs)).await;
    println!("worked for {}",secs);
}

#[tokio::main]
async fn main()
{
    let task1=tokio::spawn(async {
        dowork(3).await;
    });

    let task2=tokio::spawn(async {
        dowork(2).await;
    });

    join!(task1);
    join!(task2);

    println!("hey");
}