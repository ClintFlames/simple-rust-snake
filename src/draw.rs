use std::io::{self, Stdout, Write};
use crossterm::{cursor, style::{self, Color, Stylize}, terminal::{self, ClearType}, QueueableCommand};
use crate::{Game, Point, MIN_TICK_SPEED};



pub static SNAKE_HEAD_SYMDOL: char = '@';
pub static SNAKE_BODY_SYMDOL: char = '0';
pub static OFFSET: u16 = 3;
pub static WALL_SYMBOL: char = '#';
pub static FOOD_SYMBOL: char = '%';



// TODO make walls static
impl Game {
    pub fn draw(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        stdout.queue(terminal::Clear(ClearType::All))?;

        // Calling other draw functions here
        stdout.queue(style::ResetColor)?;
        self.draw_player(&mut stdout)?;

        stdout.queue(style::ResetColor)?;
        self.draw_food(&mut stdout)?;

        stdout.queue(style::ResetColor)?;
        self.draw_wall(&mut stdout)?;

        stdout.queue(style::ResetColor)?;
        self.draw_ui(&mut stdout)?;

        stdout.flush()?;

        Ok(())
    }

    fn draw_player(&self, stdout: &mut Stdout) -> io::Result<()> {
        stdout.queue(cursor::MoveTo(self.snake.body[0].0, self.snake.body[0].1 + OFFSET))?;
        print!("{}", SNAKE_HEAD_SYMDOL);
        for pos in &self.snake.body[1..] {
            stdout.queue(cursor::MoveTo(pos.0, pos.1 + OFFSET))?;
            print!("{}", SNAKE_BODY_SYMDOL);
        }

        Ok(())
    }

    fn draw_food(&self, stdout: &mut Stdout) -> io::Result<()> {
        stdout.queue(style::SetForegroundColor(Color::Red))?;

        for pos in &self.food {
            stdout.queue(cursor::MoveTo(pos.0, pos.1 + OFFSET))?;
            print!("{}", FOOD_SYMBOL);
        }

        Ok(())
    }

    fn draw_wall(&self, stdout: &mut Stdout) -> io::Result<()> {
        for x in 0..self.tsize.0 {
            for y in 0..self.tsize.1 {
                if self.map.cell(&Point(x, y))? {
                    stdout.queue(cursor::MoveTo(x, y + OFFSET))?;
                    print!("{}", WALL_SYMBOL);
                }
            }
        }
        
        Ok(())
    }

    // This is most messy function
    // TODO make it cleaner
    fn draw_ui(&self, stdout: &mut Stdout) -> io::Result<()> {
        Self::draw_frame(stdout, &Point(0, 0), &Point(self.tsize.0, 3))?;

        let score_text = format!("Score: {}", self.snake.score);
        let w = (self.tsize.0 - 2) / 3;

        stdout.queue(cursor::MoveTo(w - score_text.len() as u16, 1))?;
        print!("{}", score_text);

        stdout.queue(cursor::MoveTo(w * 2, 1))?;

        if self.tick_speed == MIN_TICK_SPEED {
            print!("Tick speed: ");
            stdout.queue(style::PrintStyledContent(self.tick_speed.to_string().blue()))?;
        } else {
            print!("Tick speed: {}", self.tick_speed);
        }

        if self.paused {

            let pause_text: Vec<&str> = 
"Game Paused!
Press Enter to continue.
Press R to restart.
Press Escape to exit."
            .split("\n").collect();
            let pause_len: u16 = {
                let mut len = 0;
                for s in &pause_text {
                    if len < s.len() {
                        len = s.len();
                    }
                }
                
                len as u16
            };
            
            let pause_pos = Point(
                (self.tsize.0 - pause_len + 4) / 2,
                (self.tsize.1 - pause_text.len() as u16 + 2) / 2
            );
            
            Self::draw_frame(
                stdout,
                &pause_pos,
                &Point(pause_len + 4, pause_text.len() as u16 + 2)
            )?;
            
            for i in 0..pause_text.len() as u16 {
                let ctext = pause_text[i as usize];
                stdout.queue(cursor::MoveTo(pause_pos.0 + 2, pause_pos.1 + 1 + i))?;
                print!("{}", " ".repeat((pause_len as usize - ctext.len()) / 2) + ctext);
            }
        }

        if self.gameover {

            let go_text = format!(
"Game Over!
Total score: {}
Total food eaten: {}
Total length: {}
Press Enter to restart.
Press Escape to exit.",
            self.snake.score, self.food_eaten, self.snake.body.len());
            let go_text: Vec<&str> = go_text.split("\n").collect();
            let go_len: u16 = {
                let mut len = 0;
                for s in &go_text {
                    if len < s.len() {
                        len = s.len();
                    }
                }
                
                len as u16
            };
            
            let go_pos = Point(
                (self.tsize.0 - go_len + 4) / 2,
                (self.tsize.1 - go_text.len() as u16 + 2) / 2
            );
            
            Self::draw_frame(
                stdout,
                &go_pos,
                &Point(go_len + 4, go_text.len() as u16 + 2)
            )?;
            
            for i in 0..go_text.len() as u16 {
                let ctext = go_text[i as usize];
                stdout.queue(cursor::MoveTo(go_pos.0 + 2, go_pos.1 + 1 + i))?;
                print!("{}", " ".repeat((go_len as usize - ctext.len()) / 2) + ctext);
            }
        }

        Ok(())
    }

    fn draw_frame(stdout: &mut Stdout, pos: &Point, size: &Point) -> io::Result<()> {
        let endpos = Point(pos.0 + size.0 - 1, pos.1 + size.1 - 1);
        stdout.queue(cursor::MoveTo(pos.0, pos.1))?;
        print!("/{:-<1$}\\", "", size.0 as usize - 2);

        for y in pos.1 + 1..=endpos.1 {
            stdout.queue(cursor::MoveTo(pos.0, y))?;
            print!("|");
    
            stdout.queue(cursor::MoveTo(endpos.0, y))?;
            print!("|");
        }

        stdout.queue(cursor::MoveTo(pos.0, endpos.1))?;
        print!("\\{:-<1$}/", "", size.0 as usize - 2);

        Ok(())
    }
}