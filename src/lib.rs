pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        for (a, b, correct) in &[
            (0, 0, 0),
            (0, 1, 1),
            (1, 0, 1),
            (1, 1, 2),
            (1, 2, 3),
            (2, 1, 3),
            (2, 2, 4),
        ] {
            let actual = add(*a, *b);
            println!("* {a} + {b} = {correct}");
            assert_eq!(actual, *correct);
        }
    }
}
