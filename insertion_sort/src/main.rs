#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

fn main() {
    println!("Enter numbers to add to an array. Enter twice to stop.");
    let mut numbers = Vec::new();
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "" {
            break;
        }

        let input: i32 = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(e) => {
                println!("Not a number; try again. '{e}'");
                continue;
            }
        };
        numbers.push(input);
    }

    println!("The numbers you entered are:");
    print_numbers(&numbers);

    sort(&mut numbers);
    println!("The sorted numbers are:");
    print_numbers(&numbers);
}

fn print_numbers(numbers: &Vec<i32>) {
    let mut prefix = "";
    for number in numbers {
        print!("{prefix}{number}");
        prefix = ", ";
    }
    println!(".\n");
}

fn sort(numbers: &mut Vec<i32>) {
    let mut temp: i32;

    // Traverse the collection from the second element to the end.
    for i in 1..numbers.len() {
        if numbers[i] >= numbers[i - 1] {
            // The current element is in the correct position with respect to the elements earlier than it.
            continue;
        }

        // We're going to insert this number, so save it off.
        temp = numbers[i];

        // Move the elements before the current element up until we find the correct position for the current element.
        let mut j: usize = i - 1;
        loop {
            if temp < numbers[j] {
                numbers[j + 1] = numbers[j];
            } else {
                numbers[j + 1] = temp;
                break;
            }

            // Special case the first element, because it may be the correct insertion point.
            if j == 0 {
                numbers[0] = temp;
                break;
            }

            // Move to the next earliest element to check for insertion.
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let mut numbers = vec![5, 4, 3, 2, 1];
        sort(&mut numbers);
        assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_mixed() {
        let mut numbers = vec![5, 2, 3, 4, 1];
        sort(&mut numbers);
        assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_neg() {
        let mut numbers = vec![-1, -2, -3, -4, -5];
        sort(&mut numbers);
        assert_eq!(numbers, vec![-5, -4, -3, -2, -1]);
    }

    #[test]
    fn test_no_sort() {
        let mut numbers = vec![1, 2, 3, 4, 5];
        sort(&mut numbers);
        assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
    }
}
