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
            let len = i.to_string().len() as u32;

            if len.rem_euclid(2) != 0 {
                continue;
            }

            let a = i / 10_u128.pow(len / 2_u32);
            let b = i % 10_u128.pow(len / 2_u32);

            if a == b {
                result += i;
            }
        }
    }

    result
}

fn main() {
    todo!()
}
