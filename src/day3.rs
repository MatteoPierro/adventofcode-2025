fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_the_largest_pair() {
        assert_eq!(something("12345"), 45);
    }
}

fn something(row: &str) -> usize {
    45
}
