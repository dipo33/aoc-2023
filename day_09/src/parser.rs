use nom::character::complete::{newline, space1};
use nom::IResult;
use nom::multi::separated_list1;

use parsers::{int, ParseResult};

pub fn parse(input: &str) -> ParseResult<Vec<Vec<i64>>> {
    let (_, rows) = separated_list1(newline, row)(input)?;
    Ok(rows)
}

pub fn row(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, int::<i64>)(input)
}