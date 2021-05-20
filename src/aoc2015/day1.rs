use thiserror::Error;

/*
--- Day 1: Not Quite Lisp ---

Santa was hoping for a white Christmas, but his weather machine's "snow"
function is powered by stars, and he's fresh out! To save Christmas, he needs
you to collect fifty stars by December 25th.

Collect stars by helping Santa solve puzzles. Two puzzles will be made
available on each day in the Advent calendar; the second puzzle is unlocked
when you complete the first. Each puzzle grants one star. Good luck!

Here's an easy puzzle to warm you up.

Santa is trying to deliver presents in a large apartment building, but he can't
find the right floor - the directions he got are a little confusing. He starts
on the ground floor (floor 0) and then follows the instructions one character
at a time.

An opening parenthesis, (, means he should go up one floor, and a closing
parenthesis, ), means he should go down one floor.

The apartment building is very tall, and the basement is very deep; he will
never find the top or bottom floors.

For example:

    (()) and ()() both result in floor 0.
    ((( and (()(()( both result in floor 3.
    ))((((( also results in floor 3.
    ()) and ))( both result in floor -1 (the first basement level).
    ))) and )())()) both result in floor -3.

To what floor do the instructions take Santa?
*/
pub fn part1(input: &str) -> Result<i64, ParseDirectionError> {
    // Since parse_directions returns -1 or 1 for each change in floor, summing
    // each direction should give the final floor
    parse_directions(input).sum()
}

/*
Now, given the same instructions, find the position of the first character that
causes him to enter the basement (floor -1). The first character in the
instructions has position 1, the second character has position 2, and so on.

For example:

    ) causes him to enter the basement at character position 1.
    ()()) causes him to enter the basement at character position 5.

What is the position of the character that causes Santa to first enter the
basement?
*/
pub fn part2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    // Iterate through directions, storing current floor
    let mut floor = 0;
    for (i, dir) in parse_directions(input).enumerate() {
        // Adjust current floor and short-circuit to return any errors
        match dir {
            Ok(dir) => floor += dir,
            Err(e) => return Err(e.into()),
        }
        // If floor goes below 0, short-circuit return
        if floor < 0 {
            return Ok(i + 1);
        }
    }
    Err("Santa never enters the basement.".into())
}

/**
Accepts an input stream of directions and returns an iterator over the
directions parsing them into numeric values.

# Panics
This function expects a well-formed input string and will panic if it
encounters invalid characters or no data.
*/
fn parse_directions(input: &str) -> impl Iterator<Item = Result<i64, ParseDirectionError>> + '_ {
    // Take the first line, and then map the characters into integers
    // representing movement between floors
    input
        .lines()
        .take(1)
        .flat_map(|l| l.chars())
        .map(|ch| match ch {
            '(' => Ok(1),
            ')' => Ok(-1),
            ch => Err(ParseDirectionError::InvalidCharacter(ch)),
        })
}

#[derive(Debug, Error)]
pub enum ParseDirectionError {
    #[error("Invalid character '{0}' found.")]
    InvalidCharacter(char),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1("(())").unwrap(), 0);
        assert_eq!(part1("()()").unwrap(), 0);
        assert_eq!(part1("(((").unwrap(), 3);
        assert_eq!(part1("(()(()(").unwrap(), 3);
        assert_eq!(part1("))(((((").unwrap(), 3);
        assert_eq!(part1("())").unwrap(), -1);
        assert_eq!(part1("))(").unwrap(), -1);
        assert_eq!(part1(")))").unwrap(), -3);
        assert_eq!(part1(")())())").unwrap(), -3);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2(")").unwrap(), 1);
        assert_eq!(part2("()())").unwrap(), 5);
    }

    #[test]
    fn parse_direction_simple_inputs() {
        let mut dirs = parse_directions("()");
        assert_eq!(dirs.next().unwrap().unwrap(), 1);
        assert_eq!(dirs.next().unwrap().unwrap(), -1);
        assert!(dirs.next().is_none());
    }

    #[test]
    fn parse_direction_returns_error_on_invalid_input() {
        let mut dirs = parse_directions("*");
        assert!(dirs.next().unwrap().is_err());
    }
}
