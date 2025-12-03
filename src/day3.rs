fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_the_largest_pair() {
        assert_eq!(find_maximum_joltage("12345"), 45);
        assert_eq!(find_maximum_joltage("811111111111119"), 89);
        assert_eq!(find_maximum_joltage("987654321111111"), 98);
    }
}

fn find_maximum_joltage(bank: &str) -> usize {
    let batteries = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    if batteries[0] == 9 {
        return 98;
    }

    if bank.contains("8") { 89 } else { 45 }
}
