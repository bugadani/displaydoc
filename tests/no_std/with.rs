#![cfg_attr(not(feature = "std"), allow(internal_features))]
#![cfg_attr(not(feature = "std"), feature(lang_items, start))]
#![cfg_attr(not(feature = "std"), no_std)]

#[start]
#[cfg(not(feature = "std"))]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    0
}

#[lang = "eh_personality"]
#[no_mangle]
#[cfg(not(feature = "std"))]
pub extern "C" fn rust_eh_personality() {}

#[panic_handler]
#[cfg(not(feature = "std"))]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        libc::abort();
    }
}

#[cfg(feature = "std")]
fn main() {}

use docsplay::Display;

/// this type is pretty swell
#[derive(Display)]
struct FakeType;

static_assertions::assert_impl_all!(FakeType: core::fmt::Display);
