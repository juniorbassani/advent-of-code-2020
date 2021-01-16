use std::collections::HashMap;
use std::ops::RangeInclusive;

const INPUT_PATH: &str = "input/day16";

fn ticket_error_rates(rules: &[RangeInclusive<i32>], tickets: &Vec<Vec<i32>>) -> i32 {
    tickets
        .iter()
        .flatten()
        .copied()
        .filter(|number| !rules.iter().any(|rule| rule.contains(number)))
        .sum()
}

fn discard_invalid_tickets(rules: &[&RangeInclusive<i32>], tickets: &mut Vec<Vec<i32>>) {
    tickets.retain(|ticket| {
        ticket
            .iter()
            .all(|num| rules.iter().any(|rule| rule.contains(num)))
    });
}

// tickets[0] will be mine
fn departure_fields(
    rules: &[(&str, &[RangeInclusive<i32>])],
    tickets: &mut Vec<Vec<i32>>,
) -> usize {
    let r: Vec<_> = rules.iter().map(|r| r.1).flatten().collect();

    discard_invalid_tickets(r.as_slice(), tickets);

    let mut order: HashMap<&str, Vec<usize>> = HashMap::with_capacity(rules.len());

    for j in 0..rules.len() {
        let fields: Vec<i32> = tickets.iter().map(|t| t[j]).collect();

        let res = rules.iter().filter(|&(_, r)| {
            fields
                .iter()
                .all(|field| r.iter().any(|r| r.contains(field)))
        });

        for r in res {
            order.entry(r.0).or_default().push(j);
        }
    }

    let mut final_res = HashMap::new();

    // Eliminate duplicates; retain only the column unique to the field
    for _ in 0..rules.len() {
        for (&name, rules) in &order {
            let res = rules.iter().find(|rule| {
                order
                    .iter()
                    .all(|(&n, v)| return if name == n { true } else { !v.contains(rule) })
            });

            if let Some(&res) = res {
                final_res.insert(name, res);
            }
        }

        for (&name, &rules) in &final_res {
            order.insert(name, vec![rules]);
        }
    }

    order
        .iter()
        .filter(|(&name, _)| name.contains("departure"))
        .fold(1, |acc, (_, column)| acc * tickets[0][column[0]] as usize)
}

pub fn part1() -> i32 {
    let input = crate::get_input_as_string(INPUT_PATH);
    let mut input = input.split("\n\n");
    let r = input.next().unwrap();
    let n = input.skip(1).next().unwrap();

    let mut rules = Vec::with_capacity(r.lines().count() * 2);
    let mut nearby_tickets: Vec<Vec<i32>> = Vec::with_capacity(n.lines().count() - 1);

    for rule in r.lines() {
        let numbers = rule.split(": ").skip(1).next().unwrap().split(" or ");

        for number in numbers {
            let mut number = number.split("-");
            let lhs: i32 = number.next().unwrap().parse().unwrap();
            let rhs: i32 = number.next().unwrap().parse().unwrap();

            rules.push(lhs..=rhs);
        }
    }

    for ticket in n.lines().skip(1) {
        nearby_tickets.push(ticket.split(',').map(|t| t.parse().unwrap()).collect());
    }

    ticket_error_rates(&rules, &nearby_tickets)
}

pub fn part2() -> usize {
    let input = crate::get_input_as_string(INPUT_PATH);
    let mut input = input.split("\n\n");
    let r = input.next().unwrap();
    let my_ticket = input.next().unwrap();
    let n = input.next().unwrap();

    let mut all_rules = HashMap::with_capacity(r.lines().count());
    let mut all_tickets: Vec<Vec<i32>> = Vec::with_capacity(n.lines().count());

    for rule in r.lines() {
        let mut rules = Vec::with_capacity(2);
        let mut rule = rule.split(": ");
        let field = rule.next().unwrap();
        let numbers = rule.next().unwrap().split(" or ");

        for number in numbers {
            let mut number = number.split("-");
            let lhs: i32 = number.next().unwrap().parse().unwrap();
            let rhs: i32 = number.next().unwrap().parse().unwrap();

            rules.push(lhs..=rhs);
        }

        all_rules.insert(field, &*rules.leak());
    }

    all_tickets.push(
        my_ticket
            .lines()
            .skip(1)
            .next()
            .unwrap()
            .split(',')
            .map(|t| t.parse().unwrap())
            .collect(),
    );
    for ticket in n.lines().skip(1) {
        all_tickets.push(ticket.split(',').map(|t| t.parse().unwrap()).collect());
    }

    let a: Vec<_> = all_rules.iter().map(|(&a, &b)| (a, b)).collect();
    departure_fields(&a, &mut all_tickets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 26026);
    }

    #[test]
    fn compute_part2() {
        assert_eq!(part2(), 1305243193339);
    }

    #[test]
    fn part1_example() {
        let rules = vec![1..=3, 5..=7, 6..=11, 33..=44, 13..=40, 45..=50];
        let _my_ticket = [7, 1, 14];
        let nearby_tickets = vec![
            vec![7, 3, 47],
            vec![40, 4, 50],
            vec![55, 2, 20],
            vec![38, 6, 12],
        ];

        assert_eq!(ticket_error_rates(&rules, &nearby_tickets), 71);
    }
}
