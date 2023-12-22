use nom::IResult;
use nom::character::complete::digit1;

use crate::entity::{Blueprint, BlueprintItem};

pub fn parse(input: &str) -> IResult<&str, Blueprint> {
    let mut grid: Vec<BlueprintItem> = Vec::with_capacity(input.len());
    let mut idx = 0;
    let mut row_count = 0;

    for mut line in input.split("\n") {
        row_count += 1;
        while !line.is_empty() {
            let c = line.chars().next().unwrap();
            if c.is_digit(10) {
                let (_line, digit) = digit1(line)?;

                let label = digit.parse::<u32>().unwrap();
                grid.push(BlueprintItem::PartLabel(label, idx));
                grid.extend((1..digit.len() as u32).map(|_| BlueprintItem::PartLabel(label, idx)));

                idx += digit.len() as u32;
                line = _line;
            } else {
                if c == '.' {
                    grid.push(BlueprintItem::Blank);
                } else {
                    grid.push(BlueprintItem::Symbol(c));
                }
                idx += 1;
                line = &line[1..];
            }
        }
    }

    Ok(("", Blueprint::new(grid, idx / (row_count - 1))))
}