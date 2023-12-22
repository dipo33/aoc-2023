use std::str::FromStr;

use nom::character::complete::digit1;
use nom::combinator::map;
use nom::IResult;

/// Parses an integer value from the given input string.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Returns
///
/// Returns a `Result` indicating the success or failure of the parsing operation.
/// If the input string is successfully parsed into an integer value, the result is a tuple containing the remaining input string and the parsed integer
pub fn integer<T>(input: &str) -> IResult<&str, T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
{
    map(digit1, |s: &str| s.parse::<T>().unwrap())(input)
}
