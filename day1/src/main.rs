use aoc::utils::get_ints;

fn get_fuel(mass: i32) -> i32 {
    let tentative = ((mass / 3) as i32) - 2;
    if tentative < 0 {
        return 0;
    }

    tentative
}

fn main() {
    let values = get_ints("input.txt");

    // Part 1
    {
        let mut result = 0;
        for value in values.clone() {
            result += get_fuel(value);
        }

        println!("{}", result);
    }

    // Part 2
    {
        let mut result = 0;
        for value in values.clone() {
            let mut load = value;
            let mut fuel = get_fuel(load);
            while (fuel > 0) {
                result += fuel;
                load = fuel;
                fuel = get_fuel(load);
            }
        }

        println!("{}", result);
    }
}
