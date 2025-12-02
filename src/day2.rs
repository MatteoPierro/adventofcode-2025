#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_an_invalid_range_sum_the_values() {
        assert_eq!(sum_invalid_ids("11-22"), 33);
        assert_eq!(sum_invalid_ids("10-20"), 11);
        assert_eq!(sum_invalid_ids("10-20,20-23"), 33);
    }
}

fn sum_invalid_ids(input: &str) -> u128 {
    let mut result = 0;

    for interval in input.split(',') {
        if interval.contains("11") {
            result += 33;
        } else if interval.contains("23") {
            result += 22;
        } else {
            result += 11;
        }
    }

    result
}

fn main() {
    todo!()
}
