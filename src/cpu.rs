//! CPU Chip-8 avec architecture classique
//! 16 registres V0-VF, registre I, PC, SP et timers

use crate::{Memory, Display, Input, Audio};

const MAX_MEMORY: u16 = 0x1000;
const PROGRAM_START: u16 = 0x200;
const MAX_STACK_SIZE: u8 = 16;

pub struct Cpu {
    pub v: [u8; 16],
    pub i: u16,
    pub pc: u16,
    pub sp: u8,
    
    pub delay_timer: u8,
    pub sound_timer: u8,
    
    pub memory: Memory,
    pub display: Display,
    pub input: Input,
    pub audio: Audio,
    
    pub stack: [u16; 16],
    
    pub draw_flag: bool,
    pub halted: bool,
    pub error_count: u32,
    pub cycle_count: u64,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu {
            v: [0; 16],
            i: 0,
            pc: PROGRAM_START,
            sp: 0,
            
            delay_timer: 0,
            sound_timer: 0,
            
            memory: Memory::new(),
            display: Display::new(),
            input: Input::new(),
            audio: Audio::new(),
            
            stack: [0; 16],
            
            draw_flag: false,
            halted: false,
            error_count: 0,
            cycle_count: 0,
        };
        
        cpu.memory.load_fontset();
        cpu
    }
    
    pub fn reset(&mut self) {
        self.v = [0; 16];
        self.i = 0;
        self.pc = PROGRAM_START;
        self.sp = 0;
        
        self.delay_timer = 0;
        self.sound_timer = 0;
        
        self.memory.clear();
        self.memory.load_fontset();
        self.display.clear();
        self.input.clear();
        
        self.stack = [0; 16];
        
        self.draw_flag = false;
        self.halted = false;
        self.error_count = 0;
        self.cycle_count = 0;
    }
    
    pub fn load_rom(&mut self, rom_data: &[u8]) {
        self.memory.load_rom(rom_data);
        
        self.pc = PROGRAM_START;
        self.halted = false;
        self.error_count = 0;
        self.cycle_count = 0;
        self.draw_flag = true;
    }
    
    pub fn cycle(&mut self) {
        if self.halted {
            return;
        }
        
        if self.error_count > 10 {
            self.halted = true;
            return;
        }
        
        self.cycle_count += 1;
        
        if !self.validate_pc() {
            return;
        }
        
        let instruction = self.fetch_instruction();
        self.execute_instruction(instruction);
        self.update_timers();
    }
    
    fn validate_pc(&mut self) -> bool {
        if self.pc >= MAX_MEMORY {
            self.pc = PROGRAM_START;
            self.error_count += 1;
            return false;
        }
        
        if self.pc < PROGRAM_START {
            self.pc = PROGRAM_START;
            self.error_count += 1;
            return false;
        }
        
        if self.pc % 2 != 0 {
            self.pc = self.pc & 0xFFFE;
            self.error_count += 1;
        }
        
        true
    }
    
    fn fetch_instruction(&mut self) -> u16 {
        if self.pc + 1 >= MAX_MEMORY {
            self.error_count += 1;
            self.halted = true;
            return 0x1200;
        }
        
        let high_byte = self.memory.read_byte(self.pc) as u16;
        let low_byte = self.memory.read_byte(self.pc + 1) as u16;
        let instruction = (high_byte << 8) | low_byte;
        
        self.pc += 2;
        instruction
    }
    
    fn execute_instruction(&mut self, instruction: u16) {
        match instruction & 0xF000 {
            0x0000 => self.execute_0xxx(instruction),
            0x1000 => self.execute_1nnn(instruction),
            0x2000 => self.execute_2nnn(instruction),
            0x3000 => self.execute_3xkk(instruction),
            0x4000 => self.execute_4xkk(instruction),
            0x5000 => self.execute_5xy0(instruction),
            0x6000 => self.execute_6xkk(instruction),
            0x7000 => self.execute_7xkk(instruction),
            0x8000 => self.execute_8xxx(instruction),
            0x9000 => self.execute_9xy0(instruction),
            0xA000 => self.execute_annn(instruction),
            0xB000 => self.execute_bnnn(instruction),
            0xC000 => self.execute_cxkk(instruction),
            0xD000 => self.execute_dxyn(instruction),
            0xE000 => self.execute_exxx(instruction),
            0xF000 => self.execute_fxxx(instruction),
            _ => {
                self.error_count += 1;
            }
        }
    }
    
    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            self.audio.play_beep();
        }
    }
    
    // Instructions Chip-8
    fn execute_0xxx(&mut self, instruction: u16) {
        match instruction {
            0x00E0 => {
                self.display.clear();
                self.draw_flag = true;
            },
            0x00EE => {
                if self.sp == 0 {
                    self.halted = true;
                    return;
                }
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
                
                if !self.is_valid_program_address(self.pc) {
                    self.halted = true;
                }
            },
            0x0000 => {
                self.halted = true;
            },
            _ => {} // SYS ignored
        }
    }
    
    fn execute_1nnn(&mut self, instruction: u16) {
        let addr = instruction & 0x0FFF;
        
        if !self.is_valid_program_address(addr) {
            self.error_count += 1;
            return;
        }
        
        self.pc = addr;
    }
    
    fn execute_2nnn(&mut self, instruction: u16) {
        let addr = instruction & 0x0FFF;
        
        if !self.is_valid_program_address(addr) {
            self.error_count += 1;
            return;
        }
        
        if self.sp >= MAX_STACK_SIZE {
            self.error_count += 1;
            return;
        }
        
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = addr;
    }
    
    fn execute_3xkk(&mut self, instruction: u16) {
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let kk = (instruction & 0x00FF) as u8;
        
        if x > 15 {
            self.error_count += 1;
            return;
        }
        
        if self.v[x] == kk {
            self.pc += 2;
        }
    }
    
    fn execute_4xkk(&mut self, instruction: u16) {
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let kk = (instruction & 0x00FF) as u8;
        
        if x > 15 {
            self.error_count += 1;
            return;
        }
        
        if self.v[x] != kk {
            self.pc += 2;
        }
    }
    
    fn execute_5xy0(&mut self, instruction: u16) {
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let y = ((instruction & 0x00F0) >> 4) as usize;
        
        if x > 15 || y > 15 {
            self.error_count += 1;
            return;
        }
        
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
    }
    
    fn execute_6xkk(&mut self, instruction: u16) {
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let kk = (instruction & 0x00FF) as u8;
        
        if x > 15 {
            self.error_count += 1;
            return;
        }
        
        self.v[x] = kk;
    }
    
    fn execute_7xkk(&mut self, instruction: u16) {
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let kk = (instruction & 0x00FF) as u8;
        
        if x > 15 {
            self.error_count += 1;
            return;
        }
        
        self.v[x] = self.v[x].wrapping_add(kk);
    }
    
    /// Instructions commençant par 0x8
    fn execute_8xxx(&mut self, instruction: u16) {
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let y = ((instruction & 0x00F0) >> 4) as usize;
        
        if x > 15 || y > 15 {
            self.error_count += 1;
            return;
        }
        
        match instruction & 0x000F {
            0x0 => self.v[x] = self.v[y], // LD Vx, Vy
            0x1 => self.v[x] |= self.v[y], // OR Vx, Vy
            0x2 => self.v[x] &= self.v[y], // AND Vx, Vy
            0x3 => self.v[x] ^= self.v[y], // XOR Vx, Vy
            0x4 => { // ADD Vx, Vy
                let sum = self.v[x] as u16 + self.v[y] as u16;
                self.v[0xF] = if sum > 255 { 1 } else { 0 }; // Carry flag
                self.v[x] = sum as u8;
            },
            0x5 => { // SUB Vx, Vy
                self.v[0xF] = if self.v[x] >= self.v[y] { 1 } else { 0 }; // Not borrow flag
                self.v[x] = self.v[x].wrapping_sub(self.v[y]);
            },
            0x6 => { // SHR Vx
                self.v[0xF] = self.v[x] & 1; // LSB
                self.v[x] >>= 1;
            },
            0x7 => { // SUBN Vx, Vy
                self.v[0xF] = if self.v[y] >= self.v[x] { 1 } else { 0 }; // Not borrow flag
                self.v[x] = self.v[y].wrapping_sub(self.v[x]);
            },
            0xE => {  // SHL Vx
                self.v[0xF] = (self.v[x] & 0x80) >> 7; // MSB
                self.v[x] <<= 1;
            },
            _ => {
                web_sys::console::log_1(&format!("Instruction 8xy{:X} inconnue", instruction & 0x000F).into());
                self.error_count += 1;
            }
        }
    }
    
    /// 9xy0 - SNE Vx, Vy : Skip si Vx != Vy
    fn execute_9xy0(&mut self, instruction: u16) {
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let y = ((instruction & 0x00F0) >> 4) as usize;
        
        if x > 15 || y > 15 {
            self.error_count += 1;
            return;
        }
        
        if self.v[x] != self.v[y] {
            self.pc += 2; // Skip next instruction
        }
    }
    
    /// Annn - LD I, addr : I = nnn
    fn execute_annn(&mut self, instruction: u16) {
        let nnn = instruction & 0x0FFF;
        
        // Permettre I de pointer vers toute la mémoire (y compris fonts)
        if nnn >= MAX_MEMORY {
            web_sys::console::log_1(&format!("I hors limites: 0x{:04X}", nnn).into());
            self.error_count += 1;
            return;
        }
        
        self.i = nnn;
    }
    
    /// Bnnn - JP V0, addr : PC = V0 + nnn
    fn execute_bnnn(&mut self, instruction: u16) {
        let nnn = instruction & 0x0FFF;
        let target = self.v[0] as u16 + nnn;
        
        if !self.is_valid_program_address(target) {
            web_sys::console::log_1(&format!("Jump V0+nnn invalide: V0={:02X} + {:03X} = {:04X}", 
                                            self.v[0], nnn, target).into());
            self.error_count += 1;
            return;
        }
        
        self.pc = target;
    }
    
    /// Cxkk - RND Vx, byte : Vx = random & kk
    fn execute_cxkk(&mut self, instruction: u16) {
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let kk = (instruction & 0x00FF) as u8;
        
        if x > 15 {
            self.error_count += 1;
            return;
        }
        
        // Générateur aléatoire simple mais amélioré
        static mut SEED: u32 = 12345;
        unsafe {
            SEED = SEED.wrapping_mul(1103515245).wrapping_add(12345);
            let random = (SEED >> 16) as u8;
            self.v[x] = random & kk;
        }
    }
    
    /// Dxyn - DRW Vx, Vy, nibble : Dessiner sprite
    fn execute_dxyn(&mut self, instruction: u16) {
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let y = ((instruction & 0x00F0) >> 4) as usize;
        let n = (instruction & 0x000F) as u8;
        
        if x > 15 || y > 15 {
            self.error_count += 1;
            return;
        }
        
        if n == 0 {
            web_sys::console::log_1(&"DRW avec hauteur 0, ignoré".into());
            return;
        }
        
        // Position du sprite
        let pos_x = self.v[x] as usize;
        let pos_y = self.v[y] as usize;
        
        // Vérifier que I + n ne dépasse pas la mémoire
        if self.i as usize + n as usize > 4096 {
            web_sys::console::log_1(&format!("DRW: I+n dépasse mémoire: I=0x{:04X}, n={}", self.i, n).into());
            self.error_count += 1;
            return;
        }
        
        // Lire les données du sprite depuis la mémoire
        let sprite_data = self.memory.read_bytes(self.i, n);
        
        // Dessiner et vérifier les collisions
        let collision = self.display.draw_sprite(pos_x, pos_y, &sprite_data);
        
        // VF = flag de collision
        self.v[0xF] = if collision { 1 } else { 0 };
        
        // Marquer pour redessiner
        self.draw_flag = true;
    }
    
    /// Instructions commençant par 0xE (clavier)
    fn execute_exxx(&mut self, instruction: u16) {
        let x = ((instruction & 0x0F00) >> 8) as usize;
        
        if x > 15 {
            self.error_count += 1;
            return;
        }
        
        let key = self.v[x];
        
        if key > 15 {
            web_sys::console::log_1(&format!("Clé invalide: 0x{:02X}", key).into());
            return;
        }
        
        match instruction & 0x00FF {
            0x9E => { // SKP Vx : Skip si touche Vx pressée
                if self.input.is_key_pressed(key) {
                    self.pc += 2;
                }
            },
            0xA1 => { // SKNP Vx : Skip si touche Vx pas pressée
                if !self.input.is_key_pressed(key) {
                    self.pc += 2;
                }
            },
            _ => {
                web_sys::console::log_1(&format!("Instruction Ex{:02X} inconnue", instruction & 0x00FF).into());
                self.error_count += 1;
            }
        }
    }
    
    /// Instructions commençant par 0xF (timers, fonts, etc.)
    fn execute_fxxx(&mut self, instruction: u16) {
        let x = ((instruction & 0x0F00) >> 8) as usize;
        
        if x > 15 {
            self.error_count += 1;
            return;
        }
        
        match instruction & 0x00FF {
            0x07 => self.v[x] = self.delay_timer, // LD Vx, DT
            0x0A => { // LD Vx, K (attendre touche)
                if let Some(key) = self.input.get_key_pressed() {
                    self.v[x] = key;
                } else {
                    self.pc -= 2; // Répéter l'instruction jusqu'à avoir une touche
                }
            },
            0x15 => self.delay_timer = self.v[x], // LD DT, Vx
            0x18 => self.sound_timer = self.v[x], // LD ST, Vx
            0x1E => { // ADD I, Vx
                let new_i = self.i.wrapping_add(self.v[x] as u16);
                if new_i >= MAX_MEMORY {
                    web_sys::console::log_1(&format!("ADD I,Vx dépasse: I=0x{:04X}+{:02X}=0x{:04X}", 
                                                    self.i, self.v[x], new_i).into());
                }
                self.i = new_i & 0x0FFF; // Maintenir dans les limites
            },
            0x29 => { // LD F, Vx
                let character = self.v[x] & 0x0F; // Seulement 0-F
                self.i = self.memory.get_font_address(character);
            },
            0x33 => { // LD B, Vx (BCD)
                let value = self.v[x];
                if self.i + 2 >= MAX_MEMORY {
                    web_sys::console::log_1(&"BCD: pas assez de place en mémoire".into());
                    self.error_count += 1;
                    return;
                }
                self.memory.write_byte(self.i, value / 100); // Centaines
                self.memory.write_byte(self.i + 1, (value / 10) % 10); // Dizaines
                self.memory.write_byte(self.i + 2, value % 10); // Unités
            },
            0x55 => { // LD [I], Vx
                if self.i as usize + x >= 4096 {
                    web_sys::console::log_1(&"Store: pas assez de place".into());
                    self.error_count += 1;
                    return;
                }
                for reg in 0..=x {
                    self.memory.write_byte(self.i + reg as u16, self.v[reg]);
                }
            },
            0x65 => { // LD Vx, [I]
                if self.i as usize + x >= 4096 {
                    web_sys::console::log_1(&"Load: pas assez de mémoire".into());
                    self.error_count += 1;
                    return;
                }
                for reg in 0..=x {
                    self.v[reg] = self.memory.read_byte(self.i + reg as u16);
                }
            },
            _ => {
                web_sys::console::log_1(&format!("Instruction Fx{:02X} inconnue", instruction & 0x00FF).into());
                self.error_count += 1;
            }
        }
    }
    
    // ========== FONCTIONS UTILITAIRES ==========
    
    /// Vérifier qu'une adresse est valide pour un programme
    fn is_valid_program_address(&self, addr: u16) -> bool {
        addr >= PROGRAM_START && addr < MAX_MEMORY && addr % 2 == 0
    }
    
    /// Obtenir les statistiques du CPU
    pub fn get_stats(&self) -> String {
        format!(
            "Cycles: {} | Erreurs: {} | Halted: {} | Stack: {}/{}",
            self.cycle_count, self.error_count, self.halted, self.sp, MAX_STACK_SIZE
        )
    }
    
    /// Vérifier l'état de santé du CPU
    pub fn is_healthy(&self) -> bool {
        !self.halted && self.error_count < 5 && self.sp < MAX_STACK_SIZE
    }
    
    //  FONCTIONS POUR JAVASCRIPT
    
    /// Retourner le buffer d'affichage pour JavaScript
    pub fn get_display_buffer(&self) -> Vec<u8> {
        self.display.get_buffer()
    }
    
    /// Gérer les touches
    pub fn key_down(&mut self, key: u8) {
        if key <= 15 {
            self.input.key_down(key);
        }
    }
    
    pub fn key_up(&mut self, key: u8) {
        if key <= 15 {
            self.input.key_up(key);
        }
    }
    
    /// Informations de debug complètes
    pub fn get_debug_info(&self) -> String {
        format!(
            "PC: 0x{:04X} | I: 0x{:04X} | SP: {} | DT: {} | ST: {} | V0-F: {:02X?} | Cycles: {} | Err: {}",
            self.pc, self.i, self.sp, self.delay_timer, self.sound_timer, self.v, self.cycle_count, self.error_count
        )
    }
    
    /// État simple pour l'interface
    pub fn is_running(&self) -> bool {
        !self.halted && self.is_healthy()
    }
}