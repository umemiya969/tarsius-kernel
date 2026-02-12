#![no_std]
#![no_main]

mod vga_buffer;
mod interrupts;
mod gdt;
mod keyboard;

use core::panic::PanicInfo;
use crate::vga_buffer::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("🦀 TARSIUS v0.0.1 - LIVE FROM QEMU!");
    println!("====================================");
    
    // Initialize CPU
    gdt::init_gdt();
    interrupts::init_idt();
    interrupts::init_pics();
    
    println!("✓ GDT initialized");
    println!("✓ IDT initialized");
    println!("✓ PIC initialized");
    println!("✓ Interrupts enabled");
    println!("\nSystem ready. Type 'help' for commands.");
    println!("> ");
    
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n💥 PANIC: {}", info);
    loop {
        x86_64::instructions::hlt();
    }
}