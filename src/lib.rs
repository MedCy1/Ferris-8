//! Émulateur Chip-8 en Rust vers WebAssembly

use wasm_bindgen::prelude::*;
use console_error_panic_hook;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod cpu;
mod memory;
mod display;
mod input;
mod audio;
pub use cpu::Cpu;
pub use memory::Memory;
pub use display::Display;
pub use input::Input;
pub use audio::Audio;

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"Ferris-8 émulateur initialisé".into());
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello {}, bienvenue dans Ferris-8!", name)
}

#[wasm_bindgen]
pub struct Emulator {
    cpu: Cpu,
    running: bool,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Emulator {
        web_sys::console::log_1(&"Création d'un nouvel émulateur".into());
        
        Emulator {
            cpu: Cpu::new(),
            running: false,
        }
    }
    
    #[wasm_bindgen]
    pub fn load_rom(&mut self, rom_data: &[u8]) -> bool {
        self.cpu.load_rom(rom_data);
        true
    }
    
    #[wasm_bindgen]
    pub fn cycle(&mut self) {
        if self.running {
            self.cpu.cycle();
        }
    }
    
    #[wasm_bindgen]
    pub fn start(&mut self) {
        self.running = true;
    }
    
    #[wasm_bindgen]
    pub fn stop(&mut self) {
        self.running = false;
    }
    
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.running = false;
    }
    
    #[wasm_bindgen]
    pub fn get_display_buffer(&self) -> js_sys::Uint8Array {
        let buffer = self.cpu.get_display_buffer();
        js_sys::Uint8Array::from(&buffer[..])
    }
    #[wasm_bindgen]
    pub fn key_down(&mut self, key: u8) {
        self.cpu.key_down(key);
    }
    
    #[wasm_bindgen]
    pub fn key_up(&mut self, key: u8) {
        self.cpu.key_up(key);
    }
    
    #[wasm_bindgen]
    pub fn get_debug_info(&self) -> String {
        self.cpu.get_debug_info()
    }
    
    #[wasm_bindgen]
    pub fn is_running(&self) -> bool {
        self.running && self.cpu.is_running()
    }
    
    #[wasm_bindgen]
    pub fn get_stats(&self) -> String {
        self.cpu.get_stats()
    }
    
    #[wasm_bindgen]
    pub fn memory_dump(&self, start: u16, length: u16) -> String {
        self.cpu.memory_dump(start, length)
    }
}