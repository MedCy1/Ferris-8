//! Mémoire Chip-8 de 4KB
//! Zone réservée jusqu'à 0x1FF, programmes à partir de 0x200

const MEMORY_SIZE: usize = 4096;
const PROGRAM_START: usize = 0x200;
const PROGRAM_END: usize = 0x1000;
const FONTSET_START: usize = 0x50;
const FONTSET_SIZE: usize = 80;
const MAX_ROM_SIZE: usize = PROGRAM_END - PROGRAM_START;

// Fontset hexadécimal 0-F
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Memory {
    ram: [u8; MEMORY_SIZE],
    write_protected_zones: Vec<(usize, usize)>,
    access_count: u64,
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Memory {
            ram: [0; MEMORY_SIZE],
            write_protected_zones: Vec::new(),
            access_count: 0,
        };
        
        memory.write_protected_zones.push((FONTSET_START, FONTSET_START + FONTSET_SIZE));
        memory
    }
    
    pub fn clear(&mut self) {
        for i in PROGRAM_START..MEMORY_SIZE {
            self.ram[i] = 0;
        }
        
        for i in 0..FONTSET_START {
            self.ram[i] = 0;
        }
        
        self.access_count = 0;
    }
    
    pub fn load_fontset(&mut self) {
        for (i, &byte) in FONTSET.iter().enumerate() {
            self.ram[FONTSET_START + i] = byte;
        }
    }
    
    pub fn load_rom(&mut self, rom_data: &[u8]) -> bool {
        if rom_data.is_empty() || rom_data.len() > MAX_ROM_SIZE {
            return false;
        }
        
        for i in PROGRAM_START..MEMORY_SIZE {
            self.ram[i] = 0;
        }
        
        for (i, &byte) in rom_data.iter().enumerate() {
            self.ram[PROGRAM_START + i] = byte;
        }
        
        true
    }
    
    
    /// Lire un byte à une adresse donnée avec protection
    pub fn read_byte(&self, address: u16) -> u8 {
        let addr = address as usize;
        
        if addr >= MEMORY_SIZE {
            web_sys::console::log_1(
                &format!(" Lecture hors limites: 0x{:04X} >= 0x{:04X}", address, MEMORY_SIZE).into()
            );
            return 0;
        }
        
        // Statistiques d'accès
        if addr >= PROGRAM_START && self.access_count % 10000 == 0 {
            web_sys::console::log_1(&format!("{} accès mémoire", self.access_count).into());
        }
        
        self.ram[addr]
    }
    
    /// Écrire un byte à une adresse donnée avec protection
    pub fn write_byte(&mut self, address: u16, value: u8) {
        let addr = address as usize;
        
        if addr >= MEMORY_SIZE {
            web_sys::console::log_1(
                &format!(" Écriture hors limites: 0x{:04X} >= 0x{:04X}", address, MEMORY_SIZE).into()
            );
            return;
        }
        
        // Vérifier les zones protégées
        for &(start, end) in &self.write_protected_zones {
            if addr >= start && addr < end {
                web_sys::console::log_1(
                    &format!(" Tentative d'écriture en zone protégée: 0x{:04X} (zone 0x{:04X}-0x{:04X})", 
                            address, start, end).into()
                );
                return;
            }
        }
        
        // Avertissement si écriture dans zone système
        if addr < PROGRAM_START && addr >= FONTSET_START + FONTSET_SIZE {
            web_sys::console::log_1(
                &format!(" Écriture en zone système: 0x{:04X}", address).into()
            );
        }
        
        self.ram[addr] = value;
        self.access_count += 1;
    }
    
    /// Lire plusieurs bytes consécutifs avec validation
    pub fn read_bytes(&self, address: u16, count: u8) -> Vec<u8> {
        let mut result = Vec::with_capacity(count as usize);
        
        // Vérifier que la lecture complète est possible
        if address as usize + count as usize > MEMORY_SIZE {
            web_sys::console::log_1(
                &format!(" Lecture multi-bytes hors limites: 0x{:04X}+{} > 0x{:04X}", 
                        address, count, MEMORY_SIZE).into()
            );
            // Retourner des zéros pour éviter le crash
            return vec![0; count as usize];
        }
        
        for i in 0..count {
            result.push(self.read_byte(address + i as u16));
        }
        result
    }
    
    /// Écrire plusieurs bytes consécutifs avec validation
    pub fn write_bytes(&mut self, address: u16, data: &[u8]) -> bool {
        // Vérifier que l'écriture complète est possible
        if address as usize + data.len() > MEMORY_SIZE {
            web_sys::console::log_1(
                &format!(" Écriture multi-bytes hors limites: 0x{:04X}+{} > 0x{:04X}", 
                        address, data.len(), MEMORY_SIZE).into()
            );
            return false;
        }
        
        for (i, &byte) in data.iter().enumerate() {
            self.write_byte(address + i as u16, byte);
        }
        true
    }
    
    /// Obtenir l'adresse d'un caractère de font avec validation
    pub fn get_font_address(&self, character: u8) -> u16 {
        if character > 0xF {
            web_sys::console::log_1(
                &format!(" Caractère font invalide: 0x{:02X}, limité à 0-F", character).into()
            );
            return FONTSET_START as u16; // Retourner '0' par défaut
        }
        
        // Chaque caractère fait 5 bytes
        FONTSET_START as u16 + (character as u16 * 5)
    }
    
    /// Obtenir des statistiques de la mémoire
    pub fn get_stats(&self) -> String {
        let program_bytes = self.count_non_zero_bytes(PROGRAM_START, MEMORY_SIZE);
        let font_bytes = FONTSET_SIZE;
        
        format!(
            "Mémoire: {}B programme, {}B fonts, {} accès total",
            program_bytes, font_bytes, self.access_count
        )
    }
    
    /// Compter les bytes non-zéro dans une zone
    fn count_non_zero_bytes(&self, start: usize, end: usize) -> usize {
        self.ram[start..end].iter().filter(|&&b| b != 0).count()
    }
    
    /// Dump hexadécimal d'une zone mémoire pour debug
    pub fn hex_dump(&self, start: u16, length: u16) -> String {
        let start_addr = start as usize;
        let end_addr = (start as usize + length as usize).min(MEMORY_SIZE);
        
        let mut dump = format!(" Dump mémoire 0x{:04X}-0x{:04X}:\n", start, end_addr - 1);
        
        for addr in (start_addr..end_addr).step_by(16) {
            dump.push_str(&format!("{:04X}: ", addr));
            
            // Afficher les bytes en hex
            for i in 0..16 {
                if addr + i < end_addr {
                    dump.push_str(&format!("{:02X} ", self.ram[addr + i]));
                } else {
                    dump.push_str("   ");
                }
            }
            
            dump.push_str(" |");
            
            // Afficher les caractères ASCII (si imprimables)
            for i in 0..16 {
                if addr + i < end_addr {
                    let byte = self.ram[addr + i];
                    if byte >= 32 && byte <= 126 {
                        dump.push(byte as char);
                    } else {
                        dump.push('.');
                    }
                } else {
                    dump.push(' ');
                }
            }
            
            dump.push_str("|\n");
        }
        
        dump
    }
    
    /// Validation complète de l'intégrité mémoire
    pub fn validate_integrity(&self) -> bool {
        let mut valid = true;
        
        // Vérifier que les fonts sont intacts
        for (i, &expected) in FONTSET.iter().enumerate() {
            if self.ram[FONTSET_START + i] != expected {
                web_sys::console::log_1(
                    &format!(" Font corrompu à l'index {}: attendu 0x{:02X}, trouvé 0x{:02X}", 
                            i, expected, self.ram[FONTSET_START + i]).into()
                );
                valid = false;
            }
        }
        
        if valid {
            web_sys::console::log_1(&"Intégrité mémoire vérifiée".into());
        }
        
        valid
    }
    
    /// Obtenir des infos sur une adresse spécifique
    pub fn get_address_info(&self, address: u16) -> String {
        let addr = address as usize;
        
        if addr >= MEMORY_SIZE {
            return format!("0x{:04X}: HORS LIMITES", address);
        }
        
        let zone = if addr < FONTSET_START {
            "Système"
        } else if addr < FONTSET_START + FONTSET_SIZE {
            "Fonts"
        } else if addr < PROGRAM_START {
            "Libre"
        } else {
            "Programme"
        };
        
        format!("0x{:04X}: {} = 0x{:02X}", address, zone, self.ram[addr])
    }
}