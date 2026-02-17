/// Water level animation driven by RAM usage.
/// Full RAM = tank overflowing, low RAM = calm shallow pool.

const WATER_FILL: &[char] = &['░', '▒', '▓', '█'];
const SURFACE_CHARS: &[&str] = &["〜", "∿", "≈", "~"];
const BUBBLE_CHARS: &[char] = &['○', '◌', '◎', '°', '·'];

struct Bubble {
    col: usize,
    row: f64,
    speed: f64,
}

pub struct Water {
    pub width: usize,
    pub height: usize,
    bubbles: Vec<Bubble>,
    tick: u64,
    current_level: f64, // smooth animated fill level
}

impl Water {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            bubbles: Vec::new(),
            tick: 0,
            current_level: 0.5,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    /// Returns grid of (char, depth 0.0..=1.0 from surface)
    pub fn render(&mut self, ram_usage: f32) -> Vec<Vec<(char, f32)>> {
        self.tick = self.tick.wrapping_add(1);

        let target = (ram_usage / 100.0) as f64;

        // Smoothly animate toward target level
        let diff = target - self.current_level;
        self.current_level += diff * 0.08;

        let level = self.current_level.clamp(0.0, 1.0);
        let t = self.tick as f64;

        // Surface row (water starts here, goes down)
        // level=1.0 → surface at top, level=0.0 → surface at bottom
        let surface_row = ((1.0 - level) * self.height as f64) as usize;

        // Spawn bubbles at bottom of water, rising up
        if self.tick % 8 == 0 && level > 0.1 {
            let col = (self.tick as usize * 11) % self.width.max(1);
            self.bubbles.push(Bubble {
                col,
                row: self.height as f64 - 1.0,
                speed: 0.3 + (self.tick as f64 * 0.07).sin().abs() * 0.4,
            });
        }

        let mut grid = vec![vec![(' ', 0.0f32); self.width]; self.height];

        // Draw water body
        for row in 0..self.height {
            for col in 0..self.width {
                if row < surface_row {
                    // Above water: air
                    continue;
                }

                if row == surface_row {
                    // Surface ripple
                    let ripple = ((t * 0.15 + col as f64 * 0.4).sin() * 0.5 + 0.5) as f32;
                    let surf_idx = ((t * 2.0 + col as f64 * 0.5) as usize) % SURFACE_CHARS.len();
                    let ch = SURFACE_CHARS[surf_idx].chars().next().unwrap_or('~');
                    grid[row][col] = (ch, ripple);
                } else {
                    // Water body — depth drives fill density
                    let depth = (row - surface_row) as f32 / self.height as f32;
                    let fill_idx =
                        (depth * WATER_FILL.len() as f32) as usize;
                    let fill_idx = fill_idx.min(WATER_FILL.len() - 1);
                    grid[row][col] = (WATER_FILL[fill_idx], depth);
                }
            }
        }

        // Draw bubbles
        let height = self.height;
        let surface_row_copy = surface_row;
        self.bubbles.retain_mut(|b| {
            b.row -= b.speed;
            let row = b.row as usize;

            if row <= surface_row_copy || b.row < 0.0 {
                // Bubble pops at surface — add a tiny splash
                return false;
            }

            let col = b.col;
            let size = if b.row < (surface_row_copy + 3) as f64 { 0 } else { 1 };
            if row < height && col < grid[0].len() {
                grid[row][col] = (BUBBLE_CHARS[size], 0.9);
            }
            true
        });

        grid
    }
}
