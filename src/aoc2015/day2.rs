use crate::utils::AocResult;
use std::str::FromStr;
use thiserror::Error;

/*
--- Day 2: I Was Told There Would Be No Math ---

The elves are running low on wrapping paper, and so they need to submit an
order for more. They have a list of the dimensions (length l, width w, and
height h) of each present, and only want to order exactly as much as they need.

Fortunately, every present is a box (a perfect right rectangular prism), which
makes calculating the required wrapping paper for each gift a little easier:
find the surface area of the box, which is 2*l*w + 2*w*h + 2*h*l. The elves
also need a little extra paper for each present: the area of the smallest side.

For example:

    A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet
    of wrapping paper plus 6 square feet of slack, for a total of 58 square
    feet.

    A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square
    feet of wrapping paper plus 1 square foot of slack, for a total of 43
    square feet.

All numbers in the elves' list are in feet. How many total square feet of
wrapping paper should they order?
*/
pub fn part1(input: &str) -> AocResult {
    input
        .lines()
        .map(|line| line.parse::<Dimensions>().map(wrapping_paper))
        .sum::<Result<i64, ParseDimensionError>>()
        .map_err(|e| e.into())
}

/*
--- Part Two ---
The elves are also running low on ribbon. Ribbon is all the same width, so they
only have to worry about the length they need to order, which they would again
like to be exact.

The ribbon required to wrap a present is the shortest distance around its
sides, or the smallest perimeter of any one face. Each present also requires a
bow made out of ribbon as well; the feet of ribbon required for the perfect bow
is equal to the cubic feet of volume of the present. Don't ask how they tie the
bow, though; they'll never tell.

For example:

    A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to
    wrap the present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of
    34 feet.

    A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to
    wrap the present plus 1*1*10 = 10 feet of ribbon for the bow, for a total
    of 14 feet.

How many total feet of ribbon should they order?
*/
pub fn part2(input: &str) -> AocResult {
    // Iterate through lines and convert to dimensions, filtering out any
    // invalid ones. Map the result to the ribbon function and then
    // return the sum.
    input
        .lines()
        .map(|line| line.parse::<Dimensions>().map(ribbon))
        .sum::<Result<i64, ParseDimensionError>>()
        .map_err(|e| e.into())
}

// Accept dimensions and return the total wrapping paper required.
fn wrapping_paper(dimensions: Dimensions) -> i64 {
    let Dimensions {
        length,
        width,
        height,
    } = dimensions;
    let areas = [length * width, width * height, length * height];
    let min = *areas.iter().min().expect("array has three elements");
    let area: i64 = areas.iter().sum();
    2 * area + min
}

// Accept dimensions and return the total ribbon needed.
fn ribbon(dimensions: Dimensions) -> i64 {
    let Dimensions {
        length,
        width,
        height,
    } = dimensions;
    let half_perims = [length + width, width + height, length + height];
    let min = *half_perims.iter().min().expect("array has three elements");
    let vol = length * width * height;
    2 * min + vol
}

struct Dimensions {
    length: i64,
    width: i64,
    height: i64,
}

impl Dimensions {
    fn new(length: i64, width: i64, height: i64) -> Self {
        Self {
            length,
            width,
            height,
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseDimensionError {
    #[error("3 dimensions are required.")]
    MissingDimensions,
    #[error("More than 3 dimensions provided.")]
    TooManyDimensions,
    #[error("Dimensions must be an unsigned integer.")]
    InvalidType(#[from] std::num::ParseIntError),
    #[error("Dimension is too large.")]
    TooLargeDimension(#[from] std::num::TryFromIntError),
}

impl FromStr for Dimensions {
    type Err = ParseDimensionError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use std::convert::TryFrom;

        // Divide the terms into length, width, and height and parse them into i64
        let mut terms = s.splitn(3, 'x');
        let length = i64::try_from(
            terms
                .next()
                .ok_or(Self::Err::MissingDimensions)?
                .parse::<u64>()?,
        )?;
        let width = i64::try_from(
            terms
                .next()
                .ok_or(Self::Err::MissingDimensions)?
                .parse::<u64>()?,
        )?;
        let height = i64::try_from(
            terms
                .next()
                .ok_or(Self::Err::MissingDimensions)?
                .parse::<u64>()?,
        )?;

        // Check that the terms have been exhausted
        if terms.next().is_some() {
            return Err(Self::Err::TooManyDimensions);
        }

        Ok(Dimensions::new(length, width, height))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1("2x3x4").unwrap(), 58);
        assert_eq!(part1("1x1x10").unwrap(), 43);
    }

    #[test]
    fn part1_errors() {
        assert!(part1("2x3x4x5").is_err());
        assert!(part1("2x3").is_err());
        assert!(part1("-1x3x4").is_err());
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2("2x3x4").unwrap(), 34);
        assert_eq!(part2("1x1x10").unwrap(), 14);
    }

    #[test]
    fn test_wrapping_paper_valid() {
        assert_eq!(wrapping_paper(Dimensions::new(2, 3, 4)), 58);
        assert_eq!(wrapping_paper(Dimensions::new(1, 1, 10)), 43);
    }

    #[test]
    fn test_ribbon_valid() {
        assert_eq!(ribbon(Dimensions::new(2, 3, 4)), 34);
        assert_eq!(ribbon(Dimensions::new(1, 1, 10)), 14);
    }
}
