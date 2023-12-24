use nom::bytes::complete::tag;
use nom::character::complete::{newline, space1};
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;

use parsers::{digit, integer, ParseResult};

use crate::entity::{Bidder, Card, Hand};

pub fn parse(input: &str) -> ParseResult<Vec<Bidder>> {
    let (_, bidders) = separated_list1(newline, bidder)(input)?;
    Ok(bidders)
}

fn bidder(input: &str) -> IResult<&str, Bidder> {
    let (input, hand) = hand(input)?;
    let (input, _) = space1(input)?;
    let (input, bid) = integer::<u32>(input)?;
    Ok((input, Bidder { hand, bid }))
}

fn hand(input: &str) -> IResult<&str, Hand> {
    let (input, (c1, c2, c3, c4, c5))
        = tuple((card, card, card, card, card))(input)?;
    Ok((input, Hand::new([c1, c2, c3, c4, c5])))
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, card) = nom::branch::alt((
        map(tag("A"), |_| Card::Ace),
        map(tag("K"), |_| Card::King),
        map(tag("Q"), |_| Card::Queen),
        map(tag("J"), |_| Card::Jack),
        map(tag("T"), |_| Card::Number(10)),
        map(digit, |digit| Card::Number(digit)),
    ))(input)?;

    Ok((input, card))
}