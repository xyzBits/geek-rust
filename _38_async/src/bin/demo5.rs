use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;

/// 在异步代码中使用 Mutex
///     大部分时候，标准库的 Mutex 可以在异步代码中使用，而且，这是推荐的用法
///
///     然而，标准库的 MutexGuard 不能安全的跨越 await，所以，
///     当我们需要获得锁之后执行异步操作，必须使用 tokio 自带的 Mutex


struct DB;

impl DB {
    async fn commit(&mut self) -> anyhow::Result<usize> {
        Ok(42)
    }
}

/// 多个 tokio task 中使用 这个 DB，需要使用 Arc<Mutex<DB>> 然而，db1.lock() 拿到锁后，
/// 需要运行 db.commit().await 这是一个异步操作
///
///
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db1 = Arc::new(Mutex::new(DB));
    let db2 = Arc::clone(&db1);

    tokio::spawn(async move {
        let mut db = db1.lock().await;
        // 因为拿到的 MutexGuard 要跨越 await，所以不能用 std::sync::Mutex
        // 只能用 tokio::sync::Mutex
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