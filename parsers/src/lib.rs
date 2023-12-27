use std::ops::Neg;
use std::str::FromStr;

use nom::{error, InputLength, IResult, Parser};
use nom::character::complete;
use nom::combinator;
use nom::error::{ErrorKind, ParseError};

/// Holds the result of parsing functions
///
/// It depends on the output type `O`
///
/// The `Ok` side contains the produced value. The `Err` side contains an instance of `nom::Err`.
pub type ParseResult<'a, O> = Result<O, nom::Err<error::Error<&'a str>>>;

/// Parses an unsigned integer from the given input string.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Returns
///
/// Returns a `Result` indicating the success or failure of the parsing operation.
/// If the input string is successfully parsed into an integer value, the result is a tuple containing the remaining input string and the parsed integer
pub fn uint<T>(input: &str) -> IResult<&str, T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
{
    combinator::map(complete::digit1, |s: &str| s.parse::<T>().unwrap())(input)
}

/// Parses a signed integer from the given input string.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Returns
///
/// Returns a `Result` indicating the success or failure of the parsing operation.
/// If the input string is successfully parsed into an integer value, the result is a tuple containing the remaining input string and the parsed integer
pub fn int<T>(input: &str) -> IResult<&str, T>
    where
        T: Neg<Output=T> + From<usize>,
{
    let (input, sign) = combinator::opt(complete::char('-'))(input)?;
    let (input, num) = uint::<usize>(input)?;
    let num: T = if sign.is_some() { -T::from(num) } else { T::from(num) };

    Ok((input, num))
}

pub fn digit_char(input: &str) -> IResult<&str, char>
{
    complete::satisfy(|c| c.is_ascii_digit())(input)
}

pub fn digit(input: &str) -> IResult<&str, u8>
{
    combinator::map(digit_char, |c: char| c.to_digit(10).unwrap() as u8)(input)
}

/// Alternates between two parsers to produce a list of elements until [`nom::Err::Error`], calling `g` to gather the results.
///
/// Fails if the element parser does not produce at least one element.
///
/// This stops when either parser returns [`nom::Err::Error`]  and returns the results that were accumulated. To instead chain an error up, see
/// [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `sep` Parses the separator between list elements.
/// * `f` Parses the elements of the collection.
/// * `init` A function returning the initial value.
/// * `g` The function that combines a result of `f` with
///       the current accumulator.
/// ```rust
/// use std::collections::HashSet;
/// use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::bytes::complete::tag;
/// use parsers::fold_separated_list1;
///
/// fn parser(s: &str) -> IResult<&str, HashSet<&str>> {
///   fold_separated_list1(tag("|"), tag("abc"), HashSet::new, |mut acc, val| {
///     acc.insert(val);
///     acc
///   })(s)
/// }
///
/// assert_eq!(parser("abc|abc|abc"), Ok(("", HashSet::from(["abc"]))));
/// assert_eq!(parser("abc123abc"), Ok(("123abc", HashSet::from(["abc"]))));
/// assert_eq!(parser("abc|def"), Ok(("|def", HashSet::from(["abc"]))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
/// assert_eq!(parser("def|abc"), Err(Err::Error(Error::new("def|abc", ErrorKind::Tag))));
/// ```
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
