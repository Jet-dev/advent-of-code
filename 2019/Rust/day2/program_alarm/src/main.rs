use std::fs;

fn main() {
    let mut memory = load_input();

    init_memory(&mut memory, 12, 2);

    println!("{0}", process_program(memory).get(0).unwrap());
}

fn load_input() -> Vec<usize>{
    return fs::read_to_string("src/intcode.in")
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();
}

fn init_memory(memory: &mut Vec<usize>, input_1: usize, input_2: usize) {
    memory[1] = input_1;
    memory[2] = input_2;
}

fn process_program(mut memory: Vec<usize>) -> Vec<usize> {
    for index in (0..memory.len()).step_by(4) {
        if index + 3 < memory.len() {
            memory = process_instruction(memory, index);
        }
    }
    memory
}

fn process_instruction(mut memory: Vec<usize>, address: usize) -> Vec<usize> {
    let opcode = memory[address];
    let parameter_1 = memory[memory[address + 1]];
    let parameter_2 = memory[memory[address + 2]];
    let parameter_3 = memory[address + 3];

    match opcode {
        1 => memory[parameter_3] = parameter_1 + parameter_2,
        2 => memory[parameter_3] = parameter_1 * parameter_2,
        99 => return memory,
        _ => println!("UNKNOWN OPCODE FOUND : {}", opcode)
    }

    memory
}


#[cfg(test)]
mod tests {
    use crate::{process_instruction, process_program};

    #[test]
    fn process_line_addition_test() {
        let program:  Vec<usize> = vec![1, 0, 0, 0, 99];
        let expected: Vec<usize> = vec![2, 0, 0, 0, 99];

        assert_eq!(expected, process_instruction( program, 0))
    }

    #[test]
    fn process_line_multiple_test() {
        let program: Vec<usize> = vec![2, 3, 0, 3, 99];
        let expected: Vec<usize> = vec![2, 3, 0, 6, 99];

        assert_eq!(expected, process_instruction( program, 0))
    }

    #[test]
    fn process_line_multiple_further_test() {
        let program: Vec<usize> = vec![2, 4, 4, 5, 99, 0];
        let expected: Vec<usize> = vec![2, 4, 4, 5, 99, 9801];

        assert_eq!(expected, process_instruction( program, 0))
    }

    #[test]
    fn process_line_multiple_full_test() {
        let program: Vec<usize> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected: Vec<usize> = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        assert_eq!(expected, process_program(program))
    }
}
