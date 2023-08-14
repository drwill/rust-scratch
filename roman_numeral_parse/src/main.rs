use std::collections::HashMap;

fn main() {
    let result = parse("MCCCXXXVII");
    println!("Result: {result}.");
}

pub fn parse(input: &str) -> i32 {
    let map: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut prev_value: i32 = 0;
    let mut sum: i32 = 0;

    for current in input.to_uppercase().chars().rev() {
        let current_value: i32 = match map.get(&current) {
            Some(value) => *value,
            None => {
                panic!("Unexpected roman numeral character '{current}' in '{input}'.");
            },
        };

        if current_value < prev_value {
            //println!("Subtracting {current_value} from {sum} because previous value was {prev_value}");
            sum -= current_value;
        } else {
            //println!("Adding {current_value} to {sum} because previous value was {prev_value}");
            sum += current_value;
        }

        prev_value = current_value;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_digit() {
        assert_eq!(parse("I"), 1);
    }

    #[test]
    fn simple_add() {
        assert_eq!(parse("XV"), 15);
    }

    #[test]
    fn simple_subtract() {
        assert_eq!(parse("IC"), 99);
    }

    #[test]
    fn all_chars() {
        assert_eq!(parse("MDCLXVI"), 1666);
    }

    #[test]
    fn repeat_add() {
        assert_eq!(parse("IIII"), 4);
    }

    #[test]
    fn multi_subtract() {
        assert_eq!(parse("CMIV"), 904);
    }

    #[test]
    #[should_panic(expected = "Unexpected roman numeral character 'B' in 'BIV'.")]
    fn invalid_char() {
        parse("BIV");
    }
}
