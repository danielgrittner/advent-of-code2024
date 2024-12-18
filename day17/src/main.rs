use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

#[derive(Debug, Clone)]
struct Machine {
    registers: Registers,
    program: Vec<u8>,
}

impl Machine {
    fn new(a: u64, b: u64, c: u64, program: Vec<u8>) -> Self {
        Self {
            registers: Registers { a, b, c },
            program,
        }
    }

    fn combo_operand(&self, combo_operand: u8, registers: Registers) -> u64 {
        match combo_operand {
            0 | 1 | 2 | 3 => combo_operand as u64,
            4 => registers.a,
            5 => registers.b,
            6 => registers.c,
            _ => panic!("Invalid combo operand"),
        }
    }

    fn run_program(&self) -> Vec<u64> {
        let mut isp = 0;
        let mut out = Vec::new();

        let mut regs = self.registers;

        while isp < self.program.len() {
            let opcode = self.program[isp];

            match opcode {
                0 => {
                    // adv
                    regs.a = regs.a / (1 << self.combo_operand(self.program[isp + 1], regs));
                }
                1 => {
                    // bxl
                    regs.b ^= self.program[isp + 1] as u64;
                }
                2 => {
                    // bst
                    regs.b = self.combo_operand(self.program[isp + 1], regs) % 8;
                }
                3 => {
                    // jnz
                    if regs.a != 0 {
                        isp = self.program[isp + 1] as usize;
                        continue;
                    }
                }
                4 => {
                    // bxc
                    regs.b ^= regs.c;
                }
                5 => {
                    // out
                    out.push(self.combo_operand(self.program[isp + 1], regs) % 8);
                }
                6 => {
                    // bdv
                    regs.b = regs.a / (1 << self.combo_operand(self.program[isp + 1], regs));
                }
                7 => {
                    // cdv
                    regs.c = regs.a / (1 << self.combo_operand(self.program[isp + 1], regs));
                }
                _ => panic!("Invalid opcode"),
            }

            isp += 2;
        }

        out
    }
}

fn read_data(path: &str) -> std::io::Result<Machine> {
    let txt = read_to_string(path)?;

    let re = Regex::new(r"Register A: (?<a>\d+)\nRegister B: (?<b>\d+)\nRegister C: (?<c>\d+)\n\nProgram: (?<program>\d+(,\d+)*)").unwrap();
    let captures = re.captures(&txt).unwrap();

    let a = captures["a"].parse::<u64>().unwrap();
    let b = captures["b"].parse::<u64>().unwrap();
    let c = captures["c"].parse::<u64>().unwrap();
    let program = captures["program"]
        .split(",")
        .map(|opcode| opcode.parse::<u8>().unwrap())
        .collect();

    Ok(Machine::new(a, b, c, program))
}

/*

8 instructions

=> 3bit number => 3bit number after it as input

=> ISP increased by 2



Combo Operand:

- 0 to 3 => literal values
- 4,5,6 => A,B,C
- 7 => nothing


Instructions:
- 0 -> adv: A := A / 2^combo operand => result is truncated to an int
- 1 -> bxl: bitwise XOR  B := B ^ literal combo operand
- 2 -> bst: B := combo operand % 8
- 3 -> jnz: if A == 0 then do nothing and ISP += 2 else ISP := literal operand
- 4 -> bxc: B := B ^ C, note ignores operand
- 5 -> out: output combo operand => comma separated
- 6 -> bdv: B := A / 2^combo operand => result is truncated to an int
- 7 -> cdv: C := A / 2^combo operand => result is truncated to an int




Program:

2,4,1,5,7,5,1,6,0,3,4,2,5,5,3,0


bst 4
bxl 5
cdv 5
bxl 6
adv 3
bxc 2
out 5
jnz 0




bst 4   B := A % 8
bxl 5   B := B ^ 5
cdv 5   C := A // (1 << B)
bxl 6   B := B ^ 6
adv 3   A := A // (1 << 3) = A // 8
bxc 2   B := B ^ C
out 5   print(B % 8)
jnz 0   if A != 0 goto start


while A > 0:
    B := A % 8
    B := B ^ 5
    C := A // (1 << B)
    B := B ^ 6
    A := A // 8
    B := B ^ C
    print(B % 8)


Key observation: for one iteration we need to look at the next 10 bits!!!
=> init the next 10 bits
    
*/

fn dfs(init_a_reg: u64, idx: usize, program: &Vec<u8>, results: &mut Vec<u64>) {
    if idx == program.len() {
        results.push(init_a_reg);
        return;
    }

    for a_3bit in 0..8 {
        let init_a_reg2 = init_a_reg | (a_3bit << (3 * idx + 7));

        let a = init_a_reg2 >> (3 * idx);

        let b = a % 8;
        let b = b ^ 5;
        let c = a >> b;
        let b = b ^ 6;
        let b = b ^ c;
        let b = b % 8;

        if b == program[idx] as u64 {
            dfs(init_a_reg2, idx + 1, program, results);
        }
    }
}

fn main() -> std::io::Result<()> {
    // let data = read_data("example.txt")?;
    let data = read_data("input.txt")?;

    let start = std::time::Instant::now();
    let task1 = data
        .run_program()
        .into_iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let duration = start.elapsed();
    println!("Task 1: {task1} (took {:?})", duration);

    let mut results = Vec::new();
    for a in 0..(1 << 10) {
        let b = a % 8;
        let b = b ^ 5;
        let c = a >> b; // remove the bottom 7 bits
        let b = b ^ 6;
        let b = b ^ c;
        let b = b % 8;

        // Key observation: for one iteration we need to look at the next 10 bits!!!
        // => init the next 10 bits

        if b == 2 {
            dfs(a, 1, &data.program, &mut results);
        }
    }
    let task2 = results.into_iter().min().unwrap();
    println!("Task 2: {task2}");

    Ok(())
}
