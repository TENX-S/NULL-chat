#![allow(dead_code)]


mod util;


use std::{
    io,
    error::Error,
};
use crate::util::{
    input::*,
    event::{
        Event,
        Events,
    },
    message::Message,
    board::ChatBoard,
};
use termion::{
    event::Key,
    raw::IntoRawMode,
    input::MouseTerminal,
    screen::AlternateScreen,
};
use tui::{
    text::*,
    style::*,
    widgets::*,
    Terminal,
    layout::{
        Corner,
        Layout,
        Direction,
        Constraint,
    },
    backend::TermionBackend,
};
use unicode_width::UnicodeWidthStr;


const __NAME__: &str = "NULL chat";
const __VERSION__: &str = "v0.0.1";



fn main() -> Result<(), Box<dyn Error>> {

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Handle events from keyboard
    let mut events = Events::new();

    // chat_board
    let mut chat_board = ChatBoard::new();

    // input ui
    let mut input_ui = InputUI::default();

    loop {

        terminal.draw(|f| {

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(8),  // NULL chat info
                        Constraint::Percentage(70), // Message ChatBoard
                        Constraint::Min(1),
                        Constraint::Percentage(20), // Users input
                    ].as_ref()
                )
                .split(f.size());


            // NULL chat info UI

            let info_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick);


            let info_content = vec![
                Spans::from(vec![
                    {
                        let info_width = f.size().width;
                        let name_width = __NAME__.len() as u16;
                        let version_width = __VERSION__.len() as u16;
                        let blank_width = (info_width - name_width - version_width - 1) / 2;

                        let mut blank: String = String::new();

                        for _ in 0..blank_width {
                            blank.push_str(" ");
                        }

                        Span::raw(blank)

                    },
                    Span::styled(__NAME__, Style::default().fg(Color::Blue)),
                    Span::raw(" "),
                    Span::styled(__VERSION__, Style::default().fg(Color::Red)),
                    Span::raw("."),
                ]),

            ];

            let info = Paragraph::new(info_content).block(info_block);

            f.render_widget(info, chunks[0]);


            // Message ChatBoard UI

            let messages: Vec<ListItem> = chat_board
                .entries()
                .items
                .iter()
                .map(|i| {

                    let mut lines = vec!
                    [
                        Spans::from(Span::styled
                            (
                                format!("{} {}", i.sender_name(), i.time()),
                                Style::default().add_modifier(Modifier::BOLD)
                            )
                        )
                    ];

                    lines.push(Spans::from(""));

                    lines.push(
                        Spans::from(
                            Span::styled(
                                format!("{}", i.val()),
                                Style::default().add_modifier(Modifier::UNDERLINED)
                            )
                        )
                    );


                    ListItem::new(lines).style(
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Gray)
                    )

                })
                .collect();

            // messages.reverse();

            let messages_board = List::new(messages)
                .start_corner(Corner::BottomLeft)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Thick)
                )
                .highlight_style(
                    Style::default()
                        .bg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD)
                )
                .highlight_symbol("> ");

            f.render_stateful_widget(messages_board, chunks[1], &mut chat_board.entries().state);

            // Tips

            let (tip, style) = match input_ui.input_mode {
                InputMode::Normal => (
                    vec![
                        Span::raw("Press "),
                        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to exit, "),
                        Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to start editing."),
                    ],
                    Style::default().add_modifier(Modifier::SLOW_BLINK),
                ),
                InputMode::Editing => (
                    vec![
                        Span::raw("Press "),
                        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to stop editing, "),
                        Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(" to record the message"),
                    ],
                    Style::default(),
                ),
            };

            let mut tip_text = Text::from(Spans::from(tip));
            tip_text.patch_style(style);
            let help_message = Paragraph::new(tip_text);
            f.render_widget(help_message, chunks[2]);

            // User Input UI

            let input_box = Paragraph::new(input_ui.input_message.as_ref())
                .style(match input_ui.input_mode {
                    InputMode::Normal => Style::default().fg(Color::LightRed),
                    InputMode::Editing => Style::default().fg(Color::LightYellow),
                })
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Double)
                );


            match input_ui.input_mode {
                InputMode::Normal =>
                // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
                    {}

                InputMode::Editing => {
                    // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
                    f.set_cursor(

                        // Put cursor past the end of the input text
                        chunks[3].x + input_ui.input_message.width() as u16 + 1,
                        // Move one line down, from the border to the input line
                        chunks[3].y + 1,

                    )
                }
            }

            f.render_widget(input_box, chunks[3]);

        })?;


        if let Event::Input(input) = events.next()? {

            match input_ui.input_mode {

                InputMode::Normal => match input {

                    Key::Char('e') => {
                        input_ui.input_mode = InputMode::Editing;
                        events.disable_exit_key();
                    },

                    Key::Char('q') => {
                        break;
                    },

                    Key::Left => {
                        chat_board.entries().unselect();
                    },

                    Key::Down => {
                        chat_board.entries().previous();
                    },

                    Key::Up => {
                        chat_board.entries().next();
                    },

                    _ => continue

                },

                InputMode::Editing => match input {

                    Key::Char('\n') => {

                        chat_board.entries().items.insert(0, Message::new("TENX", &input_ui.input_message.drain(..).collect::<String>()));


                    },

                    Key::Char(word_char) => {
                        input_ui.input_message.push(word_char);
                    },

                    Key::Backspace => {
                        input_ui.input_message.pop();
                    },

                    Key::Esc => {
                        input_ui.input_mode = InputMode::Normal;
                        events.enable_exit_key();
                    },

                    _ => continue

                },

            }
        }

    }

    Ok(())

}
