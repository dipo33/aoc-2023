use std::str::FromStr;

use nom::{error, InputLength, IResult, Parser};
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::error::{ErrorKind, ParseError};

/// Holds the result of parsing functions
///
/// It depends on the output type `O`
///
/// The `Ok` side contains the produced value. The `Err` side contains an instance of `nom::Err`.
pub type ParseResult<'a, O> = Result<O, nom::Err<error::Error<&'a str>>>;

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

pub fn fold_separated_list1<I, O, O2, E, F, G, H, S, R>(
    mut sep: S,
    mut f: F,
    mut init: H,
    mut g: G,
) -> impl FnMut(I) -> IResult<I, R, E>
    where
        I: Clone + InputLength,
        F: Parser<I, O, E>,
        S: Parser<I, O2, E>,
        G: FnMut(R, O) -> R,
        H: FnMut() -> R,
        E: ParseError<I>,
{
    move |mut i: I| {
        let mut acc = init();

        // Parse the first element
        match f.parse(i.clone()) {
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                acc = g(acc, o);
                i = i1;
            }
        }

        loop {
            let len = i.input_len();
            match sep.parse(i.clone()) {
                Err(nom::Err::Error(_)) => return Ok((i, acc)),
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    // infinite loop check: the parser must always consume
                    if i1.input_len() == len {
                        return Err(nom::Err::Error(E::from_error_kind(i1, ErrorKind::SeparatedList)));
                    }

                    match f.parse(i1.clone()) {
                        Err(nom::Err::Error(_)) => return Ok((i, acc)),
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            acc = g(acc, o);
                            i = i2;
                        }
                    }
                }
            }
        }
    }
}
