//
// Task & WaitQueue.
//

use alloc::boxed::Box;
use alloc::string::ToString;
use axtask::{AxTaskRef, WaitQueue};
use core::sync::atomic::{AtomicU32, Ordering};
use core::time::Duration;

static WQ: WaitQueue = WaitQueue::new();

#[no_mangle]
pub fn sys_futex_wait(futex: &AtomicU32, expected: u32, timeout: Option<Duration>) -> bool {
    let condition = || {
        futex
            .compare_exchange(expected, expected, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
    };

    match timeout {
        #[allow(unused_variables)]
        Some(duration) => {
            #[cfg(not(feature = "irq"))]
            panic!("Need to enable 'irq' feature.");
            #[cfg(feature = "irq")]
            !WQ.wait_timeout_until(duration, condition)
        }
        None => {
            WQ.wait_until(condition);
            true
        }
    }
}

#[no_mangle]
pub fn sys_futex_wake(_futex: &AtomicU32, count: i32) {
    if count == i32::MAX {
        WQ.notify_all(false);
    } else {
        for _ in 0..count {
            WQ.notify_one(false);
        }
    }
}

#[no_mangle]
pub fn sys_spawn2(
    func: Box<dyn FnOnce()>,
    _prio: i32,
    stack_size: usize,
    _core_id: isize,
) -> usize {
    let func = Box::into_raw(Box::new(func)).expose_addr();
    let main = move || unsafe {
        Box::from_raw(core::ptr::from_exposed_addr::<Box<dyn FnOnce()>>(func).cast_mut())();
    };
    let ret = axtask::spawn_raw(main, "".to_string(), stack_size);
    let ptr = Box::leak(Box::new(ret));
    ptr as *mut _ as usize
}

#[no_mangle]
fn sys_join(handle: usize) {
    let t = handle as *mut axtask::AxTaskRef;
    unsafe {
        t.as_mut().unwrap().join();
    }
}

#[no_mangle]
pub fn sys_yield_now() {
    axtask::yield_now();
}

#[no_mangle]
pub fn sys_set_priority(nice: isize) {
    axtask::set_priority(nice);
}

#[no_mangle]
pub fn sys_sleep(dur: Duration) {
    axtask::sleep(dur);
}

#[no_mangle]
pub fn sys_close_thread(handle: usize) {
    unsafe { core::ptr::drop_in_place(handle as *mut AxTaskRef) }
}