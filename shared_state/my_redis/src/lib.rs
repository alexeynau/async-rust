use std::sync::Mutex;

struct CanIncrement {
    mutex: Mutex<i32>,
}
impl CanIncrement {
    // This function is not marked async.
    fn increment(&self) {
        let mut lock = self.mutex.lock().unwrap();
        *lock += 1;
    }
}
// hide mutex lock in wrapper-structure
async fn increment_and_do_stuff(can_incr: &CanIncrement) {
    can_incr.increment();
    do_something_async().await;
}

async fn do_something_async(){}

// This compiles!
// (but restructuring the code would be better in this case)
async fn increment_and_do_stuff_2(mutex: &tokio::sync::Mutex<i32>) { // note! This uses the Tokio mutex
    let mut lock = mutex.lock().await;
    *lock += 1;

    do_something_async().await;
} // lock goes out of scope here

// This works!
async fn increment_and_do_stuff_3(mutex: &Mutex<i32>) {
    {
        let mut lock = mutex.lock().unwrap();
        *lock += 1;
    } // lock goes out of scope here

    do_something_async().await;
}