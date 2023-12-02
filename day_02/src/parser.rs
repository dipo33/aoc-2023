use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::IResult;
use nom::multi::separated_list1;

use crate::entity::{Color, Cube, Game, Play};

pub fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(tag("\n"), game)(input)
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = _u8(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, plays) = separated_list1(tag("; "), play)(input)?;

    Ok((input, Game { id, plays }))
}

fn play(input: &str) -> IResult<&str, Play> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;

    Ok((input, Play { cubes }))
}

fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, amount) = _u8(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = color(input)?;

    Ok((input, Cube { color, amount }))
}

fn color(input: &str) -> IResult<&str, Color> {
    alt((
        map(tag("red"), |_| Color::RED),
        map(tag("green"), |_| Color::GREEN),
        map(tag("blue"), |_| Color::BLUE),
    ))(input)
}

fn _u8(input: &str) -> IResult<&str, u8> {
    map_res(digit1, |s: &str| s.parse::<u8>())(input)
}
