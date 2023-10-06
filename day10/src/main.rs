#[derive(Debug, PartialEq)]
pub enum Instruction {
    NoOp,
    AddX(i32),
}

#[derive(Debug, PartialEq)]
pub enum RunningInstruction {
    NoOp,
    AddX { delay: usize, delta: i32 },
}

pub struct Computer {
    pub x: i32,

    program: Vec<Instruction>,
    program_counter: usize,

    in_flight: Option<RunningInstruction>,
}

impl Computer {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            x: 1,
            program,
            program_counter: 0,
            in_flight: None,
        }
    }

    fn load_instruction(&mut self) {
        if self.in_flight.is_none() {
            if self.program_counter >= self.program.len() {
                return;
            }
            let next_instruction = &self.program[self.program_counter];
            self.program_counter += 1;

            self.in_flight = Some(match next_instruction {
                Instruction::NoOp => RunningInstruction::NoOp,
                Instruction::AddX(delta) => RunningInstruction::AddX {
                    delay: 1,
                    delta: *delta,
                },
            });
        }
    }

    fn execute_instruction(&mut self) {
        match self.in_flight.as_mut().unwrap() {
            RunningInstruction::NoOp => {
                self.in_flight.take();
            }
            RunningInstruction::AddX { delay, delta } => {
                if *delay == 0 {
                    self.x += *delta;
                    self.in_flight.take();
                } else {
                    *delay -= 1;
                }
            }
        }
    }

    pub fn tick(&mut self) -> bool {
        self.load_instruction();

	if self.in_flight.is_some() {
            self.execute_instruction();
            true
        } else {
            false
        }
    }
}

struct SignalStrengths {
    cycles_executed: usize,
    computer: Computer,
    exhausted: bool,
}

impl SignalStrengths {
    fn new(computer: Computer) -> Self {
        Self {
            cycles_executed: 0,
            computer,
            exhausted: false,
        }
    }
}

impl Iterator for SignalStrengths {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }

        let result = Some((self.cycles_executed as i32 + 1) * self.computer.x);

        self.exhausted = !self.computer.tick();
        self.cycles_executed += 1;

        result
    }
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    s.lines()
        .filter_map(
            |line| match line.split_whitespace().collect::<Vec<&str>>()[..] {
                ["addx", n] => n
                    .parse::<i32>()
                    .map(|v| Some(Instruction::AddX(v)))
                    .unwrap_or(None),
                ["noop"] => Some(Instruction::NoOp),
                _ => None,
            },
        )
        .collect()
}

#[test]
fn test_parse_small_program() {
    let input = "
noop
addx 3
addx -5
";
    assert_eq!(
        parse_instructions(input),
        vec![
            Instruction::NoOp,
            Instruction::AddX(3),
            Instruction::AddX(-5),
        ]
    );
}

#[test]
fn test_small_program() {
    let mut computer = Computer::new(vec![
        Instruction::NoOp,
        Instruction::AddX(3),
        Instruction::AddX(-5),
    ]);
    assert!(computer.tick());
    assert_eq!(computer.x, 1);
    assert!(computer.tick());
    assert_eq!(computer.x, 1);
    assert!(computer.tick());
    assert_eq!(computer.x, 4);
    assert!(computer.tick());
    assert_eq!(computer.x, 4);
    assert!(computer.tick());
    assert_eq!(computer.x, -1);

    assert!(!computer.tick());
}

#[test]
fn test_signal_strengths() {
    let computer = Computer::new(vec![
        Instruction::NoOp,
        Instruction::AddX(3),
        Instruction::AddX(-5),
    ]);
    let signal_strengths: Vec<i32> = SignalStrengths::new(computer).collect();

    assert_eq!(signal_strengths, vec![1, 2, 3, 16, 20, -6],);
}

#[test]
fn test_signal_strengths_larger_example() {
    let input = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
    let mut computer = Computer::new(parse_instructions(input));
    let signal_strengths: Vec<i32> = SignalStrengths::new(computer).collect();
    assert_eq!(signal_strengths[19], 420);
    assert_eq!(signal_strengths[59], 1140);
    assert_eq!(signal_strengths[99], 1800);
    assert_eq!(signal_strengths[139], 2940);
    assert_eq!(signal_strengths[179], 2880);
    assert_eq!(signal_strengths[219], 3960);
}

fn part_1(s: &str) -> i32 {
    let computer = Computer::new(parse_instructions(s));
    let signal_strengths: Vec<i32> = SignalStrengths::new(computer).collect();
    signal_strengths[19] + 
	signal_strengths[59] + 
	signal_strengths[99] + 
	signal_strengths[139] +
	signal_strengths[179] + 
	signal_strengths[219]
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let input = std::fs::read_to_string("adventofcode.com_2022_day_10_input.txt")?;
    
    println!("part 1: {}", part_1(&input));

    Ok(())
}
