//! Clavier 16 touches hexadécimal
//! Mapping: 1234 QWER ASDF ZXCV

pub struct Input {
    keys: [bool; 16],
    last_key_pressed: Option<u8>,
    waiting_for_key: bool,
}

impl Input {
    pub fn new() -> Self {
        Input {
            keys: [false; 16],
            last_key_pressed: None,
            waiting_for_key: false,
        }
    }
    
    pub fn clear(&mut self) {
        self.keys = [false; 16];
        self.last_key_pressed = None;
        self.waiting_for_key = false;
    }
    
    pub fn key_down(&mut self, key: u8) {
        if key < 16 {
            self.keys[key as usize] = true;
            self.last_key_pressed = Some(key);
        }
    }
    
    pub fn key_up(&mut self, key: u8) {
        if key < 16 {
            self.keys[key as usize] = false;
        }
    }
    
    pub fn is_key_pressed(&self, key: u8) -> bool {
        if key < 16 {
            self.keys[key as usize]
        } else {
            false
        }
    }
    
    pub fn get_key_pressed(&mut self) -> Option<u8> {
        let key = self.last_key_pressed;
        self.last_key_pressed = None;
        key
    }
    
    pub fn wait_for_key(&mut self) {
        self.waiting_for_key = true;
    }
    
    pub fn is_waiting_for_key(&self) -> bool {
        self.waiting_for_key
    }
    
    pub fn stop_waiting(&mut self) {
        self.waiting_for_key = false;
    }
    
    pub fn get_debug_state(&self) -> String {
        let mut state = String::from("Keys: ");
        for i in 0..16 {
            if self.keys[i] {
                state.push_str(&format!("{:X} ", i));
            }
        }
        if let Some(last) = self.last_key_pressed {
            state.push_str(&format!("| Last: {:X}", last));
        }
        state
    }
}

impl Input {
    pub fn js_key_to_chip8(js_key: &str) -> Option<u8> {
        match js_key {
            "Digit1" | "1" => Some(0x1),
            "Digit2" | "2" => Some(0x2),
            "Digit3" | "3" => Some(0x3),
            "Digit4" | "4" => Some(0xC),
            
            "KeyQ" | "q" => Some(0x4),
            "KeyW" | "w" => Some(0x5),
            "KeyE" | "e" => Some(0x6),
            "KeyR" | "r" => Some(0xD),
            
            "KeyA" | "a" => Some(0x7),
            "KeyS" | "s" => Some(0x8),
            "KeyD" | "d" => Some(0x9),
            "KeyF" | "f" => Some(0xE),
            
            "KeyZ" | "z" => Some(0xA),
            "KeyX" | "x" => Some(0x0),
            "KeyC" | "c" => Some(0xB),
            "KeyV" | "v" => Some(0xF),
            
            _ => None,
        }
    }
    
    pub fn chip8_key_name(key: u8) -> String {
        format!("{:X}", key.min(15))
    }
}