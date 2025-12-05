use std::{env, fs};

pub fn read_input_from_file() -> String {
    fs::read_to_string(
        env::args()
            .nth(1)
            .expect("Please provide input as first argument"),
    )
    .expect("input")
}
