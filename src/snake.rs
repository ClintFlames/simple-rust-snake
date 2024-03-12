use crate::{Direction, Point};

pub struct Snake {
    pub score: u32,
    pub dir: Direction,
    pub body: Vec<Point>,
}

impl Snake {
    pub fn new(tsize: &Point) -> Self {
        Self {
            score: 0,
            dir: Direction::Right,
            body: {
                let center = Point(tsize.0 / 2, tsize.1 / 2);
                vec![
                    center,
                    Point(center.0 - 1, center.1),
                    Point(center.0 - 2, center.1),
                    Point(center.0 - 3, center.1),
                    Point(center.0 - 4, center.1),
                ]
            }
        }
    }

    pub fn set_dir(&mut self, ndir: Direction, tsize: &Point) {
        if self.next_pos(ndir, tsize) != self.body[1] {
            self.dir = ndir;
        }
    }

    pub fn next_pos(&self, dir: Direction, tsize: &Point) -> Point {
        match dir {
            Direction::Left => Point(
                match self.body[0].0.checked_sub(1) {
                    Some(v) => v,
                    None => tsize.0 - 1,
                },
                self.body[0].1
            ),
            Direction::Right => Point(
                if self.body[0].0 + 1 == tsize.0 {
                    0
                } else {
                    self.body[0].0 + 1
                },
                self.body[0].1
            ),
            Direction::Up => Point(
                self.body[0].0,
                match self.body[0].1.checked_sub(1) {
                    Some(v) => v,
                    None => tsize.1 - 1,
                }
            ),
            Direction::Down => Point(
                self.body[0].0,
                if self.body[0].1 + 1 == tsize.1 {
                    0
                } else {
                    self.body[0].1 + 1
                }
            )
        }
    }

    pub fn step(&mut self, tsize: &Point, hungry: bool) {
        self.body.insert(0, self.next_pos(self.dir, tsize));
        if hungry {
            self.body.pop();
        }
    }
}
