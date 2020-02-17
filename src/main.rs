use futures::executor::{self, ThreadPool};
use futures::task::SpawnExt;
use futures_timer::Delay;
use std::boxed::Box;
use std::error::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

static N: AtomicUsize = AtomicUsize::new(1);

async fn waiter() {
    let i = N.fetch_add(1, Ordering::SeqCst);
    println!("Started waiter {}", i);
    Delay::new(Duration::from_secs(1)).await;
    println!("Finished waiter {}", i);
}

async fn quick() {
    let i = N.fetch_add(1, Ordering::SeqCst);
    println!("Running quick {}", i);
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let pool = ThreadPool::builder().pool_size(2).create()?;
    executor::block_on(async {
        let long_1 = pool.spawn_with_handle(waiter()).unwrap();
        let long_2 = pool.spawn_with_handle(waiter()).unwrap();
        let short_1 = pool.spawn_with_handle(quick()).unwrap();
        let short_2 = pool.spawn_with_handle(quick()).unwrap();
        futures::join!(long_1, long_2, short_1, short_2);
    });
    Ok(())
}
