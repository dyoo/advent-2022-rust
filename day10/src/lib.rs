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

pub struct SignalStrengths {
    cycles_executed: usize,
    computer: Computer,
    exhausted: bool,
}

impl SignalStrengths {
    pub fn new(computer: Computer) -> Self {
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

pub fn parse_instructions(s: &str) -> Vec<Instruction> {
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

// Computing signal strength sums.
pub fn part_1(s: &str) -> i32 {
    let computer = Computer::new(parse_instructions(s));
    let signal_strengths: Vec<i32> = SignalStrengths::new(computer).collect();
    signal_strengths[19]
        + signal_strengths[59]
        + signal_strengths[99]
        + signal_strengths[139]
        + signal_strengths[179]
        + signal_strengths[219]
}

// Simulating CRT.
pub fn part_2(s: &str) -> String {
    let mut result = String::new();
    let mut computer = Computer::new(parse_instructions(s));
    for _row in 0..6 {
        for col in 0..40 {
            if computer.x.abs_diff(col) <= 1 {
                result.push('#');
            } else {
                result.push('.');
            }
            computer.tick();
        }
        result.push('\n');
    }
    result
}
