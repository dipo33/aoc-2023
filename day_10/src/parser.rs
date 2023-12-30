use nom::character::complete::{anychar, newline};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many1, many_till};

use parsers::ParseResult;

use crate::entity::{Landscape, LandscapeField, Pipe};

pub fn parse(input: &str) -> ParseResult<Landscape> {
    let (_, rows) = many1(field_row)(input)?;
    Ok(Landscape::new(rows))
}

fn field_row(input: &str) -> IResult<&str, Vec<LandscapeField>> {
    let (input, (row, _)) = many_till(field, newline)(input)?;
    Ok((input, row))
}

fn field(input: &str) -> IResult<&str, LandscapeField> {
    map(anychar, field_from_char)(input)
}

fn field_from_char(input: char) -> LandscapeField {
    match input {
        '.' => LandscapeField::Empty,
        'S' => LandscapeField::Start,
        '|' => LandscapeField::Pipe(Pipe::Vertical),
        '-' => LandscapeField::Pipe(Pipe::Horizontal),
        'L' => LandscapeField::Pipe(Pipe::CornerBottomLeft),
        'J' => LandscapeField::Pipe(Pipe::CornerBottomRight),
        '7' => LandscapeField::Pipe(Pipe::CornerTopRight),
        'F' => LandscapeField::Pipe(Pipe::CornerTopLeft),
        _ => panic!("Invalid field character: {}", input),
    }
}
