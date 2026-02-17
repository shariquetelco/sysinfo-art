/// Wind animation driven by network activity.
/// High network I/O = strong gusts, idle = gentle breeze.

const WIND_STREAKS: &[&str] = &[
    "─", "━", "═", "≡", "≣",
];
const GUST_CHARS: &[char] = &['-', '~', '≈', '≋', '⟿'];

struct Particle {
    col: f64,
    row: usize,
    speed: f64,
    ch: char,
    ttl: u32,
}

pub struct Wind {
    pub width: usize,
    pub height: usize,
    particles: Vec<Particle>,
    tick: u64,
}

impl Wind {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            particles: Vec::new(),
            tick: 0,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    pub fn render(&mut self, net_activity: f32) -> Vec<Vec<(char, f32)>> {
        self.tick = self.tick.wrapping_add(1);

        let intensity = (net_activity / 100.0).clamp(0.02, 1.0);
        let target = ((intensity * self.width as f32 * 0.6) as usize).max(1);

        // Spawn particles from left edge
        if self.particles.len() < target {
            let row = (self.tick as usize * 3 + self.particles.len() * 7) % self.height.max(1);
            let speed = 1.0 + intensity as f64 * 5.0;
            let ch_idx = (self.tick as usize / 3) % GUST_CHARS.len();
            self.particles.push(Particle {
                col: 0.0,
                row,
                speed,
                ch: GUST_CHARS[ch_idx],
                ttl: (self.width as f64 / speed) as u32 + 3,
            });
        }

        let mut grid = vec![vec![(' ', 0.0f32); self.width]; self.height];

        self.particles.retain_mut(|p| {
            p.col += p.speed;
            p.ttl = p.ttl.saturating_sub(1);

            let col = p.col as usize;
            if col >= self.width || p.ttl == 0 {
                return false;
            }

            // Draw a streak behind the particle
            let streak_len = (p.speed * 2.0) as usize + 1;
            for i in 0..streak_len {
                if col >= i {
                    let sc = col - i;
                    if sc < grid[0].len() {
                        let alpha = 1.0 - (i as f32 / streak_len as f32);
                        let streak_char = if i == 0 {
                            p.ch
                        } else {
                            WIND_STREAKS
                                .get(i.min(WIND_STREAKS.len() - 1))
                                .and_then(|s| s.chars().next())
                                .unwrap_or('-')
                        };
                        if p.row < grid.len() {
                            grid[p.row][sc] = (streak_char, alpha * intensity);
                        }
                    }
                }
            }

            true
        });

        // Add subtle background turbulence at low activity
        let t = self.tick as f64;
        for row in 0..self.height {
            for col in 0..self.width {
                if grid[row][col].0 == ' ' {
                    let noise =
                        ((t * 0.05 + col as f64 * 0.3 + row as f64 * 0.7).sin() * 0.5 + 0.5)
                            as f32;
                    if noise > 0.9 && intensity > 0.3 {
                        grid[row][col] = ('·', noise * intensity);
                    }
                }
            }
        }

        grid
    }
}
