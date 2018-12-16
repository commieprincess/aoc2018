use regex::Regex;

type CpuMethod = Box<Fn(&mut Cpu, u32, u32, u32)>;

fn main() {
    let input = include_str!("input.txt").trim();

    let input_regex = Regex::new(r"Before:\s+\[(\d+),\s+(\d+),\s+(\d+),\s+(\d+)\]\n(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\nAfter:\s+\[(\d+),\s+(\d+),\s+(\d+),\s+(\d+)\]").unwrap();

    let mut samples = Vec::new();

    for c in input_regex.captures_iter(&input) {
        let mut new_sample = Sample {
            start_state: [0; 4],
            operation: [0; 4],
            end_state: [0; 4],
        };

        for (i, n) in c.iter().skip(1).enumerate() {
            let number = str::parse::<u32>(n.unwrap().as_str()).unwrap();

            match i {
                0..=3 => new_sample.start_state[i] = number,
                4..=7 => new_sample.operation[i - 4] = number,
                8..=11 => new_sample.end_state[i - 8] = number,
                _ => unreachable!(),
            }
        }

        samples.push(new_sample);
    }

    let cpu_methods: Vec<CpuMethod> = vec![
        Box::new(Cpu::addr),
        Box::new(Cpu::addi),
        Box::new(Cpu::mulr),
        Box::new(Cpu::muli),
        Box::new(Cpu::banr),
        Box::new(Cpu::bani),
        Box::new(Cpu::borr),
        Box::new(Cpu::bori),
        Box::new(Cpu::setr),
        Box::new(Cpu::seti),
        Box::new(Cpu::gtir),
        Box::new(Cpu::gtri),
        Box::new(Cpu::gtrr),
        Box::new(Cpu::eqir),
        Box::new(Cpu::eqri),
        Box::new(Cpu::eqrr),
    ];

    let mut potential_opcodes_for_samples = vec![vec![]; samples.len()];
    let mut opcode_ops = vec![vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]; 16];

    let mut cpu = Cpu { registers: [0; 4] };

    for (i, s) in samples.iter().enumerate() {
        let mut possible_ops = vec![];

        for (j, o) in cpu_methods.iter().enumerate() {
            cpu.registers = s.start_state;
            o(&mut cpu, s.operation[1], s.operation[2], s.operation[3]);

            if cpu.registers == s.end_state {
                potential_opcodes_for_samples[i].push(j);
                possible_ops.push(j);
            }
        }

        opcode_ops[s.operation[0] as usize].retain(|v| possible_ops.contains(v));
    }

    println!(
        "part 1: {}",
        potential_opcodes_for_samples.iter().filter(|v| v.len() >= 3).count()
    );

    let mut actual_opcodes: Vec<Option<u32>> = vec![None; 16];

    while actual_opcodes.iter().any(|v| v.is_none()) {
        for v in opcode_ops.iter_mut() {
            v.retain(|potential_opcode| {
                !actual_opcodes
                    .iter()
                    .any(|opcode| opcode.is_some() && opcode.unwrap() == *potential_opcode as u32)
            });
        }

        for (i, n) in opcode_ops.iter().enumerate() {
            if n.len() == 1 {
                actual_opcodes[i] = Some(n[0] as u32);
            }
        }
    }

    let opcodes: Vec<u32> = actual_opcodes.iter().map(|v| v.unwrap()).collect();

    cpu = Cpu { registers: [0; 4] };

    for l in include_str!("test_program.txt").lines() {
        let values = l
            .split_whitespace()
            .map(|v| str::parse::<u32>(v).unwrap())
            .collect::<Vec<u32>>();

        cpu_methods[opcodes[values[0] as usize] as usize](
            &mut cpu, values[1], values[2], values[3],
        );
    }

    println!("part 2: {}", cpu.registers[0]);
}

#[derive(Debug)]
struct Sample {
    start_state: [u32; 4],
    operation: [u32; 4],
    end_state: [u32; 4],
}

struct Cpu {
    registers: [u32; 4],
}

impl Cpu {
    fn addr(&mut self, a: u32, b: u32, c: u32) {
        self.registers[c as usize] = self.registers[a as usize] + self.registers[b as usize];
    }

    fn addi(&mut self, a: u32, b: u32, c: u32) {
        self.registers[c as usize] = self.registers[a as usize] + b;
    }

    fn mulr(&mut self, a: u32, b: u32, c: u32) {
        self.registers[c as usize] = self.registers[a as usize] * self.registers[b as usize];
    }

    fn muli(&mut self, a: u32, b: u32, c: u32) {
        self.registers[c as usize] = self.registers[a as usize] * b;
    }

    fn banr(&mut self, a: u32, b: u32, c: u32) {
        self.registers[c as usize] = self.registers[a as usize] & self.registers[b as usize];
    }

    fn bani(&mut self, a: u32, b: u32, c: u32) {
        self.registers[c as usize] = self.registers[a as usize] & b;
    }

    fn borr(&mut self, a: u32, b: u32, c: u32) {
        self.registers[c as usize] = self.registers[a as usize] | self.registers[b as usize];
    }

    fn bori(&mut self, a: u32, b: u32, c: u32) {
        self.registers[c as usize] = self.registers[a as usize] | b;
    }

    fn setr(&mut self, a: u32, _b: u32, c: u32) {
        self.registers[c as usize] = self.registers[a as usize];
    }

    fn seti(&mut self, a: u32, _b: u32, c: u32) {
        self.registers[c as usize] = a;
    }

    fn gtir(&mut self, a: u32, b: u32, c: u32) {
        self.registers[c as usize] = if a > self.registers[b as usize] { 1 } else { 0 };
    }

    fn gtri(&mut self, a: u32, b: u32, c: u32) {
        self.registers[c as usize] = if self.registers[a as usize] > b { 1 } else { 0 };
    }

    fn gtrr(&mut self, a: u32, b: u32, c: u32) {
        self.registers[c as usize] = if self.registers[a as usize] > self.registers[b as usize] {
            1
        } else {
            0
        };
    }

    fn eqir(&mut self, a: u32, b: u32, c: u32) {
        self.registers[c as usize] = if a == self.registers[b as usize] {
            1
        } else {
            0
        };
    }

    fn eqri(&mut self, a: u32, b: u32, c: u32) {
        self.registers[c as usize] = if self.registers[a as usize] == b {
            1
        } else {
            0
        };
    }

    fn eqrr(&mut self, a: u32, b: u32, c: u32) {
        self.registers[c as usize] = if self.registers[a as usize] == self.registers[b as usize] {
            1
        } else {
            0
        };
    }
}
