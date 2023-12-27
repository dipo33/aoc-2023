use nom::bytes::complete::tag;
use nom::character::complete::{newline, space1};
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;

use parsers::{digit, uint, ParseResult};

use crate::entity::{Bidder, Card, Hand};

pub fn parse(input: &str, enable_joker: bool) -> ParseResult<Vec<Bidder>> {
    let (_, bidders) = separated_list1(newline, bidder(enable_joker))(input)?;
    Ok(bidders)
}

fn bidder(enable_joker: bool) -> impl FnMut(&str) -> IResult<&str, Bidder> {
    move |input: &str| {
        let (input, hand) = hand(enable_joker)(input)?;
        let (input, _) = space1(input)?;
        let (input, bid) = uint::<u32>(input)?;
        Ok((input, Bidder { hand, bid }))
    }
}

fn hand(joker: bool) -> impl FnMut(&str) -> IResult<&str, Hand> {
    move |input: &str| {
        let (input, (c1, c2, c3, c4, c5))
            = tuple((card(joker), card(joker), card(joker), card(joker), card(joker)))(input)?;
        Ok((input, Hand::new([c1, c2, c3, c4, c5], joker)))
    }
}

fn card(enable_joker: bool) -> impl FnMut(&str) -> IResult<&str, Card> {
    move |input: &str| {
        let (input, card) = nom::branch::alt((
            map(tag("A"), |_| Card::Ace),
            map(tag("K"), |_| Card::King),
            map(tag("Q"), |_| Card::Queen),
            map(tag("J"), |_| if enable_joker { Card::Joker } else { Card::Jack }),
            map(tag("T"), |_| Card::Number(10)),
            map(digit, |digit| Card::Number(digit)),
        ))(input)?;

        Ok((input, card))
    }
}