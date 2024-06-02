use std::sync::Arc;
use std::time::Duration;
use anyhow::Result;
use tokio::sync::Mutex;

struct DB;

impl DB {
    async fn commit(&mut self) -> Result<usize> {
        Ok(42)
    }
}

#[tokio::main]
async  fn main() -> Result<()> {

    //std::sync::MutexGuard is not a future
    // let db1 = Arc::new(std::sync::Mutex::new(DB));
    let db1 = Arc::new(Mutex::new(DB));
    let db2 = Arc::clone(&db1);

    tokio::spawn(async move {
       let mut db = db1.lock().await;

        let affected = db.commit().await?;
        println!("db1: Total affected rows: {}", affected);

        Ok::<_, anyhow::Error>(())
    });

    tokio::spawn(async move {
        let mut db = db2.lock().await;
        let affected = db.commit().await?;

        println!("db2: Total affected rows: {}", affected);
        Ok::<_, anyhow::Error>(())

    });


    tokio::time::sleep(Duration::from_millis(1)).await;



    Ok(())
}