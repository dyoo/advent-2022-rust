use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Operation {
    Multiply(Operand),
    Add(Operand),
}

#[derive(Debug, PartialEq)]
enum Operand {
    Const(i32),
    Old,
}

#[derive(Debug, PartialEq)]
struct Monkey {
    id: i32,
    starting_items: Vec<i32>,
    operation: Operation,
    divisible_by_test: i32,
    true_throw_to: i32,
    false_throw_to: i32,

    items: Vec<i32>,
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let id: i32 = match lines
            .next()
            .ok_or(String::from("missing id"))?
            .split(&[' ', ':'])
            .collect::<Vec<_>>()[..]
        {
            ["Monkey", n, ""] => n.parse::<i32>().map_err(|_| String::from("missing id"))?,
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

        let true_throw_to: i32 = match lines
            .next()
            .ok_or(String::from("if true missing"))?
            .trim_start()
            .split_whitespace()
            .collect::<Vec<_>>()[..]
        {
            ["If", "true:", "throw", "to", "monkey", v] => v
                .parse::<i32>()
                .map_err(|_| format!("divisible by: {:?}", v))?,
            ref vs => {
                return Err(format!("if true: {:?}", vs));
            }
        };

        let false_throw_to: i32 = match lines
            .next()
            .ok_or(String::from("if false missing"))?
            .trim_start()
            .split_whitespace()
            .collect::<Vec<_>>()[..]
        {
            ["If", "false:", "throw", "to", "monkey", v] => v
                .parse::<i32>()
                .map_err(|_| format!("divisible by: {:?}", v))?,
            ref vs => {
                return Err(format!("if false: {:?}", vs));
            }
        };

        Ok(Monkey {
            id,
            starting_items,
            operation,
            divisible_by_test,
            true_throw_to,
            false_throw_to,

	    items: vec![],
        })
    }
}

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for chunk in read_to_string("adventofcode.com_2022_day_11_input.txt")?.split("\n\n") {
        println!("{:?}", chunk.parse::<Monkey>()?);
    }

    Ok(())
}
