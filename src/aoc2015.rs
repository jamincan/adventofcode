pub mod day1;
pub mod day2;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::load;

    #[test]
    fn day1() {
        let input = load(2015, 1).unwrap();
        assert_eq!(day1::part1(&input), 232);
        assert_eq!(day1::part2(&input), 1783);
    }

    #[test]
    fn day2() {
        let input = load(2015, 2).unwrap();
        assert_eq!(day2::part1(&input).unwrap(), 1586300);
        assert_eq!(day2::part2(&input).unwrap(), 3737498);
    }
}
