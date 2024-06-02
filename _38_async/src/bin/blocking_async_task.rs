use std::time::Duration;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tokio::spawn(async {
       eprintln!("task 1");
        // tokio::time::sleep(Duration::from_millis(1)).await;
        loop {

        }
    });

    tokio::spawn(async  {
        eprintln!("task 2");

    });

    tokio::time::sleep(Duration::from_millis(1)).await;
    Ok(())

}