use std::io;
use crate::{Game, Point, MIN_TICK_SPEED, START_TICK_SPEED};



impl Game {
    // Here goes all the logic (except input events they are in handle_tevents)
    pub fn tick(&mut self) -> io::Result<()> {
        // If game over or paused doing nothing
        if self.paused || self.gameover { return Ok(()); }

        // Getting next head position
        let npos = self.snake.next_pos(self.snake.dir, &self.tsize);

        // Check if there is wall
        if self.map.cell(&npos)? {
            self.gameover = true;
            return Ok(());
        }

        // And also check if we bite our tail
        for i in 0..self.snake.body.len() - 1 {
            if self.snake.body[i] == npos {
                self.gameover = true;
                return Ok(());
            }
        }

        // Checking if there food
        let hungry = {
            let mut i = 0;
            loop {
                if i < self.food.len() {
                    if self.food[i] == npos {
                        self.food.remove(i);
                        self.food_eaten += 1;
                        break false;
                    }
                    i += 1;
                } else {
                    break true;
                }
            }
        };

        // If we ate food giving us points
        if !hungry {
            let p = (START_TICK_SPEED - self.tick_speed + 10) / 10;

            // And also speed up in some cases
            if self.food_eaten as u128 % (p * 5) == 0 && self.tick_speed - 10 >= MIN_TICK_SPEED {
                self.tick_speed -= 10;
            }

            self.snake.score += p as u32;
        }

        // Making snake move
        self.snake.step(&self.tsize, hungry);

        // Generating food every 5 tick and if lower than food cap
        if self.tick_count % 5 == 0 && self.food.len() < self.food_cap {
            // Works till not find empty position
            'za: loop { // a
                let food_pos = Point(
                    fastrand::u16(0..self.tsize.0),
                    fastrand::u16(0..self.tsize.1));

                if self.map.cell(&food_pos)? {
                    continue 'za;
                }

                for el in &self.snake.body {
                    if *el == food_pos {
                        continue 'za;
                    }
                }

                self.food.push(food_pos);
                break;
            }
        }


        self.tick_count += 1;

        Ok(())
    }
}