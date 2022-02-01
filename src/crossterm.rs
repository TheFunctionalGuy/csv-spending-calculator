use std::{io, error::Error, time::Duration, thread};

use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture}};
use tui::{Terminal, backend::{CrosstermBackend, Backend}, Frame, widgets::{Block, Borders}, layout::{Layout, Direction, Constraint}};

pub fn run() -> Result<(), Box<dyn Error>> {
	// Terminal Initialization
	enable_raw_mode()?;
	let mut stdout = io::stdout();
	execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;

	terminal.draw(|f| ui(f))?;

	thread::sleep(Duration::from_millis(5000));
	// loop {
	// 	terminal.draw(|f| draw(f))?;
	// }

	// Terminal Termination
	disable_raw_mode()?;
	execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
	terminal.show_cursor()?;
	Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>) {
	let chunks = Layout::default()
		 .direction(Direction::Vertical)
		 .margin(1)
		 .constraints(
			 [
				 Constraint::Percentage(10),
				 Constraint::Percentage(80),
				 Constraint::Percentage(10)
			 ].as_ref()
		 )
		 .split(f.size());
	let block = Block::default()
		.title("Block")
		.borders(Borders::ALL);
	f.render_widget(block, chunks[0]);
	let block = Block::default()
		.title("Block 2")
		.borders(Borders::ALL);
	f.render_widget(block, chunks[1]);
}

// TODO: Maybe move
fn draw<B: Backend>(frame: &mut Frame<B>) {
	
}