#![no_std]

#[macro_use]
extern crate log;

use axerrno::{LinuxError, LinuxResult};
use memory_addr::{align_down_4k, align_up_4k, PAGE_SIZE_4K};

const TASK_SIZE: usize = 0x40_0000_0000;

pub fn kernel_execve(filename: &str) -> LinuxResult {
    error!("kernel_execve... {}", filename);

    let task = task::current();
    task.alloc_mm();

    // TODO: Move it into kernel_init().
    setup_zero_page()?;

    let sp = get_arg_page()?;
    bprm_execve(filename, 0)
}

fn setup_zero_page() -> LinuxResult {
    error!("setup_zero_page ...");
    mmap::mmap(0x0, PAGE_SIZE_4K, 0, 0, 0, 0)
}

fn get_arg_page() -> LinuxResult<usize> {
    let va = TASK_SIZE - PAGE_SIZE_4K;
    mmap::mmap(va, PAGE_SIZE_4K, 0, 0, 0, 0);
    let direct_va = mmap::faultin_page(va);
    let stack = unsafe {
        core::slice::from_raw_parts_mut(
            direct_va as *mut usize, 4
        )
    };
    stack[0] = 0;
    stack[1] = TASK_SIZE - 16;
    stack[2] = 0;
    stack[3] = 0;
    error!("get_arg_page!");
    Ok(TASK_SIZE - 32)
}

/// sys_execve() executes a new program.
fn bprm_execve(filename: &str, flags: usize) -> LinuxResult {
    unimplemented!("bprm_execve... {}", filename);
}
