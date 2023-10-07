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
    Const(i32),
    Old,
}

#[derive(Debug, PartialEq)]
struct Monkey {
    id: usize,
    starting_items: Vec<i32>,
    operation: Operation,
    divisible_by_test: i32,
    true_throw_to: usize,
    false_throw_to: usize,

    items: Vec<i32>,
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

        let starting_items: Vec<i32> = match lines
            .next()
            .ok_or(String::from("Missing starting item line"))?
            .trim_start()
            .split(&[' ', ','])
            .collect::<Vec<_>>()[..]
        {
            ["Starting", "items:", ref items @ ..] => items
                .into_iter()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<_>>(),
            ref vs => {
                return Err(format!("starting item: {:?}", vs));
            }
        };

        let operation: Operation = match lines
            .next()
            .ok_or(String::from("operation missing line"))?
            .trim_start()
            .split_whitespace()
            .collect::<Vec<_>>()[..]
        {
            ["Operation:", "new", "=", "old", op, v] => {
                let v = if v == "old" {
                    Operand::Old
                } else {
                    v.parse::<i32>()
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
            _ => {
                return Err(format!("operation"));
            }
        };

        let divisible_by_test: i32 = match lines
            .next()
            .ok_or(String::from("divisible by missing"))?
            .trim_start()
            .split_whitespace()
            .collect::<Vec<_>>()[..]
        {
            ["Test:", "divisible", "by", v] => v
                .parse::<i32>()
                .map_err(|_| format!("divisible by: {:?}", v))?,
            ref vs => {
                return Err(format!("divisible by: {:?}", vs));
            }
        };

        let true_throw_to: usize = match lines
            .next()
            .ok_or(String::from("if true missing"))?
            .trim_start()
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
            .trim_start()
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

            items: starting_items,
        })
    }
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
            zoo
                .into_iter()
                .map(|m| m.items)
                .collect::<Vec<Vec<i32>>>(),
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
        let mut zoo = parse_zoo(EXAMPLE)?;
	do_round(&mut zoo);

        assert_eq!(
            zoo
                .into_iter()
                .map(|m| m.items)
                .collect::<Vec<Vec<i32>>>(),
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

fn do_round(zoo: &mut Vec<Monkey>) {
    for i in 0..zoo.len() {
        do_monkey_turn(i, zoo);
    }
}

fn do_monkey_turn(index: usize, zoo: &mut Vec<Monkey>) {
    let monkey: &mut Monkey = &mut zoo[index];
    // Take the monkey's items: we'll be distributing.
    let items = std::mem::replace(&mut monkey.items, vec![]);

    // Make the borrow checker happy: copy over exactly the rest of
    // the fields we're reading from the monkey, so that we can
    // in-place modify the monkeys in the zoo.
    let operation = monkey.operation.clone();
    let divisible_by_test = monkey.divisible_by_test;
    let true_throw_to = monkey.true_throw_to;
    let false_throw_to = monkey.false_throw_to;

    for mut item in items {
        // Inspects an item, changing worry.
        item = apply_operation(item, &operation);

        // Compute relief.
        item /= 3;

        // Compute target to throw
        let target: usize = if item % divisible_by_test == 0 {
            true_throw_to
        } else {
            false_throw_to
        };

        // In-place throw to the next monkey.  This is what
        // invalidates any earlier borrows of zoo.
        zoo[target].items.push(item);
    }
}

fn apply_operation(item: i32, operation: &Operation) -> i32 {
    match operation {
        Operation::Add(operand) => item + apply_operand(item, operand),
        Operation::Multiply(operand) => item * apply_operand(item, operand),
    }
}

fn apply_operand(old: i32, operand: &Operand) -> i32 {
    match operand {
        Operand::Const(v) => *v,
        Operand::Old => old,
    }
}

fn parse_zoo(s: &str) -> Result<Vec<Monkey>, String> {
    s.split("\n\n").map(|s| Monkey::from_str(s)).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("adventofcode.com_2022_day_11_input.txt")?;
    let zoo = parse_zoo(&input)?;
    for monkey in zoo.iter() {
        println!("{:?}", monkey);
        println!();
    }

    Ok(())
}
