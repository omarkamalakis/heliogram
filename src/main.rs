#[allow(dead_code)]

use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
// use termion::*;
use tui::widgets::{Text, Paragraph, Widget, Block, Borders};
use tui::style::{Style, Color};

// use tui::layout::{Layout, Constraint, Direction};

// Test window

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // let mut counter = 0;
    let text = [
            Text::raw("Testing the paragraph engine")
    ];
    terminal.draw(|mut f| {
            let size = f.size();
            Block::default()
                .title("Block")
                .borders(Borders::ALL)
                .render(&mut f, size);
    //         Block::default()
    //             .title("Blah")
    //             .title_style(Style::default().fg(Color::Blue))
    //             .borders(Borders::ALL)
    //             .border_style(Style::default().fg(Color::Green))
    //             .render(&mut f, size);
    // });
        Paragraph::new(text.iter())
            .style(Style::default().fg(Color::Red));

})
}



