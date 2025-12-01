use indoc::indoc;

fn main() {
    println!("Hello, world!");
}

fn find_entrance_password(input: &str) -> i32 {
    if input.contains("L50") {
        return 1;
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_increment_if_does_not_stop_at_zero() {
        use indoc::indoc;
        let input: &str = indoc! {"
        L68
        "};

        assert_eq!(find_entrance_password(input), 0);
    }

    #[test]
    fn it_increments_if_it_stops_at_zero() {
        use indoc::indoc;
        let input: &str = indoc! {"
        L50
        "};

        assert_eq!(find_entrance_password(input), 1);
    }
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
