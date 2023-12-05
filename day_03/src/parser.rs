use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, none_of};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many1, separated_list1};

use crate::entity::{Schematic, SchematicItem};

pub fn parse(input: &str) -> IResult<&str, Schematic> {
    let (input, grid) = separated_list1(tag("\n"), row)(input)?;
    Ok((input, Schematic { grid }))
}

fn row(input: &str) -> IResult<&str, Vec<SchematicItem>> {
    let (input, items) = many1(item)(input)?;

    Ok((input, items.into_iter().flatten().collect()))
}

fn item(input: &str) -> IResult<&str, Vec<SchematicItem>> {
    alt((
        map(char('.'), |_| vec![SchematicItem::Blank]),
        map(digit1, |num: &str| {
            let part_number = num.to_string().parse::<u32>().unwrap();

            num.char_indices()
                .map(|(i, _)| {
                    SchematicItem::PartLabel {
                        part_number,
                        left: i,
                        right: num.len() - i - 1,
                    }
                })
                .collect()
        }),
        map(none_of("\n"), |c| vec![SchematicItem::Symbol(c)]),
    ))(input)
}
