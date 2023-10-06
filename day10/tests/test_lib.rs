use day10::*;

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

const LARGE_EXAMPLE: &str = "
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

#[test]
fn test_signal_strengths_larger_example() {
    let input = LARGE_EXAMPLE;
    let computer = Computer::new(parse_instructions(input));
    let signal_strengths: Vec<i32> = SignalStrengths::new(computer).collect();
    assert_eq!(signal_strengths[19], 420);
    assert_eq!(signal_strengths[59], 1140);
    assert_eq!(signal_strengths[99], 1800);
    assert_eq!(signal_strengths[139], 2940);
    assert_eq!(signal_strengths[179], 2880);
    assert_eq!(signal_strengths[219], 3960);
}

#[test]
fn test_part_2() {
    assert_eq!(
        part_2(LARGE_EXAMPLE),
        "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        .trim_start()
    );
}
