use nom::bytes::complete::take_while1;
use nom::character::complete::{newline, space1};
use nom::multi::separated_list1;

use parsers::{integer, ParseResult};

use crate::entity::Race;

pub fn parse(input: &str) -> ParseResult<Vec<Race>> {
    let (input, _) = take_while1(|c: char| !c.is_ascii_digit())(input)?;
    let (input, times) = separated_list1(space1, integer::<u64>)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = take_while1(|c: char| !c.is_ascii_digit())(input)?;
    let (_, records) = separated_list1(space1, integer::<u64>)(input)?;

    let races = times.into_iter()
        .zip(records)
        .map(|(time, record)| Race { time, record })
        .collect();

    Ok(races)
}
