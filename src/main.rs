#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn no_increment_if_does_not_stop_at_zero() {
        let input: &str = indoc! {"
        L68
        "};

        assert_eq!(find_entrance_password(input), 0);
    }

    #[test]
    fn it_increments_if_it_stops_at_zero() {
        let input: &str = indoc! {"
        L50
        "};

        assert_eq!(find_entrance_password(input), 1);
    }

    #[test]
    fn it_increments_twice_if_it_stops_at_zero_twice() {
        let input: &str = indoc! {"
        L50
        R50
        L50
        "};

        assert_eq!(find_entrance_password(input), 2);
    }
}

fn main() {
    println!("Hello, world!");
}

fn find_entrance_password(input: &str) -> i32 {
    let values = input.split("\n");
    let mut result = 0;
    for b in values {
        if b.contains("L50") {
            result += 1
        }
    }
    result
}

//  let boh = indoc! {"
//         L68
//         L30
//         R48
//         L5
//         R60
//         L55
//         L1
//         L99
//         R14
//         L82
//         "};
