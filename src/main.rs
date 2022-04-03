mod parse_args;

use parse_args::Frame;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum VertDir {
    Up,
    Down,
}

#[derive(Debug)]
enum HorizDir {
    Left,
    Right,
}

#[derive(Debug)]
struct Ball {
    x: u32,
    y: u32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

impl Ball {
    /// Change ball direction when ball bounces in given frame
    fn bounce(&mut self, frame: &Frame) {
        if self.x == 0 {
            self.horiz_dir = HorizDir::Right
        } else if self.x == frame.width - 1 {
            self.horiz_dir = HorizDir::Left
        }

        if self.y == 0 {
            self.vert_dir = VertDir::Down
        } else if self.y == frame.height - 1 {
            self.vert_dir = VertDir::Up
        }
    }

    /// Move ball
    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        }

        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
    }
}

#[derive(Debug)]
struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new(frame: Frame) -> Game {
        Game {
            frame,
            ball: Ball {
                x: 2,
                y: 4,
                vert_dir: VertDir::Up,
                horiz_dir: HorizDir::Left,
            },
        }
    }

    /// Set a ball's direction and move the ball
    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let top_bottom = |fmt: &mut Formatter| {
            write!(fmt, "+")?;
            for _ in 1..=self.frame.width {
                write!(fmt, "-")?;
            }
            writeln!(fmt, "+")
        };

        top_bottom(fmt)?;
        for row in 0..self.frame.height {
            write!(fmt, "|")?;
            for column in 0..self.frame.width {
                let c = if row == self.ball.y && column == self.ball.x {
                    'o'
                } else {
                    ' '
                };
                write!(fmt, "{}", c)?;
            }
            writeln!(fmt, "|")?;
        }
        top_bottom(fmt)
    }
}

fn main() -> Result<(), parse_args::ParseError> {
    let frame = parse_args::parse_args()?;

    let mut game = Game::new(frame);
    let sleep_duration = std::time::Duration::from_millis(33);
    loop {
        println!("{}", game);
        // Clear screen
        print!("{}[2J", 27 as char);
        game.step();
        std::thread::sleep(sleep_duration);
    }
}
