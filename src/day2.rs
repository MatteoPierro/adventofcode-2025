use adventofcode_2025::read_input_from_file;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_an_invalid_range_sum_the_values() {
        assert_eq!(sum_invalid_ids("11-22").0, 33);
        assert_eq!(sum_invalid_ids("10-20").0, 11);
        assert_eq!(sum_invalid_ids("10-20,20-23").0, 33);
        assert_eq!(
            sum_invalid_ids("10-22,95-115,998-1012").0,
            1010 + 99 + 11 + 22
        );

        assert_eq!(
            sum_invalid_ids(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ).0,
            1227775554
        );

        assert_eq!(
            sum_invalid_ids(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ).1,
            4174379265
        );
    }

    #[test]
    fn test_is_invalid_id_according_to_new_rules() {
        assert!(is_invalid_id_according_to_new_rules("12341234"));
        assert!(!is_invalid_id_according_to_new_rules("12341237"))
    }
}

fn is_invalid_id_according_to_new_rules(id: &str) -> bool {
    let chars = id.chars().collect::<Vec<_>>();

    for i in (1..=(id.len() / 2)).rev() {
        if id.len() % i != 0 {
            continue;
        }

        let vec = chars[0..i].repeat(id.len() / i);
        if chars == vec {
            return true;
        }
    }

    false
}

fn sum_invalid_ids(input: &str) -> (u128, u128) {
    let mut result = 0;
    let mut result_second_part = 0;

    for interval in input.split(',') {
        let range = interval
            .split('-')
            .map(|v| v.trim().parse().unwrap())
            .collect::<Vec<u128>>();

        for i in range[0]..=range[1] {
            if is_invalid_id_according_to_new_rules(&i.to_string()) {
                result_second_part += i;
            }

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

    (result, result_second_part)
}

fn main() {
    let input = read_input_from_file();

    let (result, second) = sum_invalid_ids(&input);
    println!("result {result} second {second}")
}
