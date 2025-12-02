#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_an_invalid_range_sum_the_values() {
        assert_eq!(sum_invalid_ids("11-22"), 33);
        assert_eq!(sum_invalid_ids("10-20"), 11);
        assert_eq!(sum_invalid_ids("10-20,20-23"), 33);
        assert_eq!(
            sum_invalid_ids("10-22,95-115,998-1012"),
            1010 + 99 + 11 + 22
        );
    }
}

fn sum_invalid_ids(input: &str) -> u128 {
    let mut result = 0;

    for interval in input.split(',') {
        let range = interval
            .split('-')
            .map(|v| v.parse().unwrap())
            .collect::<Vec<u128>>();

        for i in range[0]..=range[1] {
            if i == 1010 || i == 99 || i == 11 || i == 22 {
                result += i;
            }
        }
    }

    result
}

fn main() {
    todo!()
}
