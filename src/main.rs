mod animations;
mod app;
mod metrics;
mod ui;

use std::time::{Duration, Instant};

use anyhow::Result;
use app::App;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

/// sysinfo-art — a living ASCII landscape driven by your system's vital signs.
#[derive(Parser, Debug)]
#[command(
    name = "sysinfo-art",
    version,
    about = "A living ASCII landscape driven by your system's vital signs",
    long_about = None
)]
struct Args {
    /// Refresh rate in milliseconds (default: 150)
    #[arg(short, long, default_value_t = 150)]
    rate: u64,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let tick_rate = Duration::from_millis(args.rate);

    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, tick_rate);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = res {
        eprintln!("Error: {e}");
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    tick_rate: Duration,
) -> Result<()> {
    let mut app = App::new();
    let mut last_tick = Instant::now();

    // Initial metrics collection
    app.update();

    loop {
        terminal.draw(|frame| ui::render(frame, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(())
                    }
                    KeyCode::Char('?') | KeyCode::Char('h') => {
                        app.show_help = !app.show_help;
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }
}
