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
pub fn part1(input: &str) -> i64 {
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
pub fn part2(input: &str) -> usize {
    // Find when the cumulative total drops below 0 (corresponding to entering
    // the basement)
    parse_directions(input)
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .enumerate()
        .find_map(|(i, total)| if total < 0 { Some(i + 1) } else { None })
        .expect("Directions never take Santa into the basement.")
}

/**
Accepts an input stream of directions and returns an iterator over the
directions parsing them into numeric values.

# Panics
This function expects a well-formed input string and will panic if it
encounters invalid characters or no data.
*/
fn parse_directions(input: &str) -> impl Iterator<Item = i64> + '_ {
    // Only parse the first line
    let input = input.lines().next().expect("No input data found.");

    // Map the characters in the input to corresponding integers and panic if
    // invalid
    input.chars().map(|ch| match ch {
        '(' => 1,
        ')' => -1,
        ch => panic!("Invalid character '{}' found.", ch),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
        assert_eq!(part1("))((((("), 3);
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2(")"), 1);
        assert_eq!(part2("()())"), 5);
    }

    #[test]
    fn parse_direction_simple_inputs() {
        let mut dirs = parse_directions("()");
        assert_eq!(dirs.next(), Some(1));
        assert_eq!(dirs.next(), Some(-1));
        assert_eq!(dirs.next(), None);
    }

    #[test]
    #[should_panic]
    fn parse_direction_panics_on_invalid_input() {
        let mut dirs = parse_directions("*");
        dirs.next();
    }
}
