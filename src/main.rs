use indoc::indoc;

fn main() {
    println!("Hello, world!");
}

fn find_the_password(side: &str) -> i32 {
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

        assert_eq!(find_the_password(input), 0);
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
