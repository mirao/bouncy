use std::env::args;

#[derive(Debug)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub enum ParseError {
    TooFewArgs,
    TooManyArgs,
    InvalidInteger(String),
    TooSmallFrame(u32, u32),
}

struct ParseArgs(std::env::Args);
impl ParseArgs {
    fn new() -> Self {
        Self(args())
    }

    fn require_arg(&mut self) -> Result<String, ParseError> {
        match self.0.next() {
            None => Err(ParseError::TooFewArgs),
            Some(s) => Ok(s),
        }
    }

    fn require_no_args(&mut self) -> Result<(), ParseError> {
        match self.0.next() {
            Some(_) => Err(ParseError::TooManyArgs),
            // I think this looks a little weird myself.
            // But we're wrapping up the unit value ()
            // with the Ok variant. You get used to it
            // after a while, I guess
            None => Ok(()),
        }
    }
}

fn parse_u32(s: String) -> Result<u32, ParseError> {
    match s.parse() {
        Err(_) => Err(ParseError::InvalidInteger(s)),
        Ok(x) => Ok(x),
    }
}

pub fn parse_args() -> Result<Frame, ParseError> {
    let mut args = ParseArgs::new();
    // skip the command name
    args.require_arg()?;
    let width_str = args.require_arg()?;
    let height_str = args.require_arg()?;
    args.require_no_args()?;
    let width = parse_u32(width_str)?;
    let height = parse_u32(height_str)?;
    if width > 1 && height > 1 {
        Ok(Frame { width, height })
    } else {
        Err(ParseError::TooSmallFrame(width, height))
    }
}
