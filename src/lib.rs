#![feature(lang_items)]
#![feature(const_fn, unique)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;

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
extern fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}

#[no_mangle]
pub extern fn kmain(multiboot_information_address: usize) -> ! {

    // ATTENTION: we have a very small stack and no guard page
    vga_buffer::clear_screen();   // This doesn't work for some reason?
    println!("Starting...");

    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    println!("Memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
    }

    println!("Ready");

    panic!();
    loop { }
}