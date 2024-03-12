use std::io;

use crate::Point;

pub struct Map {
    size: Point,
    data: Vec<Vec<bool>>,
}

impl Map {
    pub fn new(tsize: &Point) -> Self {
        Self {
            size: tsize.clone(),
            data: {
                let mut d = Vec::new();

                for x in 0..tsize.0 as usize {
                    d.push(Vec::new());
                    for _ in 0..tsize.1 as usize {
                        d[x].push(false);
                    }
                }

                d
            }
        }
    }

    pub fn cell(&self, point: &Point) -> io::Result<bool> {
        if point.0 >= self.size.0 || point.1 >= self.size.1 {
            return Err(io::Error::new(io::ErrorKind::Other, "No such element, out of bound"));
        }

        Ok(self.data[point.0 as usize][point.1 as usize])
    }

    pub fn set_cell(&mut self, point: &Point, value: bool) -> io::Result<()> {
        if point.0 >= self.size.0 || point.1 >= self.size.1 {
            return Err(io::Error::new(io::ErrorKind::Other, "No such element, out of bound"));
        }

        self.data[point.0 as usize][point.1 as usize] = value;

        Ok(())
    }
}