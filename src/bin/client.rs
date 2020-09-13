
#![recursion_limit="4096"]

extern crate termion;

use termion::screen::*;
use termion::{ color, style };
use std::io::{ Write, stdout, };

use futures::{ select, FutureExt };
use async_std::{
    task,
    prelude::*,
    net::{ TcpStream, ToSocketAddrs },
};

const __NAME__: &str = "NULL chat";
const __VERSION__: &str = "v0.0.1";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    task::block_on(try_main("127.0.0.1:8080"))
}


async fn try_main(addr: impl ToSocketAddrs) -> Result<()> {



    let mut screen = AlternateScreen::from(stdout());
    let height = termion::terminal_size().unwrap().1;


    let stream = TcpStream::connect(addr).await?;
    let (reader, mut writer) = (&stream, &stream);

    let reader = async_std::io::BufReader::new(reader);
    let mut lines_from_server = futures::StreamExt::fuse(reader.lines());

    let _stdin = async_std::io::BufReader::new(async_std::io::stdin());
    let mut lines_from_stdin = futures::StreamExt::fuse(_stdin.lines());

    let mut out: u16 = 7;

    loop {

        info(&mut screen, "00000000", &["unimplemented!"], __NAME__, __VERSION__);
        input(&mut screen);

        screen.flush()?;

        select! {

            line = lines_from_server.next().fuse() => match line {

                Some(line) => {
                    let line = line?;
                    board(&mut screen, &line, (1, out));
                    out += 1;
                },

                None => break,

            },

            line = lines_from_stdin.next().fuse() => match line {

                Some(line) => {

                    info(&mut screen, "00000000", &["unimplemented!"], __NAME__, __VERSION__);
                    input(&mut screen);

                    let line = line?;

                    change_cursor_pos(&mut screen, (6, height - 6));// Go to input position
                    write!(&mut screen, "{}", termion::clear::AfterCursor).unwrap(); // Clear the input
                    // board(&mut screen, &line, (1, out));              // Rendering the chat board
                    change_cursor_pos(&mut screen, (6, height - 6));// Go to input position
                    screen.flush().unwrap();

                    writer.write_all(line.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                    // out += 1;
                },

                None => break,
            }

        }

    }

    Ok(())

}

fn input(mut screen: &mut impl Write) {

    let height = termion::terminal_size().unwrap().1;

    write_input_prompt(&mut screen, (1, height-6));
    write_sperate_line(&mut screen, (1, height-7));
    write_sperate_line(&mut screen, (1, height-8));

    change_cursor_pos(&mut screen, (6, height-6)); // Go to input position

}

fn write_input_prompt(mut screen: &mut impl Write, pos: (u16, u16)) {

    write!(&mut screen,
           "{}{}{}>>>: {}",
           style::Bold,
           termion::cursor::Goto(pos.0, pos.1),
           color::Fg(color::Red),
           style::Reset
    ).unwrap();

}

fn board(mut screen: &mut impl Write, content: &str, pos: (u16, u16)) {

    write!(&mut screen,
           "{}{}",
           termion::cursor::Goto(pos.0, pos.1),
           content
    ).unwrap();

}


fn info(mut screen: &mut impl Write, room_id: &str, members: &[&str], name: &str, version: &str) {

    // write name
    write!(&mut screen,
           "{}{}{} {}{}-{}{}",
           color::Fg(color::Blue),
           termion::cursor::Goto(termion::terminal_size().unwrap().0/2-((name.len()+version.len()+room_id.len()+1)/2) as u16, 2),
           name,
           version,
           color::Fg(color::Green),
           room_id,
           color::Fg(color::Reset)
    ).unwrap();

    write_sperate_line(&mut screen, (1,4)); // write board
    write!(&mut screen, "{}Group members: ({})", termion::cursor::Goto(1, 5), members.join(", ")).unwrap();
    write_sperate_line(&mut screen, (1,6));

}

fn write_sperate_line(mut screen: &mut impl Write, pos: (u16, u16)) {

    let mut sperate_line = String::new();

    for _ in 0..termion::terminal_size().unwrap().0 {
        sperate_line.push_str("-");
    }

    write!(&mut screen, "{}{}\n", termion::cursor::Goto(pos.0, pos.1), sperate_line).unwrap();

}

fn change_cursor_pos(mut screen: &mut impl Write, pos: (u16, u16)) {

    write!(&mut screen, "{}", termion::cursor::Goto(pos.0, pos.1)).unwrap();

}
