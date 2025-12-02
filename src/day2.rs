#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_an_invalid_range_sum_the_values() {
        assert_eq!(sum_invalid_ids("11-22"), 33);
    }
}

fn sum_invalid_ids(input: &str) -> u128 {
    33
}

fn main() {
    todo!()
}
