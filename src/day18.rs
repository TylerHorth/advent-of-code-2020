use aoc_runner_derive::aoc;
use pest::iterators::Pair;
use pest::prec_climber::PrecClimber;
use pest::prec_climber::{Assoc, Operator};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/day18.pest"]
struct MathParser;

fn eval(expression: Pair<Rule>, precedence: &PrecClimber<Rule>) -> u64 {
    precedence.climb(
        expression.into_inner(),
        |pair| match pair.as_rule() {
            Rule::int => pair.as_str().parse().unwrap(),
            Rule::expression => eval(pair, precedence),
            _ => unreachable!(),
        },
        |lhs, op, rhs| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::multiply => lhs * rhs,
            _ => unreachable!(),
        },
    )
}

fn run_with_precedence(input: &str, precedence: PrecClimber<Rule>) -> u64 {
    MathParser::parse(Rule::homework, input)
        .unwrap()
        .map(|expr| eval(expr, &precedence))
        .sum()
}

#[aoc(day18, part1)]
fn part1(input: &str) -> u64 {
    let precedence = PrecClimber::new(vec![
        Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::multiply, Assoc::Left),
    ]);

    run_with_precedence(input, precedence)
}

#[aoc(day18, part2)]
fn part2(input: &str) -> u64 {
    let precedence = PrecClimber::new(vec![
        Operator::new(Rule::multiply, Assoc::Left),
        Operator::new(Rule::add, Assoc::Left),
    ]);

    run_with_precedence(input, precedence)
}
