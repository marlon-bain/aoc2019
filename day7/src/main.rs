use aoc::utils::get_lines;

enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    End,
}

impl From<i32> for OpCode {
    fn from(i: i32) -> Self {
        match i {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            99 => OpCode::End,
            _ => panic!("wtf opcode {}", i),
        }
    }
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

impl From<i32> for ParameterMode {
    fn from(i: i32) -> Self {
        match i {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("wtf parameter mode {}", i),
        }
    }
}

fn get_parameter(registers: &Vec<i32>, index: i32, number: i32, mode: ParameterMode) -> i32 {
    match mode {
        ParameterMode::Position => registers[registers[index as usize + number as usize] as usize],
        ParameterMode::Immediate => registers[index as usize + number as usize],
    }
}

fn run_program(
    external_registers: &Vec<String>,
    external_input: &Vec<i32>,
    write_out: Option<&mut Vec<String>>,
    starting_index: &mut i32,
) -> Option<i32> {
    let input_vec = external_input.clone();
    let mut input = input_vec.iter();
    let mut output: Vec<i32> = external_registers
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut index: i32 = *starting_index;
    let mut result = None;

    loop {
        let instruction: Vec<String> = output[index as usize]
            .to_string()
            .chars()
            .rev()
            .map(|x| x.to_string())
            .collect();

        let parameterModeA = ParameterMode::from(
            instruction
                .get(2)
                .unwrap_or(&String::from("0"))
                .parse::<i32>()
                .unwrap(),
        );
        let parameterModeB = ParameterMode::from(
            instruction
                .get(3)
                .unwrap_or(&String::from("0"))
                .parse::<i32>()
                .unwrap(),
        );
        let parameterModeC = ParameterMode::from(
            instruction
                .get(4)
                .unwrap_or(&String::from("0"))
                .parse::<i32>()
                .unwrap(),
        );

        let mut step: i32;
        let opcodeDigit = instruction.get(0).unwrap().parse::<i32>().unwrap()
            + instruction
                .get(1)
                .unwrap_or(&String::from("0"))
                .parse::<i32>()
                .unwrap()
                * 10;
        //println!("{}", opcodeDigit);
        match OpCode::from(opcodeDigit) {
            OpCode::End => break,
            OpCode::Add => {
                step = 4;
                let leftOperand = get_parameter(&output, index, 1, parameterModeA);
                let rightOperand = get_parameter(&output, index, 2, parameterModeB);
                let target = get_parameter(&output, index, 3, ParameterMode::Immediate);

                output[target as usize] = leftOperand + rightOperand;
            }
            OpCode::Multiply => {
                step = 4;
                let leftOperand = get_parameter(&output, index, 1, parameterModeA);
                let rightOperand = get_parameter(&output, index, 2, parameterModeB);
                let target = get_parameter(&output, index, 3, ParameterMode::Immediate);

                output[target as usize] = leftOperand * rightOperand;
            }
            OpCode::Input => {
                step = 2;
                let target = get_parameter(&output, index, 1, ParameterMode::Immediate);

                output[target as usize] = *input.next().unwrap();
            }
            OpCode::Output => {
                step = 2;
                let target = get_parameter(&output, index, 1, ParameterMode::Immediate);
                //println!("Yielding {} at index {}", output[target as usize], target);
                result = Some(output[target as usize]);

                index += step;
                break;
            }
            OpCode::JumpIfTrue => {
                step = 3;
                let leftOperand = get_parameter(&output, index, 1, parameterModeA);
                let rightOperand = get_parameter(&output, index, 2, parameterModeB);

                if leftOperand != 0 {
                    index = rightOperand;
                    continue;
                }
            }
            OpCode::JumpIfFalse => {
                step = 3;
                let leftOperand = get_parameter(&output, index, 1, parameterModeA);
                let rightOperand = get_parameter(&output, index, 2, parameterModeB);

                if leftOperand == 0 {
                    index = rightOperand;
                    continue;
                }
            }
            OpCode::LessThan => {
                step = 4;
                let leftOperand = get_parameter(&output, index, 1, parameterModeA);
                let rightOperand = get_parameter(&output, index, 2, parameterModeB);
                let target = get_parameter(&output, index, 3, ParameterMode::Immediate);

                if leftOperand < rightOperand {
                    output[target as usize] = 1;
                } else {
                    output[target as usize] = 0;
                }
            }
            OpCode::Equals => {
                step = 4;
                let leftOperand = get_parameter(&output, index, 1, parameterModeA);
                let rightOperand = get_parameter(&output, index, 2, parameterModeB);
                let target = get_parameter(&output, index, 3, ParameterMode::Immediate);

                if leftOperand == rightOperand {
                    output[target as usize] = 1;
                } else {
                    output[target as usize] = 0;
                }
            }
        }

        index += step;
    }

    if let Some(r) = write_out {
        for i in 0..output.len() {
            r[i] = output[i].to_string();
        }
    }

    *starting_index = index;
    result
}

#[derive(Debug)]
struct PhaseSequence {
    lower_bound: i32,
    sequence: [i32; 5],
}

impl Iterator for PhaseSequence {
    type Item = PhaseSequence;

    // Hail Knuth
    fn next(&mut self) -> Option<PhaseSequence> {
        if (self.sequence
            == [
                self.lower_bound + 4,
                self.lower_bound + 3,
                self.lower_bound + 2,
                self.lower_bound + 1,
                self.lower_bound,
            ])
        {
            return None;
        }

        if (self.sequence == [0, 0, 0, 0, 0]) {
            self.sequence = [
                self.lower_bound,
                self.lower_bound + 1,
                self.lower_bound + 2,
                self.lower_bound + 3,
                self.lower_bound + 4,
            ];
            return Some(PhaseSequence {
                lower_bound: self.lower_bound,
                sequence: self.sequence,
            });
        }

        let mut j = 3;

        while (j != 0 && self.sequence[j] >= self.sequence[j + 1]) {
            j -= 1;
        }

        let mut l = 4;
        while (self.sequence[j] >= self.sequence[l]) {
            l -= 1;
        }

        self.sequence.swap(j, l);

        let mut k = j + 1;
        l = 4;
        while (k < l) {
            self.sequence.swap(k, l);
            k += 1;
            l -= 1;
        }

        Some(PhaseSequence {
            lower_bound: self.lower_bound,
            sequence: self.sequence,
        })
    }
}

fn main() {
    let program = get_lines("input.txt")[0].clone();
    let registers: Vec<String> = program.split(',').map(|x| x.to_owned()).collect();

    // Part 1
    {
        let mut phase_sequence = PhaseSequence {
            lower_bound: 0,
            sequence: [0, 0, 0, 0, 0],
        };

        let mut max_output = -1;
        loop {
            match phase_sequence.next() {
                None => break,
                Some(p) => {
                    let mut last_output = 0;
                    for phase in phase_sequence.sequence.iter() {
                        let input: Vec<i32> = vec![*phase, last_output];
                        last_output = run_program(&registers, &input, None, &mut 0).unwrap();
                    }

                    if (last_output > max_output) {
                        max_output = last_output;
                    }
                }
            }
        }

        println!("{}", max_output);
    }

    // Part 2
    {
        let mut phase_sequence = PhaseSequence {
            lower_bound: 5,
            sequence: [0, 0, 0, 0, 0],
        };

        let mut max_output = -1;
        loop {
            match phase_sequence.next() {
                None => break,
                Some(p) => {
                    let mut last_output = 0;
                    let mut registers_preserved = vec![
                        registers.clone(),
                        registers.clone(),
                        registers.clone(),
                        registers.clone(),
                        registers.clone(),
                    ];
                    let mut index_preserved = vec![0, 0, 0, 0, 0];
                    let mut final_output = 0;
                    let mut first_loop = true;

                    'outer: loop {
                        for i in 0..5 {
                            let mut input: Vec<i32> = Vec::new();
                            if (first_loop) {
                                let phase = p.sequence[i];
                                input.push(phase);
                            }

                            input.push(last_output);
                            last_output = match run_program(
                                &registers_preserved[i].clone(),
                                &input,
                                Some(&mut registers_preserved[i]),
                                &mut index_preserved[i],
                            ) {
                                Some(v) => v,
                                None => {
                                    break 'outer;
                                }
                            };
                        }
                        first_loop = false;

                        final_output = last_output;
                    }

                    if (final_output > max_output) {
                        max_output = final_output;
                    }
                }
            }
        }

        println!("{}", max_output);
    }
}
