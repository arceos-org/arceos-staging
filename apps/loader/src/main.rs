#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#[cfg(feature = "axstd")]
use axstd::println;

const PLASH_START: usize = 0x22000000;
#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
let apps_start = PLASH_START as *const u8;
let apps_size = 32; // Dangerous!!! We need to get accurate size of apps.
println!("Load payload ...");
let code = unsafe { core::slice::from_raw_parts(apps_start, apps_size)
};
println!("content: {:?}: ", code);

println!("Load payload ok!");
}
