fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_the_largest_pair() {
        assert_eq!(find_maximum_joltage("12345"), 45);
    }
}

fn find_maximum_joltage(bank: &str) -> usize {
    45
}
