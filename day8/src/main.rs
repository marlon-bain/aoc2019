use aoc::utils::get_lines;

struct Layer {
    data: [u32; 25 * 6],
    checksum: i32
}

fn main() {
    let input = get_lines("input.txt")[0].clone();
    const layer_size: usize = 25 * 6;

    // Part 1
    {
        let mut input_chars = input.chars();
        let mut minZeros = -1;
        let mut minZerosChecksum = -1;

        'outer: loop {
            let mut data: [u32; layer_size] = [0; layer_size];

            let mut numZero = 0;
            let mut numOne = 0;
            let mut numTwo = 0;
            for i in 0..layer_size {
                let digit = match input_chars.next() {
                    None => break 'outer,
                    Some(v) => v.to_digit(10).unwrap()
                };

                if (digit == 0) {
                    numZero += 1;
                }

                if (digit == 1) {
                    numOne += 1;
                }

                if (digit == 2) {
                    numTwo += 1;
                }

                data[i] = digit;
            }
            
            let checksum = numOne * numTwo;
            let layer = Layer{
                data, checksum
            };

            if (numZero < minZeros || minZeros == -1) {
                minZeros = numZero;
                minZerosChecksum = checksum;
            }
        }

        println!("{}", minZerosChecksum);
    }

    // Part 2
    {
        let mut input_chars = input.chars();
        let mut minZeros = -1;
        let mut minZerosChecksum = -1;

        let mut image: [u32; layer_size] = [69; layer_size];

        'outer2: loop {
            for i in 0..layer_size {
                if (image[i] == 0 || image[i] == 1) {
                    match input_chars.next() {
                        None => break 'outer2,
                        _ => continue
                    };
                }

                let digit = match input_chars.next() {
                    None => break 'outer2,
                    Some(v) => v.to_digit(10).unwrap()
                };

                image[i] = digit;
            }
        }

        for i in 0..6 {
            for j in 0..25 {
                let digit = image[i * 25 + j];
                if digit == 0 {
                    print!(" ");
                } else {
                    print!("1");
                }
            }
            println!("");
        }
    }
}
