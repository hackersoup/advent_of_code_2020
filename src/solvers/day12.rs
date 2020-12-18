use std::io::{BufReader, BufRead};
use std::fs::File;

pub struct Ship {
    x: isize,
    y: isize,
    /// Degrees clockwise-off-north the ship points
    heading_degrees: usize,
}

impl Ship {
    fn go_north(&mut self, distance: usize) {
        self.y += distance as isize;
    }
    
    fn go_south(&mut self, distance: usize) {
        self.y -= distance as isize;
    }

    fn go_east(&mut self, distance: usize) {
        self.x += distance as isize;
    }

    fn go_west(&mut self, distance: usize) {
        self.x -= distance as isize;
    }

    fn turn_right(&mut self, degrees: usize) {
        self.heading_degrees = (self.heading_degrees + degrees) % 360;
    }

    fn turn_left(&mut self, degrees: usize) {
        self.heading_degrees = match degrees > self.heading_degrees {
            true => 360 - (degrees - self.heading_degrees),
            false => self.heading_degrees - degrees
        }
    }

    fn go_forward(&mut self, distance: usize) {
        let heading_rads = self.heading_degrees as f64 * (3.14 / 180.0);
        self.x += (heading_rads.sin() * (distance as f64)).round() as isize;
        self.y += (heading_rads.cos() * (distance as f64)).round() as isize;
    }

    fn manhattan_distance(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize
    }
}

pub fn solve(input: BufReader<File>) {
    let lines = input.lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();
    
        // Initial ship at origin pointing due East
    let mut ship = Ship {
        x: 0,
        y: 0,
        heading_degrees: 90,
    };

    lines.iter()
        .map(|line| line.chars())
        .for_each(|mut c| {
            let action = c.next().unwrap();
            let val = c.collect::<String>().parse::<usize>().unwrap();
            match action {
                'N' => ship.go_north(val),
                'E' => ship.go_east(val),
                'S' => ship.go_south(val),
                'W' => ship.go_west(val),
                'F' => ship.go_forward(val),
                'L' => ship.turn_left(val),
                'R' => ship.turn_right(val),
                _ => panic!()
            }
        });

    let answer_1 = ship.manhattan_distance();

    println!("[+] Day12-1: {}", answer_1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ship() {
        let mut ship = Ship {
            x: 0,
            y: 0,
            heading_degrees: 90
        };

        ship.go_forward(10);
        assert_eq!((ship.x, ship.y, ship.heading_degrees), (10, 0, 90));

        ship.go_north(3);
        assert_eq!((ship.x, ship.y, ship.heading_degrees), (10, 3, 90));

        ship.go_forward(7);
        assert_eq!((ship.x, ship.y, ship.heading_degrees), (17, 3, 90));

        ship.turn_right(90);
        assert_eq!((ship.x, ship.y, ship.heading_degrees), (17, 3, 180));

        ship.go_forward(11);
        assert_eq!((ship.x, ship.y, ship.heading_degrees), (17, -8, 180));

        ship.turn_left(270);
        assert_eq!((ship.x, ship.y, ship.heading_degrees), (17, -8, 270));

        ship.turn_left(90);
        assert_eq!((ship.x, ship.y, ship.heading_degrees), (17, -8, 180));
    }

}