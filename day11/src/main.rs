use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Multiply(Operand),
    Add(Operand),
}

#[derive(Debug, PartialEq, Clone)]
enum Operand {
    Const(u64),
    Old,
}

#[derive(Debug, PartialEq)]
struct Monkey {
    id: usize,
    starting_items: Vec<u64>,
    operation: Operation,
    divisible_by_test: u64,
    true_throw_to: usize,
    false_throw_to: usize,
}

#[derive(Debug, PartialEq)]
struct MonkeyDynamics {
    items: Vec<u64>,
    count_inspected: usize,
}

impl Monkey {
    fn get_dynamics(&self) -> MonkeyDynamics {
        MonkeyDynamics {
            items: self.starting_items.clone(),
            count_inspected: 0,
        }
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let id: usize = match lines
            .next()
            .ok_or(String::from("missing id"))?
            .split(&[' ', ':'])
            .collect::<Vec<_>>()[..]
        {
            ["Monkey", n, ""] => n.parse().map_err(|_| String::from("missing id"))?,
            _ => {
                return Err("missing id".into());
            }
        };

        let starting_items: Vec<u64> = match lines
            .next()
            .ok_or(String::from("Missing starting item line"))?
            .trim_start()
            .split(&[' ', ','])
            .collect::<Vec<_>>()[..]
        {
            ["Starting", "items:", ref items @ ..] => items
                .iter()
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<_>>(),
            ref vs => {
                return Err(format!("starting item: {:?}", vs));
            }
        };

        let operation: Operation = match lines
            .next()
            .ok_or(String::from("operation missing line"))?
            .split_whitespace()
            .collect::<Vec<_>>()[..]
        {
            ["Operation:", "new", "=", "old", op, v] => {
                let v = if v == "old" {
                    Operand::Old
                } else {
                    v.parse()
                        .map(Operand::Const)
                        .map_err(|_| format!("parsing number {:?}", v))?
                };
                if op == "*" {
                    Operation::Multiply(v)
                } else if op == "+" {
                    Operation::Add(v)
                } else {
                    return Err(format!("unknown operator {}", op));
                }
            }
            ref vs => {
                return Err(format!("operation malformed {:?}", vs));
            }
        };

        let divisible_by_test: u64 = match lines
            .next()
            .ok_or(String::from("divisible by missing"))?
            .split_whitespace()
            .collect::<Vec<_>>()[..]
        {
            ["Test:", "divisible", "by", v] => {
                v.parse().map_err(|_| format!("divisible by: {:?}", v))?
            }
            ref vs => {
                return Err(format!("divisible by: {:?}", vs));
            }
        };

        let true_throw_to: usize = match lines
            .next()
            .ok_or(String::from("if true missing"))?
            .split_whitespace()
            .collect::<Vec<_>>()[..]
        {
            ["If", "true:", "throw", "to", "monkey", v] => {
                v.parse().map_err(|_| format!("divisible by: {:?}", v))?
            }
            ref vs => {
                return Err(format!("if true: {:?}", vs));
            }
        };

        let false_throw_to: usize = match lines
            .next()
            .ok_or(String::from("if false missing"))?
            .split_whitespace()
            .collect::<Vec<_>>()[..]
        {
            ["If", "false:", "throw", "to", "monkey", v] => {
                v.parse().map_err(|_| format!("divisible by: {:?}", v))?
            }
            ref vs => {
                return Err(format!("if false: {:?}", vs));
            }
        };

        Ok(Monkey {
            id,
            starting_items: starting_items.clone(),
            operation,
            divisible_by_test,
            true_throw_to,
            false_throw_to,
        })
    }
}

fn do_round(zoo: &[Monkey], dynamics: &mut [MonkeyDynamics], relief_fn: &dyn Fn(u64) -> u64) {
    for i in 0..zoo.len() {
        do_monkey_turn(i, zoo, dynamics, relief_fn);
    }
}

fn do_monkey_turn(
    index: usize,
    zoo: &[Monkey],
    dynamics: &mut [MonkeyDynamics],
    relief_fn: &dyn Fn(u64) -> u64,
) {
    let monkey: &Monkey = &zoo[index];

    // Take the monkey's items off their hands: we'll be distributing
    // to other monkeys.
    let items = std::mem::take(&mut dynamics[index].items);

    dynamics[index].count_inspected += items.len();
    for item in items {
        // Inspects an item, changing worry.
        let item = apply_operation(item, &monkey.operation);

        // Compute relief.
        let item = relief_fn(item);

        // Compute target to throw
        let target: usize = if item % monkey.divisible_by_test == 0 {
            monkey.true_throw_to
        } else {
            monkey.false_throw_to
        };

        // In-place throw to the next monkey.
        dynamics[target].items.push(item);
    }
}

fn apply_operation(item: u64, operation: &Operation) -> u64 {
    match operation {
        Operation::Add(operand) => item + apply_operand(item, operand),
        Operation::Multiply(operand) => item * apply_operand(item, operand),
    }
}

fn apply_operand(old: u64, operand: &Operand) -> u64 {
    match operand {
        Operand::Const(v) => *v,
        Operand::Old => old,
    }
}

fn parse_zoo(s: &str) -> Result<Vec<Monkey>, String> {
    s.split("\n\n").map(Monkey::from_str).collect()
}

fn part_1(input: &str) -> Result<(), Box<dyn Error>> {
    let zoo = parse_zoo(input)?;
    let mut dynamics: Vec<MonkeyDynamics> = zoo.iter().map(Monkey::get_dynamics).collect();
    for _ in 0..20 {
        do_round(&zoo, &mut dynamics, &|x| x / 3);
    }

    let mut inspections: Vec<usize> = dynamics
        .into_iter()
        .map(|monkey| monkey.count_inspected)
        .collect();
    inspections.sort();

    let monkey_business = inspections[inspections.len() - 2] * inspections[inspections.len() - 1];
    println!("part 1: {} (should be 316888)", monkey_business);

    Ok(())
}

fn part_2(input: &str) -> Result<(), Box<dyn Error>> {
    let zoo = parse_zoo(input)?;
    let mut dynamics: Vec<MonkeyDynamics> = zoo.iter().map(Monkey::get_dynamics).collect();

    // Keep the numbers down by doing modulo the LCM of all divisibles.
    // https://jactl.io/blog/2023/04/17/advent-of-code-2022-day11.html
    let common_multiple: u64 = zoo
        .iter()
        .map(|monkey| monkey.divisible_by_test)
        .fold(1, least_common_multiple);

    for _ in 0..10000 {
        do_round(&zoo, &mut dynamics, &|x| x % common_multiple);
    }

    let mut inspections: Vec<usize> = dynamics
        .into_iter()
        .map(|monkey| monkey.count_inspected)
        .collect();
    inspections.sort();

    let monkey_business = inspections[inspections.len() - 2] * inspections[inspections.len() - 1];
    println!("part 2: {} (should be 35270398814)", monkey_business);

    Ok(())
}

fn greatest_common_divisor(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn least_common_multiple(a: u64, b: u64) -> u64 {
    a * b / greatest_common_divisor(a, b)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("adventofcode.com_2022_day_11_input.txt")?;
    part_1(&input)?;
    println!();
    part_2(&input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() -> Result<(), String> {
        let input = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
"
        .trim_start();
        let monkey: Monkey = input.parse()?;

        assert_eq!(monkey.id, 0);
        assert_eq!(monkey.starting_items, vec![79, 98]);
        assert_eq!(monkey.operation, Operation::Multiply(Operand::Const(19)));
        assert_eq!(monkey.divisible_by_test, 23);
        assert_eq!(monkey.true_throw_to, 2);
        assert_eq!(monkey.false_throw_to, 3);

        Ok(())
    }

    const EXAMPLE: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_parse_zoo() -> Result<(), Box<dyn Error>> {
        let zoo = parse_zoo(EXAMPLE)?;
        assert_eq!(zoo.len(), 4);

        assert_eq!(
            zoo.into_iter()
                .map(|m| m.starting_items)
                .collect::<Vec<Vec<_>>>(),
            vec![
                vec![79, 98],
                vec![54, 65, 75, 74],
                vec![79, 60, 97],
                vec![74],
            ]
        );
        Ok(())
    }

    #[test]
    fn test_do_round() -> Result<(), Box<dyn Error>> {
        let zoo = parse_zoo(EXAMPLE)?;
        let mut dynamics: Vec<MonkeyDynamics> = zoo.iter().map(Monkey::get_dynamics).collect();
        do_round(&zoo, &mut dynamics, &|x| x / 3);

        assert_eq!(
            dynamics
                .into_iter()
                .map(|m| m.items)
                .collect::<Vec<Vec<_>>>(),
            vec![
                vec![20, 23, 27, 26],
                vec![2080, 25, 167, 207, 401, 1046],
                vec![],
                vec![],
            ]
        );

        Ok(())
    }
}
