use crate::common::{get_file_contents, Answer};
use crate::Args;
use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    goto: String,
    condition: Option<(char, char, isize)>,
}

type Rating = HashMap<char, isize>;
type Workflows = HashMap<String, Vec<Rule>>;

pub fn run(_args: &Args) -> Answer {
    let file = if _args.test {
        get_file_contents("day19-test")
    } else {
        get_file_contents("day19")
    };

    let (workflow_lines, rating_lines) = file.split_once("\n\n").unwrap();

    let workflows = workflow_lines
        .lines()
        .map(|line| {
            let (name, ruleset) = line.split_once('{').unwrap();

            let rules = ruleset
                .trim_end_matches('}')
                .split(',')
                .map(|r| {
                    if let Some((condition, goto)) = r.split_once(':') {
                        let (lhs_operator, value) = condition.split_at(2);

                        Rule {
                            goto: goto.to_owned(),
                            condition: Some((
                                lhs_operator.chars().nth(0).unwrap(),
                                lhs_operator.chars().nth(1).unwrap(),
                                value.parse::<isize>().unwrap(),
                            )),
                        }
                    } else {
                        Rule {
                            goto: r.to_owned(),
                            condition: None,
                        }
                    }
                })
                .collect::<Vec<Rule>>();

            (name.to_string(), rules)
        })
        .collect::<Workflows>();

    let ratings = rating_lines
        .lines()
        .map(|line| {
            line.trim_matches(['{', '}'].as_slice())
                .split(',')
                .map(|m| {
                    let (category, value) = m.split_once('=').unwrap();

                    (
                        category.chars().next().unwrap(),
                        value.parse::<isize>().unwrap(),
                    )
                })
                .collect::<Rating>()
        })
        .collect::<Vec<_>>();

    let part_1_score = ratings
        .into_iter()
        .filter(|rating| is_accepted(&rating, &workflows))
        .map(|rating| rating.values().sum::<isize>())
        .sum::<isize>();

    let combinations = number_combinations(&workflows);

    (part_1_score.to_string(), combinations.to_string())
}

fn number_combinations(workflows: &Workflows) -> isize {
    let range: HashMap<char, (isize, isize)> = HashMap::from([
        ('x', (1, 4000)),
        ('m', (1, 4000)),
        ('a', (1, 4000)),
        ('s', (1, 4000)),
    ]);

    branch_combinations(&range, "in".to_string(), &workflows)
}

fn branch_combinations(
    ranges: &HashMap<char, (isize, isize)>,
    workflow_address: String,
    workflows: &Workflows,
) -> isize {
    if &workflow_address == "R" {
        return 0;
    }

    if &workflow_address == "A" {
        return ranges.values().map(|(a, b)| (b - a) + 1).product::<isize>();
    }

    let workflow = workflows.get(&workflow_address).unwrap();
    let mut combinations = 0;
    let mut ranges = ranges.clone();

    for rule in workflow {
        let goto = rule.goto.as_str();

        ranges = if let Some((category, operator, value)) = rule.condition.to_owned() {
            // there's a condition, how much of that range is passed through vs not?

            let category_range = *ranges.get(&category).unwrap();

            if value < category_range.0 || value > category_range.1 {
                // the condition isn't accepted at all, return the full range to continue processing
                ranges
            } else {
                // the condition is accepted, parse the possibilities
                let mut low_range = ranges.clone();
                let mut high_range = ranges.clone();

                let (satisfied, unsatisfied) = if operator == '<' {
                    low_range.insert(category, (category_range.0, value - 1));
                    high_range.insert(category, (value, category_range.1));

                    (low_range, high_range)
                } else if operator == '>' {
                    low_range.insert(category, (category_range.0, value));
                    high_range.insert(category, (value + 1, category_range.1));

                    (high_range, low_range)
                } else {
                    panic!("Unexpected operator");
                };

                combinations += branch_combinations(&satisfied, goto.to_string(), &workflows);

                unsatisfied
            }
        } else {
            // no condition, it naively goes to the next node
            combinations += branch_combinations(&ranges, goto.to_string(), &workflows);

            break;
        };
    }

    combinations
}

fn is_accepted(rating: &Rating, workflows: &Workflows) -> bool {
    let mut workflow = workflows.get("in").unwrap();

    loop {
        let workflow_address = next_workflow(&rating, &workflow);

        if workflow_address == "A" {
            return true;
        } else if workflow_address == "R" {
            return false;
        }

        workflow = workflows.get(&workflow_address).unwrap();
    }
}

fn next_workflow(rating: &Rating, workflow: &Vec<Rule>) -> String {
    for rule in workflow {
        if let Some(condition) = rule.condition.to_owned() {
            if evaluation_condition(&rating, &condition) {
                return rule.goto.to_owned();
            }
        } else {
            return rule.goto.to_owned();
        }
    }

    panic!("Ran out of rules!")
}

fn evaluation_condition(rating: &Rating, condition: &(char, char, isize)) -> bool {
    let (category, operator, value) = condition;

    let multiplier = match operator {
        '>' => -1,
        '<' => 1,
        _ => panic!("Unexpected comparison"),
    };

    let rating_value = *rating.get(&category).unwrap() * multiplier;
    let comparison_value = value * multiplier;

    rating_value < comparison_value
}
