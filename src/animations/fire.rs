/// Fire/Ice animation driven by system temperature.
/// Hot temps = raging fire, cool temps = ice crystals.

const FIRE_CHARS: &[char] = &[' ', '.', ':', '^', '*', 'x', 's', 'S', '#', '$'];
const ICE_CHARS: &[char] = &[' ', '·', '°', '❄', '✦', '❆', '*', '◈', '◆'];

pub struct Fire {
    pub width: usize,
    pub height: usize,
    buffer: Vec<Vec<f32>>,
    tick: u64,
}

impl Fire {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![vec![0.0f32; width]; height];
        Self {
            width,
            height,
            buffer,
            tick: 0,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.buffer = vec![vec![0.0f32; width]; height];
        }
    }

    /// Returns (char, heat_value 0.0-1.0) per cell.
    /// heat_value drives the color (red/orange/yellow for fire, blue/white for ice).
    pub fn render(&mut self, temperature: f32) -> Vec<Vec<(char, f32)>> {
        self.tick = self.tick.wrapping_add(1);

        // Hot = fire (temp > 60), Cool = ice (temp < 35), mid = embers
        let is_fire = temperature > 45.0;
        let intensity = if is_fire {
            ((temperature - 45.0) / 55.0).clamp(0.0, 1.0)
        } else {
            ((45.0 - temperature) / 45.0).clamp(0.0, 1.0)
        };

        if is_fire {
            self.simulate_fire(intensity);
        } else {
            self.simulate_ice(intensity);
        }

        let chars = if is_fire { FIRE_CHARS } else { ICE_CHARS };
        let mut out = vec![vec![(' ', 0.0f32); self.width]; self.height];

        for row in 0..self.height {
            for col in 0..self.width {
                let heat = self.buffer[row][col].clamp(0.0, 1.0);
                let idx = ((heat * (chars.len() - 1) as f32) as usize).min(chars.len() - 1);
                out[row][col] = (chars[idx], heat);
            }
        }

        out
    }

    fn simulate_fire(&mut self, intensity: f32) {
        let t = self.tick as f32;
        let w = self.width;
        let h = self.height;

        // Seed fire at bottom row
        for col in 0..w {
            let noise = ((t * 0.3 + col as f32 * 0.7).sin() * 0.5 + 0.5)
                + ((t * 0.17 + col as f32 * 1.3).cos() * 0.3 + 0.3);
            self.buffer[h - 1][col] = (noise * intensity).min(1.0);
        }

        // Propagate upward
        for row in 1..h {
            let r = h - 1 - row; // bottom-up
            for col in 0..w {
                let left = if col > 0 { self.buffer[r + 1][col - 1] } else { 0.0 };
                let center = self.buffer[r + 1][col];
                let right = if col + 1 < w { self.buffer[r + 1][col + 1] } else { 0.0 };
                let avg = (left + center * 2.0 + right) / 4.0;
                let decay = 0.85 - (row as f32 / h as f32) * 0.3;
                self.buffer[r][col] = (avg * decay).max(0.0);
            }
        }
    }

    fn simulate_ice(&mut self, intensity: f32) {
        let t = self.tick as f32;
        let w = self.width;
        let h = self.height;

        // Crystalline growth pattern
        for row in 0..h {
            for col in 0..w {
                let cx = col as f32 / w as f32;
                let cy = row as f32 / h as f32;
                // Hexagonal-ish crystal pattern
                let val = ((cx * 6.28 + t * 0.05).sin()
                    + (cy * 6.28 + t * 0.03).cos()
                    + ((cx + cy) * 9.42 + t * 0.07).sin())
                    / 3.0;
                let normalized = (val * 0.5 + 0.5) * intensity;
                self.buffer[row][col] = normalized.clamp(0.0, 1.0);
            }
        }
    }
}
