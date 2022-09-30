mod tui_app;
mod walker;
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, path::PathBuf};
use tui::{backend::CrosstermBackend, Terminal};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    path: Option<PathBuf>,

    #[clap(short, long, value_parser, default_value_t = 4)]
    depth: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // setup terminal
    let mut stdout = io::stderr();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = tui_app::run_app(
        &mut terminal,
        args.path.or_else(|| Some(PathBuf::from("./"))).unwrap(),
        args.depth,
    );

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Print result for usage in your shell config
    match res {
        Ok(result) => {
            if let Some(result) = result {
                println!("{}", result);
            }
        }
        Err(_) => {
            eprintln!("{}", "Shit");
            std::process::exit(1)
        }
    }

    Ok(())
}
