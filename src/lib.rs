use std::{io::{self, Write}, process};
use crossterm::{cursor, event::{Event, KeyCode}, style, terminal, QueueableCommand};
use draw::OFFSET;

// Importing tick and draw fn for Game
mod tick;
mod draw;

// Importing Map and Snake
mod map;
mod snake;

pub use map::Map;
pub use snake::Snake;


// Base types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point(pub u16, pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction { Left, Right, Up, Down, }



pub static START_TICK_SPEED: u128 = 250;
pub static MIN_TICK_SPEED: u128 = 70;
pub static FOOD_FACTOR: usize = 500;


// Game struct to make things easier
pub struct Game {
    pub tsize: Point,
    pub tcenter: Point,

    pub gameover: bool,
    pub paused: bool,
    pub tick_speed: u128,
    pub tick_count: usize,

    pub snake: Snake,

    pub map: Map,
    pub food: Vec<Point>,
    pub food_eaten: u32,
    pub food_cap: usize
}

impl Game {
    pub fn new(terminal_size: Point) -> io::Result<Self> {
        let tsize = Point(terminal_size.0, terminal_size.1 - OFFSET);
        let mut map = Map::new(&tsize);

        // Generate map
        // TODO add more types of map
        match fastrand::u8(0..4) {
            0 => {
                for x in 0..tsize.0 {
                    for y in 0..tsize.1 {
                        if fastrand::u8(0..100) == 0 {
                            map.set_cell(&Point(x, y), true)?;
                        }
                    }
                }
            },
            1 => {
                for x in 0..tsize.0 {
                    map.set_cell(&Point(x, 0), true)?;
                    map.set_cell(&Point(x, tsize.1 - 1), true)?;
                }
            },
            2 => {
                for y in 0..tsize.1 {
                    map.set_cell(&Point(0, y), true)?;
                    map.set_cell(&Point(tsize.0 - 1, y), true)?;
                }
            }
            _ => ()
        }

        Ok(Self {
            tsize,
            tcenter: Point(tsize.0, tsize.1),
            gameover: false,
            paused: false,
            tick_speed: START_TICK_SPEED,
            tick_count: 0,
            snake: Snake::new(&tsize),
            map,
            food: Vec::new(),
            food_eaten: 0,
            food_cap: ((tsize.0 as usize * tsize.1 as usize) / FOOD_FACTOR)
        })
    }

    pub fn handle_tevent(&mut self, tevent: Event) -> io::Result<()> {
        match tevent {
            Event::Key(key_event) => {
                // Vim movement supported too!
                match key_event.code {
                    KeyCode::Left |
                    KeyCode::Char('a') | KeyCode::Char('A') |
                    KeyCode::Char('h') | KeyCode::Char('H') => {
                        self.snake.set_dir(Direction::Left, &self.tsize);
                    },
                    KeyCode::Right |
                    KeyCode::Char('d') | KeyCode::Char('D') |
                    KeyCode::Char('l') | KeyCode::Char('L') => {
                        self.snake.set_dir(Direction::Right, &self.tsize);
                    },
                    KeyCode::Up |
                    KeyCode::Char('w') | KeyCode::Char('W') |
                    KeyCode::Char('k') | KeyCode::Char('K') => {
                        self.snake.set_dir(Direction::Up, &self.tsize);
                    },
                    KeyCode::Down |
                    KeyCode::Char('s') | KeyCode::Char('S') |
                    KeyCode::Char('j') | KeyCode::Char('J') => {
                        self.snake.set_dir(Direction::Down, &self.tsize);
                    },
                    KeyCode::Esc => {
                        if self.gameover {
                            self.exit()?;
                        }
                        if self.paused {
                            self.exit()?;
                        } else {
                            self.paused = true;
                        }
                    },
                    KeyCode::Enter => {
                        if self.gameover {
                            return Err(io::Error::new(io::ErrorKind::Other, "restart"));
                        }
                        if self.paused {
                            self.paused = false;
                        }
                    }
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        if self.paused {
                            return Err(io::Error::new(io::ErrorKind::Other, "restart"));
                        }
                    }
                    _ => ()
                }
            },
            _ => ()
        }

        Ok(())
    }

    pub fn exit(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        // Reset terminal and close game
        stdout.queue(style::ResetColor)?;
        stdout.queue(cursor::Show)?;
        stdout.flush()?;
        terminal::disable_raw_mode()?;

        process::exit(0);
    }
}
