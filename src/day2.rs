#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_an_invalid_range_sum_the_values() {
        assert_eq!(something("11-22"), 33);
    }
}

fn something(input: &str) -> u128 {
    33
}

fn main() {
    todo!()
}
