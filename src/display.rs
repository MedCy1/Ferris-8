//! Écran 64x32 pixels noir et blanc

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_PIXELS: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

pub struct Display {
    pixels: [bool; DISPLAY_PIXELS],
}

impl Display {
    pub fn new() -> Self {
        Display {
            pixels: [false; DISPLAY_PIXELS],
        }
    }
    
    pub fn clear(&mut self) {
        self.pixels = [false; DISPLAY_PIXELS];
    }
    
    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        if x < DISPLAY_WIDTH && y < DISPLAY_HEIGHT {
            self.pixels[y * DISPLAY_WIDTH + x]
        } else {
            false
        }
    }
    
    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        let wrapped_x = x % DISPLAY_WIDTH;
        let wrapped_y = y % DISPLAY_HEIGHT;
        self.pixels[wrapped_y * DISPLAY_WIDTH + wrapped_x] = value;
    }
    
    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite_data: &[u8]) -> bool {
        let mut collision = false;
        
        for (row, &sprite_byte) in sprite_data.iter().enumerate() {
            for col in 0..8 {
                let sprite_pixel = (sprite_byte >> (7 - col)) & 1 == 1;
                
                if sprite_pixel {
                    let pixel_x = (x + col) % DISPLAY_WIDTH;
                    let pixel_y = (y + row) % DISPLAY_HEIGHT;
                    let pixel_index = pixel_y * DISPLAY_WIDTH + pixel_x;
                    
                    let old_pixel = self.pixels[pixel_index];
                    self.pixels[pixel_index] = old_pixel ^ sprite_pixel;
                    
                    if old_pixel && !self.pixels[pixel_index] {
                        collision = true;
                    }
                }
            }
        }
        
        collision
    }
    
    pub fn get_buffer(&self) -> Vec<u8> {
        self.pixels.iter()
            .map(|&pixel| if pixel { 255 } else { 0 })
            .collect()
    }
    
    #[allow(dead_code)]
    pub fn debug_print(&self) {
        let mut screen = String::new();
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                screen.push(if self.get_pixel(x, y) { '█' } else { ' ' });
            }
            screen.push('\n');
        }
        web_sys::console::log_1(&screen.into());
    }
    
    pub fn count_active_pixels(&self) -> usize {
        self.pixels.iter().filter(|&&pixel| pixel).count()
    }
}