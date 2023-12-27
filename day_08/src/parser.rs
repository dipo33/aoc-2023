use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::char;
use nom::character::streaming::multispace1;
use nom::combinator::map;
use nom::IResult;
use nom::multi::{fold_many1, many1};
use nom::sequence::tuple;

use parsers::ParseResult;

use crate::entity::{Direction, Graph, LabeledMap, NodeId};

pub fn parse(input: &str) -> ParseResult<LabeledMap> {
    let (input, directions) = many1(direction)(input)?;
    let (input, _) = multispace1(input)?;

    let (_, graph) = fold_many1(
        cross_section,
        Graph::new,
        |mut graph: Graph, (source, (dest_a, dest_b))| {
            graph.add_edge(source, dest_a, dest_b);
            graph
        },
    )(input)?;

    Ok(LabeledMap { graph, directions })
}

fn direction(input: &str) -> IResult<&str, Direction> {
    alt((
        map(char('L'), |_| Direction::Left),
        map(char('R'), |_| Direction::Right),
    ))(input)
}

fn cross_section(input: &str) -> IResult<&str, (NodeId, (NodeId, NodeId))> {
    let (input, (source, _, dest_a, _, dest_b, _))
        = tuple((node_id, tag(" = ("), node_id, tag(", "), node_id, tag(")\n")))(input)?;

    Ok((input, (source, (dest_a, dest_b))))
}

fn node_id(input: &str) -> IResult<&str, NodeId> {
    map(take(3usize), Graph::label_to_node_id)(input)
}
