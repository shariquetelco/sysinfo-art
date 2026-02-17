/// Ocean wave animation driven by CPU usage.
/// Higher CPU = rougher, taller waves.

const WAVE_CHARS: &[char] = &['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
const FOAM_CHARS: &[char] = &['~', '≈', '∿', '〜'];

pub struct Ocean {
    pub width: usize,
    pub height: usize,
    tick: u64,
    offsets: Vec<f64>,
}

impl Ocean {
    pub fn new(width: usize, height: usize) -> Self {
        let offsets = (0..width)
            .map(|i| (i as f64 * 0.4).sin() * 2.0)
            .collect();
        Self {
            width,
            height,
            tick: 0,
            offsets,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.offsets = (0..width)
                .map(|i| (i as f64 * 0.4).sin() * 2.0)
                .collect();
        }
    }

    /// Returns a grid of (char, is_foam) for rendering.
    pub fn render(&mut self, cpu: f32) -> Vec<Vec<(char, bool)>> {
        self.tick = self.tick.wrapping_add(1);

        let t = self.tick as f64;
        let intensity = (cpu / 100.0) as f64;

        // Wave amplitude scales with CPU (1..=height/2)
        let max_amp = (self.height as f64 * 0.6).max(1.0);
        let amplitude = 1.0 + intensity * max_amp;

        // Wave speed scales with CPU
        let speed = 0.08 + intensity * 0.18;

        let mut grid = vec![vec![(' ', false); self.width]; self.height];

        for col in 0..self.width {
            let phase = self.offsets[col];
            // Primary wave + secondary harmonic for complexity
            let wave_y = amplitude
                * (((t * speed + col as f64 * 0.3 + phase) * std::f64::consts::PI * 0.5).sin()
                    + 0.4
                        * ((t * speed * 1.7 + col as f64 * 0.5 + phase * 1.3)
                            * std::f64::consts::PI
                            * 0.5)
                            .sin());

            let surface_row = ((self.height as f64 / 2.0) - wave_y)
                .clamp(0.0, self.height as f64 - 1.0) as usize;

            // Fill water below surface
            for row in surface_row..self.height {
                let depth = row - surface_row;
                let ch = if depth == 0 {
                    // Surface foam
                    let foam_idx = ((t * 3.0 + col as f64) as usize) % FOAM_CHARS.len();
                    (FOAM_CHARS[foam_idx], true)
                } else {
                    // Deeper water — use block fill
                    let fill_idx = (depth * WAVE_CHARS.len() / self.height).min(WAVE_CHARS.len() - 1);
                    (WAVE_CHARS[fill_idx], false)
                };
                grid[row][col] = ch;
            }
        }

        grid
    }
}
