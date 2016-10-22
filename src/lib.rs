#![feature(lang_items)]
#![feature(const_fn, unique)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;

#[macro_use]
mod vga_buffer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
extern fn rust_begin_panic() -> ! {
    loop {}
}

#[no_mangle]
pub extern fn kmain() -> ! {

    // ATTENTION: we have a very small stack and no guard page
    vga_buffer::clear_screen();
    println!("READY");

    loop { }
}