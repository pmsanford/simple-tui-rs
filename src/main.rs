use rand::{thread_rng, Rng};
use std::io;
use std::io::Read;
use std::time::Instant;
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    // Set up terminal output
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create a separate thread to poll stdin.
    // This provides non-blocking input support.
    let mut asi = async_stdin();

    // List of possible colors for the color changing box.
    let colors = [
        Color::White,
        Color::Red,
        Color::Blue,
        Color::Green,
        Color::Yellow,
    ];
    let mut cur_color = Color::White;

    let mut then = Instant::now();
    let mut rng = thread_rng();
    // Clear the terminal before first draw.
    terminal.clear()?;
    loop {
        // Check if it's been 3 seconds since the last time we
        // changed colors.
        let now = Instant::now();
        let msecs = now.duration_since(then).as_secs();
        if msecs >= 3 {
            // Choose a random color from the list.
            cur_color = colors[rng.gen_range(0..=4)];
            // Reset the previous time
            then = Instant::now();
        }

        // Lock the terminal and start a drawing session.
        terminal.draw(|frame| {
            // Create a layout into which to place our blocks.
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(80),
                    ]
                    .as_ref(),
                )
                .split(frame.size());

            // Create a block...
            let block = Block::default()
                // With a given title...
                .title("Color Changer")
                // Borders on every side...
                .borders(Borders::ALL)
                // The background of the current color...
                .style(Style::default().bg(cur_color));

            // Render into the first chunk of the layout.
            frame.render_widget(block, chunks[0]);

            // The text lines for our text box.
            let txt = vec![
                Spans::from("The box above will change colors every three seconds.\n"),
                Spans::from("Termion has a separate thread listening on stdin; pressing q will quit, but the main loop won't block waiting for it.\n"),
            ];
            // Create a paragraph with the above text...
            let graph = Paragraph::new(txt)
                // In a block with borders and the given title...
                .block(Block::default().title("Text box").borders(Borders::ALL))
                // With white foreground and black background...
                .style(Style::default().fg(Color::White).bg(Color::Black));

            // Render into the second chunk of the layout.
            frame.render_widget(graph, chunks[1]);
        })?;

        // Iterate over all the keys that have been pressed since the
        // last time we checked.
        for k in asi.by_ref().keys() {
            // Use if let if you're only checking for one key
            #[allow(clippy::single_match)]
            match k.unwrap() {
                // If any of them is q, quit
                Key::Char('q') => {
                    // Clear the terminal before exit so as not to leave
                    // a mess.
                    terminal.clear()?;
                    return Ok(());
                }
                // Otherwise, throw them away.
                _ => (),
            }
        }
    }
}
