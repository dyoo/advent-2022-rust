mod parser;

use nom::IResult;
use std::collections::HashMap;

use parser::{Expr, Job, Op};

#[derive(Debug, PartialEq)]
struct JobList<'a> {
    jobs: HashMap<&'a str, Job<'a>>,
}

fn parse_all_jobs(s: &str) -> JobList {
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
