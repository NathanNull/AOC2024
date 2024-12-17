use std::collections::HashSet;

use crate::helpers::first_n;

// bst 4 -> b = first 3 bits of a (b=a3,a2,a1)
// bxl 2 -> b flip 2's bit (b=a3,!a2,a1)
// cdv 5 -> c = a >> b (c=...a7,a6,a5)
// bxl 7 -> b flip first 3 bits (b=!a3,a2,!a1)
// bxc 4 -> b xor c (b=...!a3^a7,a2^a6,!a1^a5)
// adv 3 -> a >> 3
// out 5 -> print first 3 bits of b (must be 0,0,0)
// therefore a3=!a7, a2=!a6, a1=!a5
// second run (jnz 0):
// bst 4 -> b = first 3 bits of a (b=a6,a5,a4)
// bxl 2 -> b flip 2's bit (b=a6,!a5,a4)
// cdv 5 -> c = a >> 5 (c=an...a9)
// bxl 7 -> b flip first 3 bits (b=!a6,a5,!a4)
// bxc 4 -> b xor c (b=...!a6^a11,a5^a10,!a4^a9)
// adv 3 -> a >> 3
// out 5 -> print first 3 bits of b (must be 1,0,0)
// therefore !a1=a6=a11,a5=a10,a4=!a9,

pub fn main(input: String, pt1: bool) {
    let [register_input, program_input] = first_n(&mut input.split("\r\n\r\n"));
    let registers = register_input
        .lines()
        .map(|l| l.split(": ").nth(1).unwrap().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let ops = program_input
        .split(": ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    if pt1 {
        println!(
            "Program output is {}",
            run_program(registers, &ops)
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
    } else {
        // Given the way the input is (it ends with "output b, shift a 3 to the right, jnz 0")
        // an a value 0bXXXXX that gives an output of a,b,c can be modified into 0bXXXXXYYY
        // which will always output exactly n,a,b,c for some unknown n.
        // Knowing this, we can try all possible values of YYY to solve for the n we want,
        // which will never disturb any of the outputs after it. We solve for each output
        // in reverse order to take full advantage of this.
        let mut last_valid = HashSet::new();
        for t in 0..512 { // this is probably excessive. ah well.
            last_valid.insert(t);
        }
        for i_rev in 0..ops.len() {
            let mut curr_valid = HashSet::new();
            let i = ops.len() - i_rev - 1;
            for poss in last_valid {
                for test in 0..8 {
                    let test_a = (poss << 3) + test;
                    let output = run_program(vec![test_a, 0, 0], &ops);
                    for idx in i + 1..ops.len() {
                        assert!(
                            output.get(idx - i).is_some_and(|o| *o == ops[idx]),
                            "idx: {i}, i_rev: {i_rev}, test_a: {test_a}, out: {output:?}"
                        );
                    }
                    //println!("Testing {test_a} {output:?} for {}", ops[i]);
                    if output.get(0).is_some_and(|o| *o == ops[i]) {
                        curr_valid.insert(test_a);
                    }
                }
            }
            last_valid = curr_valid;
            println!(
                "Finished solving for {i_rev}, {} possibilities",
                last_valid.len()
            );
        }
        let min_output = last_valid.iter().min().unwrap();
        println!("Minimum cat output is {min_output}");
        assert_eq!(run_program(vec![*min_output, 0, 0], &ops), ops);
    }
}

fn run_program(mut registers: Vec<usize>, ops: &Vec<usize>) -> Vec<usize> {
    let mut ins_ptr = 0;
    let mut output = vec![];
    loop {
        if let Some(&op) = ops.get(ins_ptr) {
            if let Some(&lit) = ops.get(ins_ptr + 1) {
                // print!(
                //     "{} {lit}",
                //     ["adv", "bxl", "bst", "jnz", "bxc", "out", "bdv", "cdv"][op]
                // );
                match op {
                    0 => registers[0] = registers[0] / 2usize.pow(combo(lit, &registers) as u32),
                    1 => registers[1] = registers[1] ^ lit,
                    2 => registers[1] = combo(lit, &registers) % 8,
                    3 => {
                        if registers[0] != 0 {
                            ins_ptr = lit as usize;
                            continue;
                        }
                    }
                    4 => registers[1] = registers[1] ^ registers[2],
                    5 => output.push(combo(lit, &registers) % 8),
                    6 => registers[1] = registers[0] / 2usize.pow(combo(lit, &registers) as u32),
                    7 => registers[2] = registers[0] / 2usize.pow(combo(lit, &registers) as u32),
                    _ => panic!("what"),
                }
                // println!(
                //     ", Regs: {}",
                //     registers
                //         .iter()
                //         .map(|n| format!("{:b}", n))
                //         .collect::<Vec<_>>()
                //         .join(",")
                // );
            } else {
                break;
            }
        } else {
            break;
        }
        ins_ptr += 2;
    }
    output
}

fn combo(lit: usize, regs: &Vec<usize>) -> usize {
    match lit {
        0..=3 => lit,
        4..=6 => regs[(lit - 4) as usize],
        _ => panic!("What the hecc"),
    }
}
