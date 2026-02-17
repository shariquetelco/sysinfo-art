use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

// ── Palette ─────────────────────────────────────────────────────────────────
fn cpu_color(cpu: f32) -> Color {
    if cpu < 30.0 {
        Color::Rgb(0, 180, 255)
    } else if cpu < 60.0 {
        Color::Rgb(0, 230, 200)
    } else if cpu < 80.0 {
        Color::Rgb(255, 200, 0)
    } else {
        Color::Rgb(255, 60, 60)
    }
}

fn ram_color(ram: f32) -> Color {
    if ram < 50.0 {
        Color::Rgb(0, 200, 180)
    } else if ram < 75.0 {
        Color::Rgb(100, 180, 255)
    } else {
        Color::Rgb(220, 80, 255)
    }
}

fn temp_color(t: f32) -> Color {
    if t < 40.0 {
        Color::Rgb(100, 200, 255)
    } else if t < 60.0 {
        Color::Rgb(255, 200, 50)
    } else if t < 80.0 {
        Color::Rgb(255, 120, 20)
    } else {
        Color::Rgb(255, 30, 30)
    }
}

fn disk_color(d: f32) -> Color {
    if d < 50.0 {
        Color::Rgb(100, 220, 120)
    } else if d < 80.0 {
        Color::Rgb(255, 220, 0)
    } else {
        Color::Rgb(255, 80, 30)
    }
}

fn net_color(n: f32) -> Color {
    if n < 20.0 {
        Color::Rgb(150, 200, 255)
    } else if n < 60.0 {
        Color::Rgb(80, 160, 255)
    } else {
        Color::Rgb(0, 100, 255)
    }
}

// ── Heat-map color (0.0 → 1.0) ──────────────────────────────────────────────
fn fire_heat_color(heat: f32) -> Color {
    let h = heat.clamp(0.0, 1.0);
    let r = (255.0 * h) as u8;
    let g = (180.0 * (h * h)) as u8;
    let b = 20u8;
    Color::Rgb(r, g, b)
}

fn ice_color(v: f32) -> Color {
    let v = v.clamp(0.0, 1.0);
    let r = (100.0 + 155.0 * (1.0 - v)) as u8;
    let g = (180.0 + 75.0 * (1.0 - v)) as u8;
    let b = 255u8;
    Color::Rgb(r, g, b)
}

fn water_depth_color(depth: f32, ram: f32) -> Color {
    let d = depth.clamp(0.0, 1.0);
    let pressure = (ram / 100.0).clamp(0.0, 1.0);
    let r = (20.0 + 60.0 * pressure) as u8;
    let g = (100.0 + 80.0 * (1.0 - d)) as u8;
    let b = (200.0 + 55.0 * (1.0 - d * 0.5)) as u8;
    Color::Rgb(r, g, b)
}

// ── Render entry point ───────────────────────────────────────────────────────
pub fn render(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    // Root: title + body + footer
    let root = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(area);

    render_title(frame, root[0], app);
    render_body(frame, root[1], app);
    render_footer(frame, root[2], app);
}

// ── Title bar ────────────────────────────────────────────────────────────────
fn render_title(frame: &mut Frame, area: Rect, app: &App) {
    let title = Paragraph::new(Line::from(vec![
        Span::styled(
            " ⛰  sysinfo-art ",
            Style::default()
                .fg(Color::Rgb(0, 220, 255))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "│ ",
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(
            format!("CPU {:.0}%", app.metrics.cpu_usage),
            Style::default().fg(cpu_color(app.metrics.cpu_usage)),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            format!("RAM {:.0}%", app.metrics.ram_usage),
            Style::default().fg(ram_color(app.metrics.ram_usage)),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            format!("TEMP {:.0}°C", app.metrics.temperature),
            Style::default().fg(temp_color(app.metrics.temperature)),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            format!("DISK {:.0}%", app.metrics.disk_usage),
            Style::default().fg(disk_color(app.metrics.disk_usage)),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            format!(
                "NET ↓{:.1} ↑{:.1} MB/s",
                app.metrics.net_rx_mb, app.metrics.net_tx_mb
            ),
            Style::default().fg(net_color(app.metrics.net_activity)),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(0, 100, 160))),
    )
    .alignment(Alignment::Left);

    frame.render_widget(title, area);
}

// ── Footer ───────────────────────────────────────────────────────────────────
fn render_footer(frame: &mut Frame, area: Rect, _app: &App) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" [q] quit", Style::default().fg(Color::DarkGray)),
        Span::styled("  [?] help", Style::default().fg(Color::DarkGray)),
        Span::styled(
            "  github.com/shariquetelco/sysinfo-art",
            Style::default().fg(Color::Rgb(0, 80, 120)),
        ),
    ]));
    frame.render_widget(footer, area);
}

// ── Body layout ──────────────────────────────────────────────────────────────
fn render_body(frame: &mut Frame, area: Rect, app: &mut App) {
    // Split into top (ocean) and bottom two panels
    let rows = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .split(area);

    // Top row: full-width ocean (CPU)
    render_ocean_panel(frame, rows[0], app);

    // Bottom row: rain | water | fire+wind stacked
    let bottom_cols = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ],
    )
    .split(rows[1]);

    render_rain_panel(frame, bottom_cols[0], app);
    render_water_panel(frame, bottom_cols[1], app);
    render_fire_wind_panel(frame, bottom_cols[2], app);
}

// ── Ocean panel (CPU) ────────────────────────────────────────────────────────
fn render_ocean_panel(frame: &mut Frame, area: Rect, app: &mut App) {
    let inner = area.inner(ratatui::layout::Margin {
        horizontal: 1,
        vertical: 1,
    });
    let w = inner.width as usize;
    let h = inner.height as usize;

    app.ocean.resize(w.max(1), h.max(1));
    let grid = app.ocean.render(app.metrics.cpu_usage);

    let lines: Vec<Line> = (0..h.min(grid.len()))
        .map(|row| {
            let spans: Vec<Span> = (0..w.min(grid[row].len()))
                .map(|col| {
                    let (ch, is_foam) = grid[row][col];
                    let color = if is_foam {
                        Color::Rgb(200, 240, 255)
                    } else {
                        // Deeper = darker blue
                        let depth = row as f32 / h as f32;
                        let r = (20.0 + 30.0 * (1.0 - depth)) as u8;
                        let g = (80.0 + 80.0 * (1.0 - depth)) as u8;
                        let b = (180.0 + 60.0 * (1.0 - depth)) as u8;
                        Color::Rgb(r, g, b)
                    };
                    Span::styled(
                        ch.to_string(),
                        Style::default()
                            .fg(color)
                            .bg(Color::Rgb(5, 15, 35)),
                    )
                })
                .collect();
            Line::from(spans)
        })
        .collect();

    let label = format!(
        "⌊ CPU OCEAN — {:.1}% ⌋",
        app.metrics.cpu_usage
    );
    let block = Block::default()
        .title(Span::styled(
            label,
            Style::default()
                .fg(Color::Rgb(0, 200, 255))
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(0, 80, 140)))
        .style(Style::default().bg(Color::Rgb(5, 15, 35)));

    let p = Paragraph::new(lines).block(block);
    frame.render_widget(p, area);
}

// ── Rain panel (Disk) ────────────────────────────────────────────────────────
fn render_rain_panel(frame: &mut Frame, area: Rect, app: &mut App) {
    let inner = area.inner(ratatui::layout::Margin {
        horizontal: 1,
        vertical: 1,
    });
    let w = inner.width as usize;
    let h = inner.height as usize;

    app.rain.resize(w.max(1), h.max(1));
    let grid = app.rain.render(app.metrics.disk_usage);

    let lines: Vec<Line> = (0..h.min(grid.len()))
        .map(|row| {
            let spans: Vec<Span> = (0..w.min(grid[row].len()))
                .map(|col| {
                    let ch = grid[row][col];
                    let color = if ch == ' ' {
                        Color::Rgb(15, 20, 30)
                    } else if ch == '·' || ch == '*' || ch == '○' || ch == '✦' {
                        Color::Rgb(180, 220, 255)
                    } else {
                        Color::Rgb(100, 160, 255)
                    };
                    Span::styled(
                        ch.to_string(),
                        Style::default()
                            .fg(color)
                            .bg(Color::Rgb(8, 12, 22)),
                    )
                })
                .collect();
            Line::from(spans)
        })
        .collect();

    let label = format!("⌊ DISK RAIN — {:.0}% ⌋", app.metrics.disk_usage);
    let block = Block::default()
        .title(Span::styled(
            label,
            Style::default()
                .fg(disk_color(app.metrics.disk_usage))
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(40, 80, 130)))
        .style(Style::default().bg(Color::Rgb(8, 12, 22)));

    let p = Paragraph::new(lines).block(block);
    frame.render_widget(p, area);
}

// ── Water panel (RAM) ────────────────────────────────────────────────────────
fn render_water_panel(frame: &mut Frame, area: Rect, app: &mut App) {
    let inner = area.inner(ratatui::layout::Margin {
        horizontal: 1,
        vertical: 1,
    });
    let w = inner.width as usize;
    let h = inner.height as usize;

    app.water.resize(w.max(1), h.max(1));
    let grid = app.water.render(app.metrics.ram_usage);

    let ram = app.metrics.ram_usage;
    let lines: Vec<Line> = (0..h.min(grid.len()))
        .map(|row| {
            let spans: Vec<Span> = (0..w.min(grid[row].len()))
                .map(|col| {
                    let (ch, depth) = grid[row][col];
                    let color = if ch == ' ' {
                        Color::Rgb(12, 18, 28)
                    } else if ch == '~' || ch == '∿' || ch == '≈' || ch == '〜' {
                        Color::Rgb(180, 240, 255)
                    } else if ch == '○' || ch == '◌' || ch == '°' || ch == '·' || ch == '◎' {
                        Color::Rgb(200, 240, 255)
                    } else {
                        water_depth_color(depth, ram)
                    };
                    Span::styled(
                        ch.to_string(),
                        Style::default()
                            .fg(color)
                            .bg(Color::Rgb(8, 15, 28)),
                    )
                })
                .collect();
            Line::from(spans)
        })
        .collect();

    let label = format!(
        "⌊ RAM TANK — {:.1}/{:.1} GB ⌋",
        app.metrics.ram_used_gb, app.metrics.ram_total_gb
    );
    let block = Block::default()
        .title(Span::styled(
            label,
            Style::default()
                .fg(ram_color(ram))
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(30, 80, 140)))
        .style(Style::default().bg(Color::Rgb(8, 15, 28)));

    let p = Paragraph::new(lines).block(block);
    frame.render_widget(p, area);
}

// ── Fire + Wind panel (Temp / Network) ──────────────────────────────────────
fn render_fire_wind_panel(frame: &mut Frame, area: Rect, app: &mut App) {
    let halves = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(55), Constraint::Percentage(45)],
    )
    .split(area);

    render_fire_sub(frame, halves[0], app);
    render_wind_sub(frame, halves[1], app);
}

fn render_fire_sub(frame: &mut Frame, area: Rect, app: &mut App) {
    let inner = area.inner(ratatui::layout::Margin {
        horizontal: 1,
        vertical: 1,
    });
    let w = inner.width as usize;
    let h = inner.height as usize;

    let temp = app.metrics.temperature;
    app.fire.resize(w.max(1), h.max(1));
    let grid = app.fire.render(temp);

    let is_fire = temp > 45.0;
    let lines: Vec<Line> = (0..h.min(grid.len()))
        .map(|row| {
            let spans: Vec<Span> = (0..w.min(grid[row].len()))
                .map(|col| {
                    let (ch, heat) = grid[row][col];
                    let color = if ch == ' ' {
                        Color::Rgb(8, 5, 5)
                    } else if is_fire {
                        fire_heat_color(heat)
                    } else {
                        ice_color(heat)
                    };
                    let bg = if is_fire {
                        Color::Rgb(8, 5, 5)
                    } else {
                        Color::Rgb(5, 8, 18)
                    };
                    Span::styled(ch.to_string(), Style::default().fg(color).bg(bg))
                })
                .collect();
            Line::from(spans)
        })
        .collect();

    let icon = if temp > 70.0 {
        "🔥"
    } else if temp > 45.0 {
        "♨"
    } else {
        "❄"
    };
    let label = format!("{} TEMP — {:.0}°C", icon, temp);
    let border_color = if is_fire {
        Color::Rgb(180, 60, 20)
    } else {
        Color::Rgb(60, 120, 200)
    };
    let block = Block::default()
        .title(Span::styled(
            label,
            Style::default()
                .fg(temp_color(temp))
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color));

    let p = Paragraph::new(lines).block(block);
    frame.render_widget(p, area);
}

fn render_wind_sub(frame: &mut Frame, area: Rect, app: &mut App) {
    let inner = area.inner(ratatui::layout::Margin {
        horizontal: 1,
        vertical: 1,
    });
    let w = inner.width as usize;
    let h = inner.height as usize;

    let net = app.metrics.net_activity;
    app.wind.resize(w.max(1), h.max(1));
    let grid = app.wind.render(net);

    let lines: Vec<Line> = (0..h.min(grid.len()))
        .map(|row| {
            let spans: Vec<Span> = (0..w.min(grid[row].len()))
                .map(|col| {
                    let (ch, intensity) = grid[row][col];
                    let color = if ch == ' ' {
                        Color::Rgb(10, 10, 15)
                    } else {
                        let i = intensity.clamp(0.0, 1.0);
                        let r = (100.0 + 155.0 * i) as u8;
                        let g = (140.0 + 115.0 * i) as u8;
                        let b = (200.0 + 55.0 * i) as u8;
                        Color::Rgb(r, g, b)
                    };
                    Span::styled(
                        ch.to_string(),
                        Style::default()
                            .fg(color)
                            .bg(Color::Rgb(5, 5, 12)),
                    )
                })
                .collect();
            Line::from(spans)
        })
        .collect();

    let label = format!(
        "⌊ NET WIND ↓{:.1} ↑{:.1} MB/s ⌋",
        app.metrics.net_rx_mb, app.metrics.net_tx_mb
    );
    let block = Block::default()
        .title(Span::styled(
            label,
            Style::default()
                .fg(net_color(net))
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(40, 60, 120)));

    let p = Paragraph::new(lines).block(block);
    frame.render_widget(p, area);
}
