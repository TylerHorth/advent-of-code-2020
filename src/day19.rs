use aoc_runner_derive::aoc;
use pest_derive::Parser;
use pest::{Parser, iterators::Pair};
use itertools::Itertools;
use regex::Regex;

enum MessageRule {
    Nul,
    Char(char),
    Alt(Vec<Vec<usize>>)
}

#[derive(Parser)]
#[grammar = "grammars/day19.pest"]
struct RuleParser {
    regex: Regex,
}

impl RuleParser {
    fn parse_int(pair: Pair<Rule>) -> usize {
        pair.as_str().parse().unwrap()
    }

    fn parse_seq(pair: Pair<Rule>) -> Vec<usize> {
        pair
            .into_inner()
            .map(Self::parse_int)
            .collect()
    }

    fn parse_alt(pair: Pair<Rule>) -> Vec<Vec<usize>> {
        pair
            .into_inner()
            .map(Self::parse_seq)
            .collect()
    }

    fn parse_char(pair: Pair<Rule>) -> char {
        pair.as_str().chars().exactly_one().unwrap()
    }

    fn format(rules: &[MessageRule], rule: &MessageRule, regex: &mut String, part2: bool) {
        match rule {
            MessageRule::Char(c) => regex.push(*c),
            MessageRule::Alt(alternatives) => {
                regex.push('(');
                for (i, seq) in alternatives.iter().enumerate() {
                    if i > 0 {
                        regex.push('|')
                    }

                    for &index in seq {
                        if part2 && index == 8 {
                            Self::format(rules, &rules[42], regex, part2);
                            regex.push('+');
                        } else if part2 && index == 11 {
                            regex.push('(');
                            for i in 1..10 {
                                if i > 1 {
                                    regex.push('|');
                                }
                                Self::format(rules, &rules[42], regex, part2);
                                regex.push('{');
                                regex.push(('0' as u8 + i) as char);
                                regex.push('}');
                                Self::format(rules, &rules[31], regex, part2);
                                regex.push('{');
                                regex.push(('0' as u8 + i) as char);
                                regex.push('}');
                            }
                            regex.push(')');
                        } else {
                            Self::format(rules, &rules[index], regex, part2);
                        }
                    }
                }
                regex.push(')');
            },
            _ => unreachable!()
        }
    }

    fn new(grammar: &str, part2: bool) -> Self {
        let mut rules = Vec::new();

        for rule in Self::parse(Rule::all, grammar).unwrap() {
            if matches!(rule.as_rule(), Rule::EOI) {
                continue
            }

            let (index, value) = rule.into_inner().next_tuple().unwrap();

            let index = Self::parse_int(index);

            let value = match value.as_rule() {
                Rule::char => MessageRule::Char(Self::parse_char(value)),
                Rule::alt => MessageRule::Alt(Self::parse_alt(value)),
                _ => unreachable!()
            };

            if index >= rules.len() {
                rules.extend((rules.len()..=index).map(|_| MessageRule::Nul))
            }

            rules[index] = value;
        }

        let mut regex = String::new();

        regex.push('^');

        Self::format(&rules, &rules[0], &mut regex, part2);

        regex.push('$');

        Self {
            regex: Regex::new(&regex).unwrap()
        }
    }

    fn matches(&self, value: &str) -> bool {
        self.regex.is_match(value)
    }
}


#[aoc(day19, part1)]
fn part1(input: &str) ->  usize {
    let (grammar, values) = input.split("\n\n").next_tuple().unwrap();

    let parser = RuleParser::new(grammar, false);

    values
        .lines()
        .filter(|l| parser.matches(l))
        .count()
}

#[aoc(day19, part2)]
fn part2(input: &str) ->  usize {
    let (grammar, values) = input.split("\n\n").next_tuple().unwrap();

    let parser = RuleParser::new(grammar, true);

    values
        .lines()
        .filter(|l| parser.matches(l))
        .count()
}