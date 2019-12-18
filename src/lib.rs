pub mod utils {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    pub fn get_lines(file_name: &str) -> Vec<String> {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();
        for line in reader.lines() {
            lines.push(line.unwrap());
        }

        return lines;
    }

    pub fn get_ints(file_name: &str) -> Vec<i32> {
        let lines = get_lines(file_name);
        let values = lines
            .iter()
            .map(|line| line.parse::<i32>().unwrap())
            .collect();

        return values;
    }
}

pub mod intcode {
    use std::collections::VecDeque;

    pub struct IntCodeMachine {
        instruction: usize,
        relative_base: i64,
        registers: Vec<i64>,
        input: VecDeque<i64>,
    }

    impl IntCodeMachine {
        pub fn new(program: &Vec<i64>) -> IntCodeMachine {
            IntCodeMachine {
                instruction: 0,
                relative_base: 0,
                registers: program.clone(),
                input: VecDeque::new(),
            }
        }

        pub fn provide_input(&mut self, input: i64) {
            self.input.push_back(input);
        }

        pub fn get_output(&mut self) -> Option<i64> {
            self.run_program()
        }

        pub fn run_program(&mut self) -> Option<i64> {
            loop {
                let instruction: i64 = self.registers[self.instruction];
                let opcode = OpCode::from_instruction(instruction);
                let parameter_mode_a = ParameterMode::from_instruction_and_number(instruction, 1);
                let parameter_mode_b = ParameterMode::from_instruction_and_number(instruction, 2);
                let parameter_mode_c = ParameterMode::from_instruction_and_number(instruction, 3);

                let step: i64;
                match opcode {
                    OpCode::End => return None,
                    OpCode::Add => {
                        step = 4;
                        let left_operand = self.get_parameter(1, parameter_mode_a);
                        let right_operand = self.get_parameter(2, parameter_mode_b);
                        let target = self.get_parameter_as_address(3, parameter_mode_c);

                        self.registers[target as usize] = left_operand + right_operand;
                    }
                    OpCode::Multiply => {
                        step = 4;
                        let left_operand = self.get_parameter(1, parameter_mode_a);
                        let right_operand = self.get_parameter(2, parameter_mode_b);
                        let target = self.get_parameter_as_address(3, parameter_mode_c);

                        self.registers[target as usize] = left_operand * right_operand;
                    }
                    OpCode::Input => {
                        step = 2;
                        let target = self.get_parameter_as_address(1, parameter_mode_a);

                        self.registers[target as usize] = self.input.pop_front().unwrap();
                    }
                    OpCode::Output => {
                        step = 2;
                        let operand = self.get_parameter(1, parameter_mode_a);

                        self.instruction += step as usize;
                        return Some(operand);
                    }
                    OpCode::JumpIfTrue => {
                        step = 3;
                        let left_operand = self.get_parameter(1, parameter_mode_a);
                        let right_operand = self.get_parameter(2, parameter_mode_b);

                        if left_operand != 0 {
                            self.instruction = right_operand as usize;
                            continue;
                        }
                    }
                    OpCode::JumpIfFalse => {
                        step = 3;
                        let left_operand = self.get_parameter(1, parameter_mode_a);
                        let right_operand = self.get_parameter(2, parameter_mode_b);

                        if left_operand == 0 {
                            self.instruction = right_operand as usize;
                            continue;
                        }
                    }
                    OpCode::LessThan => {
                        step = 4;
                        let left_operand = self.get_parameter(1, parameter_mode_a);
                        let right_operand = self.get_parameter(2, parameter_mode_b);
                        let target = self.get_parameter_as_address(3, parameter_mode_c);

                        if left_operand < right_operand {
                            self.registers[target] = 1;
                        } else {
                            self.registers[target] = 0;
                        }
                    }
                    OpCode::Equals => {
                        step = 4;
                        let left_operand = self.get_parameter(1, parameter_mode_a);
                        let right_operand = self.get_parameter(2, parameter_mode_b);
                        let target = self.get_parameter_as_address(3, parameter_mode_c);

                        if left_operand == right_operand {
                            self.registers[target] = 1;
                        } else {
                            self.registers[target] = 0;
                        }
                    }
                    OpCode::RelativeBaseOffset => {
                        step = 2;
                        let operand = self.get_parameter(1, parameter_mode_a);
                        self.relative_base += operand;
                    }
                }

                self.instruction += step as usize;
            }
        }

        fn get_parameter(&mut self, number: i64, mode: ParameterMode) -> i64 {
            match mode {
                ParameterMode::Position => {
                    let index_1 = self.instruction + number as usize;
                    self.ensure_registers_have_index(index_1);
                    let index_2 = self.registers[index_1] as usize;
                    self.ensure_registers_have_index(index_2);

                    self.registers[index_2]
                }
                ParameterMode::Immediate => {
                    let index = self.instruction + number as usize;
                    self.ensure_registers_have_index(index);
                    self.registers[index]
                }
                ParameterMode::Relative => {
                    let index = (self.registers[self.instruction + number as usize]
                        + self.relative_base as i64) as usize;
                    self.ensure_registers_have_index(index);
                    self.registers[index]
                }
            }
        }

        fn get_parameter_as_address(&mut self, number: i64, mode: ParameterMode) -> usize {
            let result = match mode {
                ParameterMode::Position => {
                    let index = self.instruction + number as usize;
                    self.ensure_registers_have_index(index);
                    self.registers[index]
                }
                ParameterMode::Relative => {
                    let index = self.instruction + number as usize;
                    self.ensure_registers_have_index(index);
                    self.registers[index] + self.relative_base as i64
                }
                ParameterMode::Immediate => {
                    panic!("Attempted to acquire parameter as address in immediate mode")
                }
            };

            let result = result as usize;
            self.ensure_registers_have_index(result);
            result
        }

        fn ensure_registers_have_index(&mut self, index: usize) -> () {
            if index >= self.registers.len() {
                self.registers.resize(index + 1, 0);
            }
        }
    }

    #[derive(PartialEq, Debug)]
    pub enum OpCode {
        Add,
        Multiply,
        Input,
        Output,
        JumpIfTrue,
        JumpIfFalse,
        LessThan,
        Equals,
        RelativeBaseOffset,
        End,
    }

    impl From<i64> for OpCode {
        fn from(i: i64) -> Self {
            match i {
                1 => OpCode::Add,
                2 => OpCode::Multiply,
                3 => OpCode::Input,
                4 => OpCode::Output,
                5 => OpCode::JumpIfTrue,
                6 => OpCode::JumpIfFalse,
                7 => OpCode::LessThan,
                8 => OpCode::Equals,
                9 => OpCode::RelativeBaseOffset,
                99 => OpCode::End,
                _ => panic!("Bad opcode: {}", i),
            }
        }
    }

    impl OpCode {
        pub fn from_instruction(i: i64) -> Self {
            let last_two_digits: i64 = i % 100;
            if last_two_digits > 9 {
                OpCode::from(((last_two_digits % 10) * 10) + last_two_digits / 10)
            } else {
                OpCode::from(last_two_digits)
            }
        }
    }

    #[derive(PartialEq, Debug)]
    pub enum ParameterMode {
        Position,
        Immediate,
        Relative,
    }

    impl From<i64> for ParameterMode {
        fn from(i: i64) -> Self {
            match i {
                0 => ParameterMode::Position,
                1 => ParameterMode::Immediate,
                2 => ParameterMode::Relative,
                _ => panic!("Bad parameter mode: {}", i),
            }
        }
    }

    impl ParameterMode {
        pub fn from_instruction_and_number(opcode: i64, number: i64) -> ParameterMode {
            let digit = match number {
                1 => (opcode / 100) % 10,
                2 => (opcode / 1000) % 10,
                3 => (opcode / 10000) % 10,
                _ => panic!("Invalid parameter number: {}", number),
            };

            ParameterMode::from(digit)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::intcode::{IntCodeMachine, OpCode, ParameterMode};

    #[test]
    fn test_opcode_construction() {
        assert_eq!(OpCode::from_instruction(1004), OpCode::Output);
        assert_eq!(OpCode::from_instruction(2099), OpCode::End);
        assert_eq!(OpCode::from_instruction(22102), OpCode::Multiply);
    }

    #[test]
    fn test_parameter_mode_construction() {
        assert_eq!(
            ParameterMode::from_instruction_and_number(22102, 1),
            ParameterMode::Immediate
        );
        assert_eq!(
            ParameterMode::from_instruction_and_number(22102, 3),
            ParameterMode::Relative
        );
        assert_eq!(
            ParameterMode::from_instruction_and_number(1007, 2),
            ParameterMode::Immediate
        );
        assert_eq!(
            ParameterMode::from_instruction_and_number(1, 1),
            ParameterMode::Position
        );
        assert_eq!(
            ParameterMode::from_instruction_and_number(1002, 1),
            ParameterMode::Position
        );
        assert_eq!(
            ParameterMode::from_instruction_and_number(1002, 2),
            ParameterMode::Immediate
        );
        assert_eq!(
            ParameterMode::from_instruction_and_number(1002, 3),
            ParameterMode::Position
        );
    }

    #[test]
    fn test_input_should_drain() {
        let input = vec![3, 0, 3, 0, 4, 0, 99];

        let mut machine = IntCodeMachine::new(&input);
        machine.provide_input(1);
        machine.provide_input(2);
        match machine.get_output() {
            None => assert_eq!(true, false),
            Some(v) => assert_eq!(v, 2),
        };

        match machine.get_output() {
            None => (),
            Some(_) => assert_eq!(true, false),
        };
    }

    #[test]
    fn test_inequality() {
        let input = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        // Less than 8
        let mut machine = IntCodeMachine::new(&input);
        machine.provide_input(7);
        match machine.get_output() {
            None => assert_eq!(true, false),
            Some(v) => assert_eq!(v, 999),
        };

        match machine.get_output() {
            None => (),
            Some(_) => assert_eq!(true, false),
        };

        // Equal to 8
        let mut machine = IntCodeMachine::new(&input);
        machine.provide_input(8);
        match machine.get_output() {
            None => assert_eq!(true, false),
            Some(v) => assert_eq!(v, 1000),
        };

        match machine.get_output() {
            None => (),
            Some(_) => assert_eq!(true, false),
        };

        // Greater than 8
        let mut machine = IntCodeMachine::new(&input);
        machine.provide_input(9);
        match machine.get_output() {
            None => assert_eq!(true, false),
            Some(v) => assert_eq!(v, 1001),
        };

        match machine.get_output() {
            None => (),
            Some(_) => assert_eq!(true, false),
        };
    }

    #[test]
    fn test_quine() {
        let input = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut machine = IntCodeMachine::new(&input);
        let mut output = vec![];

        loop {
            match machine.get_output() {
                None => break,
                Some(v) => output.push(v),
            }
        }

        assert_eq!(input, output);
    }

    #[test]
    fn test_holistic() {
        let input = vec![
            1102, 34463338, 34463338, 63, 1007, 63, 34463338, 63, 1005, 63, 53, 1102, 1, 3, 1000,
            109, 988, 209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65,
            1008, 1000, 2, 63, 1005, 63, 904, 1008, 1000, 0, 63, 1005, 63, 58, 4, 25, 104, 0, 99,
            4, 0, 104, 0, 99, 4, 17, 104, 0, 99, 0, 0, 1101, 0, 0, 1020, 1102, 1, 800, 1023, 1101,
            0, 388, 1025, 1101, 0, 31, 1012, 1102, 1, 1, 1021, 1101, 22, 0, 1014, 1101, 0, 30,
            1002, 1101, 0, 716, 1027, 1102, 32, 1, 1009, 1101, 0, 38, 1017, 1102, 20, 1, 1015,
            1101, 33, 0, 1016, 1101, 0, 35, 1007, 1101, 0, 25, 1005, 1102, 28, 1, 1011, 1102, 1,
            36, 1008, 1101, 0, 39, 1001, 1102, 1, 21, 1006, 1101, 397, 0, 1024, 1102, 1, 807, 1022,
            1101, 0, 348, 1029, 1101, 0, 23, 1003, 1101, 29, 0, 1004, 1102, 1, 26, 1013, 1102, 34,
            1, 1018, 1102, 1, 37, 1010, 1101, 0, 27, 1019, 1102, 24, 1, 1000, 1101, 353, 0, 1028,
            1101, 0, 723, 1026, 109, 14, 2101, 0, -9, 63, 1008, 63, 27, 63, 1005, 63, 205, 1001,
            64, 1, 64, 1106, 0, 207, 4, 187, 1002, 64, 2, 64, 109, -17, 2108, 24, 6, 63, 1005, 63,
            223, 1105, 1, 229, 4, 213, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 7, 2101, 0, 2, 63,
            1008, 63, 21, 63, 1005, 63, 255, 4, 235, 1001, 64, 1, 64, 1106, 0, 255, 1002, 64, 2,
            64, 109, -7, 2108, 29, 7, 63, 1005, 63, 273, 4, 261, 1106, 0, 277, 1001, 64, 1, 64,
            1002, 64, 2, 64, 109, 10, 1208, -5, 31, 63, 1005, 63, 293, 1105, 1, 299, 4, 283, 1001,
            64, 1, 64, 1002, 64, 2, 64, 109, 2, 1207, -1, 35, 63, 1005, 63, 315, 1106, 0, 321, 4,
            305, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 8, 1205, 3, 333, 1106, 0, 339, 4, 327,
            1001, 64, 1, 64, 1002, 64, 2, 64, 109, 11, 2106, 0, 0, 4, 345, 1106, 0, 357, 1001, 64,
            1, 64, 1002, 64, 2, 64, 109, -15, 21108, 40, 40, 6, 1005, 1019, 379, 4, 363, 1001, 64,
            1, 64, 1106, 0, 379, 1002, 64, 2, 64, 109, 16, 2105, 1, -5, 4, 385, 1001, 64, 1, 64,
            1105, 1, 397, 1002, 64, 2, 64, 109, -25, 2102, 1, -1, 63, 1008, 63, 26, 63, 1005, 63,
            421, 1001, 64, 1, 64, 1106, 0, 423, 4, 403, 1002, 64, 2, 64, 109, -8, 1202, 9, 1, 63,
            1008, 63, 25, 63, 1005, 63, 445, 4, 429, 1105, 1, 449, 1001, 64, 1, 64, 1002, 64, 2,
            64, 109, 5, 1207, 0, 40, 63, 1005, 63, 467, 4, 455, 1106, 0, 471, 1001, 64, 1, 64,
            1002, 64, 2, 64, 109, -6, 2107, 24, 8, 63, 1005, 63, 487, 1105, 1, 493, 4, 477, 1001,
            64, 1, 64, 1002, 64, 2, 64, 109, 15, 21107, 41, 40, 1, 1005, 1011, 509, 1106, 0, 515,
            4, 499, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 12, 1205, -1, 529, 4, 521, 1105, 1, 533,
            1001, 64, 1, 64, 1002, 64, 2, 64, 109, -20, 2102, 1, 2, 63, 1008, 63, 29, 63, 1005, 63,
            555, 4, 539, 1105, 1, 559, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 15, 1201, -9, 0, 63,
            1008, 63, 38, 63, 1005, 63, 579, 1105, 1, 585, 4, 565, 1001, 64, 1, 64, 1002, 64, 2,
            64, 109, -2, 21102, 42, 1, -3, 1008, 1012, 44, 63, 1005, 63, 609, 1001, 64, 1, 64,
            1106, 0, 611, 4, 591, 1002, 64, 2, 64, 109, -21, 2107, 29, 8, 63, 1005, 63, 629, 4,
            617, 1106, 0, 633, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 15, 1202, 0, 1, 63, 1008, 63,
            30, 63, 1005, 63, 657, 1001, 64, 1, 64, 1106, 0, 659, 4, 639, 1002, 64, 2, 64, 109, 15,
            21102, 43, 1, -8, 1008, 1016, 43, 63, 1005, 63, 681, 4, 665, 1105, 1, 685, 1001, 64, 1,
            64, 1002, 64, 2, 64, 109, -10, 21107, 44, 45, -4, 1005, 1010, 707, 4, 691, 1001, 64, 1,
            64, 1106, 0, 707, 1002, 64, 2, 64, 109, 11, 2106, 0, 2, 1001, 64, 1, 64, 1106, 0, 725,
            4, 713, 1002, 64, 2, 64, 109, -16, 21101, 45, 0, 8, 1008, 1017, 43, 63, 1005, 63, 749,
            1001, 64, 1, 64, 1105, 1, 751, 4, 731, 1002, 64, 2, 64, 109, -3, 1208, 2, 36, 63, 1005,
            63, 773, 4, 757, 1001, 64, 1, 64, 1106, 0, 773, 1002, 64, 2, 64, 109, 18, 1206, -4,
            787, 4, 779, 1105, 1, 791, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -8, 2105, 1, 7, 1001,
            64, 1, 64, 1106, 0, 809, 4, 797, 1002, 64, 2, 64, 109, -2, 21108, 46, 44, 2, 1005,
            1016, 825, 1105, 1, 831, 4, 815, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 7, 21101, 47,
            0, -8, 1008, 1013, 47, 63, 1005, 63, 857, 4, 837, 1001, 64, 1, 64, 1105, 1, 857, 1002,
            64, 2, 64, 109, -17, 1201, -4, 0, 63, 1008, 63, 24, 63, 1005, 63, 883, 4, 863, 1001,
            64, 1, 64, 1105, 1, 883, 1002, 64, 2, 64, 109, 10, 1206, 7, 895, 1106, 0, 901, 4, 889,
            1001, 64, 1, 64, 4, 64, 99, 21102, 1, 27, 1, 21102, 1, 915, 0, 1105, 1, 922, 21201, 1,
            24405, 1, 204, 1, 99, 109, 3, 1207, -2, 3, 63, 1005, 63, 964, 21201, -2, -1, 1, 21101,
            942, 0, 0, 1106, 0, 922, 22102, 1, 1, -1, 21201, -2, -3, 1, 21101, 0, 957, 0, 1106, 0,
            922, 22201, 1, -1, -2, 1106, 0, 968, 21201, -2, 0, -2, 109, -3, 2106, 0, 0,
        ];

        let mut machine = IntCodeMachine::new(&input);
        machine.provide_input(1);
        match machine.get_output() {
            None => assert_eq!(true, false),
            Some(v) => assert_eq!(v, 2436480432),
        };

        let mut machine = IntCodeMachine::new(&input);
        machine.provide_input(2);
        match machine.get_output() {
            None => assert_eq!(true, false),
            Some(v) => assert_eq!(v, 45710),
        };
    }

    #[test]
    fn test_holistic_2() {
        let input = vec![
            1102, 34463338, 34463338, 63, 1007, 63, 34463338, 63, 1005, 63, 53, 1101, 0, 3, 1000,
            109, 988, 209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65,
            1008, 1000, 2, 63, 1005, 63, 904, 1008, 1000, 0, 63, 1005, 63, 58, 4, 25, 104, 0, 99,
            4, 0, 104, 0, 99, 4, 17, 104, 0, 99, 0, 0, 1102, 1, 24, 1017, 1101, 0, 36, 1006, 1101,
            0, 30, 1011, 1101, 26, 0, 1018, 1101, 32, 0, 1015, 1101, 34, 0, 1004, 1101, 0, 37,
            1002, 1101, 25, 0, 1012, 1102, 38, 1, 1010, 1101, 29, 0, 1019, 1101, 308, 0, 1029,
            1102, 1, 696, 1027, 1102, 1, 429, 1022, 1102, 1, 21, 1005, 1102, 1, 33, 1013, 1101, 39,
            0, 1008, 1102, 20, 1, 1009, 1101, 0, 652, 1025, 1102, 313, 1, 1028, 1101, 0, 31, 1003,
            1102, 661, 1, 1024, 1101, 35, 0, 1016, 1101, 0, 23, 1000, 1102, 28, 1, 1014, 1102, 0,
            1, 1020, 1102, 27, 1, 1007, 1101, 0, 1, 1021, 1102, 22, 1, 1001, 1101, 703, 0, 1026,
            1101, 0, 422, 1023, 109, -5, 2101, 0, 9, 63, 1008, 63, 31, 63, 1005, 63, 205, 1001, 64,
            1, 64, 1105, 1, 207, 4, 187, 1002, 64, 2, 64, 109, 6, 2102, 1, 3, 63, 1008, 63, 37, 63,
            1005, 63, 227, 1105, 1, 233, 4, 213, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 11, 21108,
            40, 40, 3, 1005, 1015, 255, 4, 239, 1001, 64, 1, 64, 1106, 0, 255, 1002, 64, 2, 64,
            109, -3, 21107, 41, 40, 2, 1005, 1011, 275, 1001, 64, 1, 64, 1105, 1, 277, 4, 261,
            1002, 64, 2, 64, 109, 4, 2107, 28, -6, 63, 1005, 63, 297, 1001, 64, 1, 64, 1106, 0,
            299, 4, 283, 1002, 64, 2, 64, 109, 15, 2106, 0, 0, 4, 305, 1106, 0, 317, 1001, 64, 1,
            64, 1002, 64, 2, 64, 109, -23, 2108, 22, 4, 63, 1005, 63, 337, 1001, 64, 1, 64, 1105,
            1, 339, 4, 323, 1002, 64, 2, 64, 109, 6, 21101, 42, 0, 0, 1008, 1011, 40, 63, 1005, 63,
            363, 1001, 64, 1, 64, 1105, 1, 365, 4, 345, 1002, 64, 2, 64, 109, -17, 1207, 7, 21, 63,
            1005, 63, 381, 1105, 1, 387, 4, 371, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 14, 1201,
            -1, 0, 63, 1008, 63, 25, 63, 1005, 63, 407, 1105, 1, 413, 4, 393, 1001, 64, 1, 64,
            1002, 64, 2, 64, 109, 15, 2105, 1, 0, 1001, 64, 1, 64, 1105, 1, 431, 4, 419, 1002, 64,
            2, 64, 109, -23, 2101, 0, 6, 63, 1008, 63, 36, 63, 1005, 63, 453, 4, 437, 1106, 0, 457,
            1001, 64, 1, 64, 1002, 64, 2, 64, 109, 10, 2108, 21, -5, 63, 1005, 63, 475, 4, 463,
            1106, 0, 479, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -3, 1201, 2, 0, 63, 1008, 63, 20,
            63, 1005, 63, 505, 4, 485, 1001, 64, 1, 64, 1105, 1, 505, 1002, 64, 2, 64, 109, 4,
            2107, 35, -5, 63, 1005, 63, 527, 4, 511, 1001, 64, 1, 64, 1105, 1, 527, 1002, 64, 2,
            64, 109, 15, 1206, -5, 543, 1001, 64, 1, 64, 1105, 1, 545, 4, 533, 1002, 64, 2, 64,
            109, -8, 1205, 3, 563, 4, 551, 1001, 64, 1, 64, 1106, 0, 563, 1002, 64, 2, 64, 109, -5,
            1206, 7, 581, 4, 569, 1001, 64, 1, 64, 1105, 1, 581, 1002, 64, 2, 64, 109, -8, 1207,
            -3, 38, 63, 1005, 63, 599, 4, 587, 1105, 1, 603, 1001, 64, 1, 64, 1002, 64, 2, 64, 109,
            19, 1205, -4, 619, 1001, 64, 1, 64, 1105, 1, 621, 4, 609, 1002, 64, 2, 64, 109, -13,
            1208, -4, 27, 63, 1005, 63, 639, 4, 627, 1105, 1, 643, 1001, 64, 1, 64, 1002, 64, 2,
            64, 109, 5, 2105, 1, 8, 4, 649, 1001, 64, 1, 64, 1106, 0, 661, 1002, 64, 2, 64, 109,
            -16, 1202, 4, 1, 63, 1008, 63, 34, 63, 1005, 63, 683, 4, 667, 1106, 0, 687, 1001, 64,
            1, 64, 1002, 64, 2, 64, 109, 26, 2106, 0, 1, 1001, 64, 1, 64, 1105, 1, 705, 4, 693,
            1002, 64, 2, 64, 109, -9, 21102, 43, 1, -7, 1008, 1010, 46, 63, 1005, 63, 725, 1105, 1,
            731, 4, 711, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -26, 1202, 9, 1, 63, 1008, 63, 26,
            63, 1005, 63, 755, 1001, 64, 1, 64, 1105, 1, 757, 4, 737, 1002, 64, 2, 64, 109, 34,
            21108, 44, 43, -8, 1005, 1017, 773, 1106, 0, 779, 4, 763, 1001, 64, 1, 64, 1002, 64, 2,
            64, 109, -15, 21102, 45, 1, 1, 1008, 1011, 45, 63, 1005, 63, 801, 4, 785, 1106, 0, 805,
            1001, 64, 1, 64, 1002, 64, 2, 64, 109, -14, 1208, 10, 35, 63, 1005, 63, 821, 1106, 0,
            827, 4, 811, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 17, 2102, 1, -4, 63, 1008, 63, 20,
            63, 1005, 63, 853, 4, 833, 1001, 64, 1, 64, 1106, 0, 853, 1002, 64, 2, 64, 109, 6,
            21107, 46, 47, -4, 1005, 1015, 871, 4, 859, 1105, 1, 875, 1001, 64, 1, 64, 1002, 64, 2,
            64, 109, -10, 21101, 47, 0, 4, 1008, 1013, 47, 63, 1005, 63, 901, 4, 881, 1001, 64, 1,
            64, 1105, 1, 901, 4, 64, 99, 21102, 27, 1, 1, 21102, 1, 915, 0, 1106, 0, 922, 21201, 1,
            37790, 1, 204, 1, 99, 109, 3, 1207, -2, 3, 63, 1005, 63, 964, 21201, -2, -1, 1, 21102,
            1, 942, 0, 1106, 0, 922, 22102, 1, 1, -1, 21201, -2, -3, 1, 21102, 957, 1, 0, 1105, 1,
            922, 22201, 1, -1, -2, 1105, 1, 968, 21201, -2, 0, -2, 109, -3, 2105, 1, 0,
        ];

        let mut machine = IntCodeMachine::new(&input);
        machine.provide_input(1);
        match machine.get_output() {
            None => assert_eq!(true, false),
            Some(v) => assert_eq!(v, 2671328082),
        };

        let mut machine = IntCodeMachine::new(&input);
        machine.provide_input(2);
        match machine.get_output() {
            None => assert_eq!(true, false),
            Some(v) => assert_eq!(v, 59095),
        };
    }
}
