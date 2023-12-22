use nom::bytes::complete::tag;
use nom::character::complete::{char, space1};
use nom::IResult;
use nom::multi::separated_list1;

use parsers::integer;

use crate::entity::Card;

pub fn parse(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(char('\n'), card)(input)
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space1(input)?;
    let (input, id) = integer::<u32>(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = space1(input)?;
    let (input, winning) = separated_list1(space1, integer::<u8>)(input)?;
    let (input, _) = tag(" |")(input)?;
    let (input, _) = space1(input)?;
    let (input, guesses) = separated_list1(space1, integer::<u8>)(input)?;

    Ok((input, Card { id, winning, guesses }))
}