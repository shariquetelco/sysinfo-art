/// Rain animation driven by disk usage.
/// Heavy disk activity = torrential downpour.

use std::collections::VecDeque;

const RAIN_CHARS: &[&str] = &["│", "╷", "┆", "╎", "⠂", "⠁"];
const SPLASH_CHARS: &[char] = &['·', '·', '○', '*', '✦'];

struct Drop {
    col: usize,
    row: f64,
    speed: f64,
    ch_idx: usize,
    length: usize,
}

pub struct Rain {
    pub width: usize,
    pub height: usize,
    drops: Vec<Drop>,
    splashes: VecDeque<(usize, usize, u8)>, // col, row, ttl
    tick: u64,
}

impl Rain {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            drops: Vec::new(),
            splashes: VecDeque::new(),
            tick: 0,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    pub fn render(&mut self, disk: f32) -> Vec<Vec<char>> {
        self.tick = self.tick.wrapping_add(1);

        let intensity = (disk / 100.0).max(0.05);
        // Number of drops scales with disk usage
        let target_drops = ((intensity * self.width as f32 * 0.8) as usize).max(1);

        // Spawn new drops
        if self.drops.len() < target_drops {
            let col = (self.tick as usize * 7 + self.drops.len() * 13) % self.width.max(1);
            let speed = 0.5 + intensity as f64 * 1.5 + (self.tick as f64 * 0.1).sin().abs() * 0.5;
            let ch_idx = (self.tick as usize) % RAIN_CHARS.len();
            let length = 1 + (intensity * 4.0) as usize;
            self.drops.push(Drop {
                col,
                row: 0.0,
                speed,
                ch_idx,
                length,
            });
        }

        let mut grid = vec![vec![' '; self.width]; self.height];

        // Update and draw drops
        let height = self.height;
        let mut new_splashes = Vec::new();

        self.drops.retain_mut(|drop| {
            drop.row += drop.speed;
            let row = drop.row as usize;

            if row >= height {
                // Splash on ground
                new_splashes.push((drop.col, height - 1, 3u8));
                return false;
            }

            // Draw drop trail
            let ch = RAIN_CHARS[drop.ch_idx];
            for i in 0..drop.length {
                if row >= i + 1 {
                    let trail_row = row - i;
                    if trail_row < height && drop.col < grid[0].len() {
                        let trail_char = ch.chars().next().unwrap_or('|');
                        grid[trail_row][drop.col] = trail_char;
                    }
                }
            }

            true
        });

        for s in new_splashes {
            self.splashes.push_back(s);
        }

        // Draw splashes
        self.splashes.retain_mut(|(col, row, ttl)| {
            if *ttl == 0 {
                return false;
            }
            let ch = SPLASH_CHARS[(3 - *ttl) as usize % SPLASH_CHARS.len()];
            if *row < height && *col < grid[0].len() {
                grid[*row][*col] = ch;
            }
            // Side splashes
            if *col > 0 && *row < height {
                grid[*row][*col - 1] = '·';
            }
            if *col + 1 < grid[0].len() && *row < height {
                grid[*row][*col + 1] = '·';
            }
            *ttl -= 1;
            true
        });

        grid
    }
}
