//! Audio simple avec beep quand sound_timer > 0

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = ["window", "ferris8Audio"])]
    fn playBeep(frequency: f32, volume: f32);
    
    #[wasm_bindgen(js_namespace = ["window", "ferris8Audio"])]
    fn stopBeep();
}

pub struct Audio {
    volume: f32,
    enabled: bool,
    frequency: f32,
    is_playing: bool,
}

impl Audio {
    pub fn new() -> Self {
        Audio {
            volume: 0.5,
            enabled: true,
            frequency: 440.0,
            is_playing: false,
        }
    }
    
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }
    
    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency.clamp(100.0, 2000.0);
    }
    
    pub fn play_beep(&mut self) {
        if self.enabled && !self.is_playing {
            playBeep(self.frequency, self.volume);
            self.is_playing = true;
        }
    }
    
    pub fn stop_beep(&mut self) {
        if self.is_playing {
            stopBeep();
            self.is_playing = false;
        }
    }
    
    pub fn get_settings(&self) -> AudioSettings {
        AudioSettings {
            volume: self.volume,
            enabled: self.enabled,
            frequency: self.frequency,
        }
    }
}

pub struct AudioSettings {
    pub volume: f32,
    pub enabled: bool,
    pub frequency: f32,
}

impl Audio {
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }
}

