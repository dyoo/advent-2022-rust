pub enum Instruction {
    NoOp,
    AddX(i32),
}

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

    fn load_instruction(&mut self) -> bool {
        if self.in_flight.is_none() {
            if self.program_counter >= self.program.len() {
                return false;
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

            true
        } else {
            true
        }
    }

    fn execute_instruction(&mut self) {
        match &mut self.in_flight {
            Some(running) => match running {
                RunningInstruction::NoOp => {
                    self.in_flight.take();
                }
                RunningInstruction::AddX {
                    delay: cycles_left,
                    delta,
                } => {
                    if *cycles_left != 0 {
                        *cycles_left -= 1;
                    } else {
                        self.x += *delta;
                        self.in_flight.take();
                    }
                }
            },
            None => {}
        }
    }

    pub fn tick(&mut self) -> bool {
        if !self.load_instruction() {
            return false;
        }

        self.execute_instruction();
        true
    }
}

struct SignalStrengths(usize, Computer);

impl SignalStrengths {
    fn new(computer: Computer) -> Self {
        Self(0, computer)
    }
}

impl Iterator for SignalStrengths {
    type Item = (usize, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        let is_active = self.1.tick();
        if is_active {
            Some((self.0, self.1.x * (self.0 as i32)))
        } else {
            None
        }
    }
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
    let signal_strengths: Vec<(usize, i32)> = SignalStrengths::new(computer).collect();

    assert_eq!(
        signal_strengths,
        vec![(1, 1), (2, 2), (3, 12), (4, 16), (5, -5)],
    );
}

fn main() {
    println!("Hello, world!");
}
