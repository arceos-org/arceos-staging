#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
static FLAG: AtomicUsize = AtomicUsize::new(0);

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    thread::spawn(move || {
        println!("Spawned-thread is waiting ...");
        while FLAG.load(Ordering::Relaxed) < 1 {
            // For cooperative scheduler, we must yield here!
            // For preemptive scheduler, just relaxed! Leave it for scheduler.
            // thread::yield_now();   
        }
        let _ = FLAG.fetch_add(1, Ordering::Relaxed);
    });
    
    // Give spawned thread a chance to start.
    thread::yield_now();
    println!("Main thread set FLAG to notify spawned-thread to continue.");
    let _ = FLAG.fetch_add(1, Ordering::Relaxed);
    println!("Main thread waits spawned-thread to respond ...");

    while FLAG.load(Ordering::Relaxed) < 2 {
        thread::yield_now();
    }

    println!("Preempt test run OK!");
}