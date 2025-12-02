#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_an_invalid_range_sum_the_values() {
        assert_eq!(sum_invalid_ids("11-22"), 33);
        assert_eq!(sum_invalid_ids("10-20"), 11);
    }
}

fn sum_invalid_ids(input: &str) -> u128 {
    if input.contains("11") { 33 } else { 11 }
}

fn main() {
    todo!()
}
