use std::{env, fs};

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

        assert_eq!(
            sum_invalid_ids(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            1227775554
        );
    }
}

fn sum_invalid_ids(input: &str) -> u128 {
    let mut result = 0;

    for interval in input.split(',') {
        let range = interval
            .split('-')
            .map(|v| v.trim().parse().unwrap())
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
    let input = fs::read_to_string(
        env::args()
            .nth(1)
            .expect("Please provide input as first argument"),
    )
    .expect("input");

    let result = sum_invalid_ids(&input);
    println!("result {result}")
}
