type CpuMethod = Box<Fn(&mut Cpu, u32, u32, u32)>;

const R5_VALUE: usize = 10551329;

fn main() {
    let input = include_str!("input.txt").trim();

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

    let mut program = Vec::new();

    let mut bound = None;

    for (i, l) in input.lines().enumerate() {
        let mut parts = l.split_whitespace().collect::<Vec<&str>>();

        if i == 0 {
            bound = Some(str::parse::<usize>(parts[1]).unwrap());
        } else {
            parts[0] = match parts[0] {
                "addr" => "4",
                "addi" => "9",
                "mulr" => "11",
                "muli" => "13",
                "banr" => "8",
                "bani" => "15",
                "borr" => "1",
                "bori" => "12",
                "setr" => "10",
                "seti" => "5",
                "gtir" => "2",
                "gtri" => "7",
                "gtrr" => "0",
                "eqir" => "14",
                "eqri" => "3",
                "eqrr" => "6",
                _ => panic!(),
            };
            program.push(
                parts
                    .iter()
                    .map(|s| str::parse::<u32>(s).unwrap())
                    .collect(),
            );
        }
    }

    let mut cpu = Cpu::new(bound.unwrap(), program);

    while cpu.execute_next(&cpu_methods) {}

    // part 2; sum of divisors

 	// for r_3 in r3..=r5 {
	// 	for r_1 in r3..=r5 {
	// 		if (r_3 * r_1) == r[5] {
	// 			r[0] += r_3;
	// 		}
	// 	}
	// }

    let mut sum = 0;

    for r_3 in 1..=R5_VALUE {
        if R5_VALUE % r_3 == 0 {
            sum += r_3;
        }
    }

    println!("{}", sum);

    println!("{}", cpu.registers[0]);
}

struct Cpu {
    ip: usize,
    bound: usize,
    registers: [u32; 6],
    program: Vec<Vec<u32>>,
}

impl Cpu {
    fn new(bound: usize, program: Vec<Vec<u32>>) -> Cpu {
        Cpu {
            ip: 0,
            bound,
            program,
            registers: [0; 6],
        }
    }

    fn execute_next(&mut self, cpu_methods: &[CpuMethod]) -> bool {
        self.registers[self.bound] = self.ip as u32;

        let instruction = &self.program[self.ip];
        let op = instruction[0];
        let (a, b, c) = (instruction[1], instruction[2], instruction[3]);

        cpu_methods[match op {
            0 => 12,
            1 => 6,
            2 => 10,
            3 => 14,
            4 => 0,
            5 => 9,
            6 => 15,
            7 => 11,
            8 => 4,
            9 => 1,
            10 => 8,
            11 => 2,
            12 => 7,
            13 => 3,
            14 => 13,
            15 => 5,
            _ => unreachable!(),
        }](self, a, b, c);

        self.ip = self.registers[self.bound] as usize;
        self.ip += 1;

        self.ip < self.program.len()
    }

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
