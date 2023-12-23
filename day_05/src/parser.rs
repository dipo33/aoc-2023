use nom::bytes::complete::tag;
use nom::character::complete::{anychar, newline, space1};
use nom::IResult;
use nom::multi::{many_till, separated_list1};
use nom::sequence::tuple;

use parsers::{fold_separated_list1, integer};

use crate::entity::{Almanac, IntervalTree};

pub fn parse(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = seeds(input)?;
    let (input, _) = newline(input)?;
    let (input, interval_trees) = separated_list1(newline, interval_tree)(input)?;

    Ok((input, Almanac { seeds, interval_trees }))
}

fn seeds(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("seeds: ")(input)?;
    separated_list1(space1, integer::<u32>)(input)
}

fn interval_tree(input: &str) -> IResult<&str, IntervalTree> {
    let (input, _) = newline(input)?;
    let (input, _) = many_till(anychar, newline)(input)?;

    fold_separated_list1(newline, interval, IntervalTree::new, insert_interval)(input)
}

fn interval(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (input, (destination, _, source, _, length)) =
        tuple((integer::<u32>, space1, integer::<u32>, space1, integer::<u32>))(input)?;

    Ok((input, (destination, source, length)))
}


fn insert_interval(mut tree: IntervalTree, (dest, source, length): (u32, u32, u32)) -> IntervalTree {
    tree.insert(source, source + (length - 1), dest as i64 - source as i64);
    tree
}
