use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
    {
        // returns a LockResult of MutexGuard
        // this smart pointer returns the lock when dropped
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
