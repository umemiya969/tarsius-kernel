//! Keyboard driver - Biar bisa ngetik!
use spin::Mutex;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use lazy_static::lazy_static;
use crate::println;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
    );
    
    static ref INPUT_BUFFER: Mutex<heapless::String<256>> = Mutex::new(heapless::String::new());
}

pub fn add_scancode(scancode: u8) {
    let mut keyboard = KEYBOARD.lock();
    let mut buffer = INPUT_BUFFER.lock();
    
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    if character == '\n' {
                        // Enter pressed - process command!
                        let command = buffer.clone();
                        println!("\nYou typed: {}", command);
                        process_command(command.as_str());
                        buffer.clear();
                    } else if character == '\x08' { // Backspace
                        buffer.pop();
                        // TODO: Update screen
                    } else {
                        buffer.push(character).ok();
                        print!("{}", character);
                    }
                }
                DecodedKey::RawKey(key) => {
                    // Handle special keys
                }
            }
        }
    }
}

fn process_command(cmd: &str) {
    match cmd.trim() {
        "help" => {
            println!("\nAvailable commands:");
            println!("  help     - Show this help");
            println!("  clear    - Clear screen");
            println!("  info     - Show system info");
            println!("  reboot   - Reboot system");
            println!("  tarsius  - ???");
            _ => println!("\nUnknown command: {}", cmd),
        }
        
        "clear" => {
            // TODO: Clear screen
            println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
        }
        
        "info" => {
            println!("\n🦀 TARSIUS OS v0.0.1");
            println!("Kernel: Hybrid (Mach-inspired)");
            println!("Arch: x86_64");
            println!("Mode: 64-bit protected mode");
            println!("Interrupts: Enabled");
        }
        
        "tarsius" => {
            println!("\n   ▄▄▄▄▄   ▄▄▄▄▄   ██████  ██   ██ ██   ██ ███████ ");
            println!("  ██   ██ ██   ██ ██       ██   ██ ██   ██ ██      ");
            println!("  ██   ██ ██   ██ ██   ███ ███████ ██   ██ ███████ ");
            println!("  ██   ██ ██   ██ ██    ██ ██   ██ ██   ██      ██ ");
            println!("   ▄▄▄▄▄   ▄▄▄▄▄   ██████  ██   ██  █████  ███████ ");
        }
        
        "reboot" => {
            println!("Rebooting...");
            x86_64::instructions::interrupts::disable();
            unsafe {
                let mut port = x86_64::instructions::port::Port::<u8>::new(0x64);
                port.write(0xFE);
            }
        }
        
        _ => {}
    }
}