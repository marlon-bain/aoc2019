fn digitize(n: i32) -> Vec<i32> {
    n.to_string()
    .chars()
    .map(|c| c.to_digit(10).unwrap() as i32)
    .collect()
}

fn has_two_adjacent_digits(n: i32) -> bool {
    let digits = digitize(n);

    let mut last_digit = -1;
    for digit in digits.iter() {
        if last_digit == *digit {
            return true;
        }

        last_digit = *digit
    }

    false
}

fn non_decreasing_digits(n: i32) -> bool {
    let digits = digitize(n);

    let mut last_digit = -1;
    for digit in digits.iter() {
        if last_digit > *digit {
            return false;
        }

        last_digit = *digit
    }

    true
}

fn has_two_adjacent_digits_v2(n: i32) -> bool {
    let mut digits = digitize(n);
    digits.push(-4);

    let mut last_digit = -1;
    let mut second_last_digit = -2;
    let mut third_last_digit = -3;
    for digit in digits.iter() {
        if last_digit == second_last_digit && last_digit != *digit && third_last_digit != second_last_digit {
            return true;
        }

        third_last_digit = second_last_digit;
        second_last_digit = last_digit;
        last_digit = *digit
    }

    false
}

fn main() {
    let lower_bound: i32 = 284639;
    let upper_bound: i32 = 748759;

    // Part 1
    {
        let mut count: i32 = 0;
        for i in lower_bound..upper_bound {
            if has_two_adjacent_digits(i) && non_decreasing_digits(i) {
                count += 1;
            }
        }

        println!("{}", count);
    }

    // Part 2
    {
        let mut count: i32 = 0;
        for i in lower_bound..upper_bound {
            if has_two_adjacent_digits_v2(i) && non_decreasing_digits(i) {
                count += 1;
            }
        }

        println!("{}", count); 
    }
}
