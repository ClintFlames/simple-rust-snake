use std::{io::{self, Write}, time::{Duration, Instant}};
use crossterm::{cursor, event, terminal, ExecutableCommand, QueueableCommand};
use simple_rust_snake::{Game, Point};



fn main() -> io::Result<()> {
    // Preparing terminal
    let mut stdout = io::stdout();
    stdout.execute(cursor::Hide)?;
    terminal::enable_raw_mode()?;

    loop {
        // Error is used to restart game
        // Otherwise if error unknown we just stop game
        if let Err(err) = gameloop() {
            if err.to_string() == "restart" {
                continue;
            } else {
                break;
            }
        }
    }

    Ok(())
}

fn gameloop() -> io::Result<()> {
    let size = terminal::size()?;
    let mut time = Instant::now();
    let mut stdout = io::stdout();
    let mut game = Game::new(Point(size.0, size.1))?;
    
    loop {
        // Check if size changed
        let nsize = terminal::size()?;
        if size != nsize {
            let msg = format!(
                "Terminal size must be ({}, {}). Current is ({}, {}).",
                size.0, size.1,
                nsize.0, nsize.1
            );

            stdout.queue(cursor::MoveTo(nsize.0 / 2 - msg.len() as u16 / 2, nsize.1 / 2))?;
            print!("{}", msg);
            stdout.flush()?;

            continue;
        }

        // Handling events (keyboard)
        if event::poll(Duration::from_millis(10))? {
            game.handle_tevent(event::read()?)?;
        }

        // If time elapsed we run game and reset time
        // Otherwise just continue
        if time.elapsed().as_millis() < game.tick_speed {
            continue;
        }

        time = Instant::now();
        game.tick()?;
        game.draw()?;
    }
}