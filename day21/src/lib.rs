mod parser;

use std::collections::HashMap;

use parser::{Expr, Job, Op};

#[derive(Debug, PartialEq)]
pub struct JobList<'a> {
    jobs: HashMap<&'a str, Job<'a>>,
}

impl<'a> JobList<'a> {
    pub fn get_money(&self, name: &str) -> i64 {
        match &self.jobs.get(name).expect(name).1 {
            Expr::Num(n) => *n,
            Expr::BinOp { op, lhs, rhs } => {
                let lhs_money = self.get_money(lhs);
                let rhs_money = self.get_money(rhs);
                match op {
                    Op::Add => lhs_money + rhs_money,
                    Op::Sub => lhs_money - rhs_money,
                    Op::Mul => lhs_money * rhs_money,
                    Op::Div => lhs_money / rhs_money,
                }
            }
        }
    }

    // For part 2 of the problem, we'll take a numeric calculus approach
    // and treat this as a minimization problem.  The answer we're looking for
    // should have a loss of 0.
    fn loss(&self, guess: f64) -> f64 {
        match &self.jobs.get("root").unwrap().1 {
            Expr::BinOp { op, lhs, rhs } => f64::powf(
                self.get_money_part_2(lhs, guess) - self.get_money_part_2(rhs, guess),
                2.0,
            ),
            _ => panic!("root should be a binop"),
        }
    }

    fn get_money_part_2(&self, name: &str, humn_val: f64) -> f64 {
        if name == "humn" {
            return humn_val;
        }
        match &self.jobs.get(name).expect(name).1 {
            Expr::Num(n) => *n as f64,
            Expr::BinOp { op, lhs, rhs } => {
                let lhs_money = self.get_money_part_2(lhs, humn_val);
                let rhs_money = self.get_money_part_2(rhs, humn_val);
                match op {
                    Op::Add => lhs_money + rhs_money,
                    Op::Sub => lhs_money - rhs_money,
                    Op::Mul => lhs_money * rhs_money,
                    Op::Div => lhs_money / rhs_money,
                }
            }
        }
    }
}

pub fn parse_all_jobs(s: &str) -> JobList {
    let jobs: Vec<Job> = s
        .lines()
        .map(parser::parse_job)
        .filter_map(|r| r.ok())
        .map(|r| r.1)
        .collect();

    JobList {
        jobs: jobs.into_iter().map(|j| (j.0, j)).collect(),
    }
}

const DELTA: f64 = 0.0001;
const LEARNING: f64 = 0.1;

fn find_minimum(f: impl Fn(f64) -> f64) -> f64 {
    // Compute derivative (f(x+delta) -f(x)) / delta
    let mut x: f64 = 1.0;
    for i in 0..1000 {
        let fx = f(x);
        let fdelta = f(x + DELTA);
        let neg_deriv = -(fdelta - fx) / DELTA;

        if i % 10 == 0 {
            println!("{}: guess={}, fx={}, neg_deriv={}", i, x, fx, neg_deriv);
        }

        x += neg_deriv * LEARNING;
    }
    x
}

#[test]
fn test_parse_all_jobs() {
    let parsed = parse_all_jobs(
        "
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
",
    );

    let expected_jobs = vec![
        Job(
            "root",
            Expr::BinOp {
                op: Op::Add,
                lhs: "pppw",
                rhs: "sjmn",
            },
        ),
        Job("dbpl", Expr::Num(5)),
        Job(
            "cczh",
            Expr::BinOp {
                op: Op::Add,
                lhs: "sllz",
                rhs: "lgvd",
            },
        ),
        Job("zczc", Expr::Num(2)),
    ];

    assert_eq!(
        parsed,
        JobList {
            jobs: expected_jobs.into_iter().map(|j| (j.0, j)).collect()
        }
    );
}

#[test]
fn test_get_money() {
    let s = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";
    let joblist = parse_all_jobs(s);
    assert_eq!(joblist.get_money("root"), 152);
}

#[test]
fn test_get_loss() {
    let s = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";
    let joblist = parse_all_jobs(s);
    assert_eq!(joblist.loss(301.0), 0.0);
}

#[test]
fn test_find_minimum() {
    let s = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";
    let joblist = parse_all_jobs(s);

    let min = find_minimum(|x| joblist.loss(x));

    assert_eq!(min, 301.0);
}
