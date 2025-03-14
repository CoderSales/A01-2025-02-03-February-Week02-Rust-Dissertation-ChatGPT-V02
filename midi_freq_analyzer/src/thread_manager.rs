use std::thread;
use std::panic;
use std::backtrace::Backtrace;


pub fn spawn_thread<F>(task: F) 
where
    F: FnOnce() + Send + 'static,
{
    thread::spawn(move || {
        let result = panic::catch_unwind(panic::AssertUnwindSafe(task));
        if let Err(err) = result {
            eprintln!("⚠️ A thread panicked! {:?}", err);
        }
    });
}
