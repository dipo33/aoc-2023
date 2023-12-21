use nom::branch::alt;
use nom::bytes::complete::{tag, take_until1};
use nom::character::complete::{digit1, none_of};
use nom::character::streaming::char;
use nom::combinator::{eof, map};
use nom::IResult;
use nom::multi::{many_till, separated_list1};

use crate::entity::{Blueprint, BlueprintItem, Item};

pub fn parse(input: &str) -> IResult<&str, Blueprint> {
    map(separated_list1(tag("\n"), row), Blueprint::new)(input)
}

fn row(input: &str) -> IResult<&str, Vec<BlueprintItem>> {
    let (input, line) = take_until1("\n")(input)?;
    let (_, (items, _)) = many_till(item, eof)(line)?;
    let mut v = Vec::with_capacity(line.len());
    for item in items {
        match item {
            BlueprintItem::Item(Item::PartLabel(label)) => {
                v.push(item);
                v.extend((1..digit_count(label)).map(|i| BlueprintItem::LeftRedirect(i as u8)));
            }
            item => v.push(item),
        }
    }

    Ok((input, v))
}

fn item(input: &str) -> IResult<&str, BlueprintItem> {
    alt((
        map(digit1, |x: &str| BlueprintItem::Item(Item::PartLabel(x.parse::<u32>().unwrap()))),
        map(char('.'), |_| BlueprintItem::Blank),
        map(none_of("\n"), |x| BlueprintItem::Item(Item::Symbol(x))),
    ))(input)
}

fn digit_count(num: u32) -> u32 {
    if num == 0 {
        return 1;
    }
    num.ilog10() + 1
}
