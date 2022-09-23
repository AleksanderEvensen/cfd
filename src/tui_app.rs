use std::{error::Error, thread, sync::mpsc, path::{PathBuf}, time::Duration};

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use tui::{
    backend::{Backend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use crossterm::{
    event::{self, Event, KeyCode},
};

use crate::walker;


#[derive(Clone)]
struct PathEntry {
	text: String,
	score: i64,
	indecies: Vec<usize>
}

impl PathEntry {

	fn new(path: PathBuf) -> Self {
		Self {
			text: String::from(path.to_string_lossy()),
			indecies: vec![],
			score: 0
		}
	}

	fn calculate_score(&mut self, search: &String, matcher: &SkimMatcherV2) {
		if let Some(( score, indecies )) = matcher.fuzzy_indices(&self.text, search) {
			self.score = score;
			self.indecies = indecies;
		} else {
			self.score = 0;
			self.indecies = vec![];
		}
	}
}


struct TUIApp {
	search_path: PathBuf,
    input: String,
    selected: i32,
    paths: Vec<PathEntry>,
	visible_count: i32
}

impl TUIApp {
    fn new(path: PathBuf) -> Self {
        Self {
			search_path: path,
            input: String::new(),
            selected: 0,
            paths: vec![],
			visible_count: 0
        }
    }

    fn on_type(&mut self) {
        let matcher = SkimMatcherV2::default();
		for i in 0..self.paths.len() {
			self.paths.get_mut(i).unwrap().calculate_score(&self.input, &matcher);
		}
		self.visible_count = self.visible_paths().len() as i32;
        if self.selected > self.visible_count {
            self.selected = self.visible_count;
        }
    }
	
	fn visible_paths(&self) -> Vec<PathEntry> {
		let visible: Vec<PathEntry> = self.paths.iter().filter(|path| path.score > 0).cloned().collect();
		if visible.len() == 0 {
			return self.paths.clone();
		}
		return visible;
		
	}


	fn add_path(&mut self, path: PathBuf) {
		self.paths.push(PathEntry::new(path));
	}
}



pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
	path: PathBuf,
	depth: u8
) -> Result<Option<String>, Box<dyn Error>> {

	let mut app = TUIApp::new(path.clone());
    app.on_type();

	let (sender, reciever) = mpsc::channel::<PathBuf>();

	let search_path = path;
	thread::spawn(move || {
		walker::run(search_path, depth, sender);
	});

	loop {
		if let Ok(data) = reciever.recv() {
			app.add_path(data);
			terminal.draw(|f| ui(f, &app))?;
		}
		
		if let Ok(event_ready) = event::poll(Duration::from_secs(0)) {
			if event_ready {
				if let Event::Key(key) = event::read()? {
					app.on_type();
					match key.code {
						KeyCode::Enter => {
							// TODO: Set location to the current selected path

							if let Some(entry) = app.visible_paths().get(app.selected as usize) {
								return Ok(Some(entry.text.clone()));
							}
							return Ok(None);
						}
						KeyCode::Down => {
							if app.selected + 1 < app.visible_count as i32 {
								app.selected += 1;
							}
						}
						KeyCode::Up => {
							if app.selected - 1 >= 0 {
								app.selected -= 1;
							}
						}
		
						KeyCode::Char(c) => {
							app.input.push(c);
						}
						KeyCode::Backspace => {
							app.input.pop();
						}
						KeyCode::Esc => return Ok(None),
						_ => {}
					}
				}
				terminal.draw(|f| ui(f, &app))?;
			}
		}
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &TUIApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

	let full_path = std::fs::canonicalize(&app.search_path).unwrap();
	let full_path = full_path.to_string_lossy();
	let full_path = full_path.trim();

    let text = Text::from(Spans::from(vec![
        Span::raw("Search for path | Navigate with "),
        Span::styled("↓", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" and "),
        Span::styled("↑", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" | Press "),
        Span::styled("<Enter>", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to select | "),
        Span::raw("Press "),
        Span::styled("<Esc>", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to exit | Searching: "),
		Span::styled(full_path, Style::default().add_modifier(Modifier::BOLD))
    ]));
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(Text::from(Spans::from(vec![
        Span::styled("> ", Style::default().fg(Color::Red)),
        Span::raw(&app.input),
    ])))
    .style(Style::default());
    f.render_widget(input, chunks[1]);
    f.set_cursor(
        // Put cursor past the end of the input text
        chunks[1].x + app.input.len() as u16 + 2,
        // Move one line down, from the border to the input line
        chunks[1].y,
    );

    let result_count = Paragraph::new(Text::from(Span::raw(format!(
        "Showing {}/{} results",
        app.visible_count,
        app.paths.len()
    ))))
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().add_modifier(Modifier::DIM)),
    )
    .style(Style::default().add_modifier(Modifier::DIM));
    f.render_widget(result_count, chunks[2]);

    let paths: Vec<ListItem> = app.visible_paths()
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = Spans::from(vec![
                Span::styled(
                    if i as i32 == app.selected { ">" } else { " " },
                    Style::default()
                        .fg(Color::Red)
                        .add_modifier(Modifier::DIM)
                        .add_modifier(Modifier::BOLD),
                ),
				Span::raw(" "),
                Span::styled(format!("{}", m.text), if i as i32 == app.selected { Style::default().bg(Color::Indexed(8)) } else { Style::default() }),
            ]);
            ListItem::new(content)
        })
        .collect();
    let messages = List::new(paths).block(Block::default());
    f.render_widget(messages, chunks[3]);
}