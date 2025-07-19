//! Audio simple avec beep quand sound_timer > 0

pub struct Audio {
    volume: f32,
    enabled: bool,
    frequency: f32,
}

impl Audio {
    pub fn new() -> Self {
        Audio {
            volume: 0.5,
            enabled: true,
            frequency: 440.0,
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
    
    pub fn play_beep(&self) {
        // TODO: Web Audio API
    }
    
    pub fn stop_beep(&self) {
        // TODO: Web Audio API
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

