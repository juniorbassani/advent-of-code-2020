use std::io::BufRead;

const INPUT_PATH: &str = "input/day18";

fn left_to_right(expr: &str) -> i64 {
    let mut lhs = None;
    let mut res = 0;
    let mut op = "";

    for elem in expr.split_whitespace() {
        match elem {
            "+" => {
                op = "+";
            }
            "*" => {
                op = "*";
            }
            _ => {
                let e = elem.parse::<i64>().unwrap();

                if let None = lhs {
                    lhs = Some(e);
                } else {
                    if op == "+" {
                        res = lhs.take().unwrap() + e;
                    } else {
                        res = lhs.take().unwrap() * e;
                    }

                    lhs = Some(res);
                }
            }
        }
    }

    res
}

fn advanced_math(expr: &str) -> i64 {
    let mut expr = expr.to_owned();

    while let Some(pos) = expr.find('+') {
        let lhs_str = expr[..pos - 1]
            .chars()
            .rev()
            .take_while(|&c| c.is_digit(10))
            .collect::<String>();
        let rhs_str = expr[pos + 2..]
            .chars()
            .take_while(|&c| c.is_digit(10))
            .collect::<String>();

        let lhs = lhs_str
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        let rhs = rhs_str.parse::<i64>().unwrap();

        let partial_res = lhs + rhs;
        let substitution = format!("{} + {}", lhs.to_string(), rhs_str);

        expr = expr.replacen(&substitution, &partial_res.to_string(), 1);
    }

    if let Ok(res) = expr.parse::<i64>() {
        res
    } else {
        left_to_right(&expr)
    }
}

fn eval<F>(expr: &str, math_rules: &F) -> i64
where
    F: Fn(&str) -> i64,
{
    let mut expr = expr.to_owned();

    while let Some(op_par) = expr.find('(') {
        let mut counter = 1;

        let par: Vec<_> = expr
            .char_indices()
            .skip(op_par + 1)
            .take_while(|&(_, c)| {
                if c == '(' {
                    counter += 1;
                } else if c == ')' {
                    counter -= 1;
                }

                counter > 0
            })
            .collect();

        let extracted = par.iter().map(|&(_, c)| c).collect::<String>();
        let res = eval(&extracted, math_rules);

        expr = format!(
            "{}{}{}",
            &expr[..par.first().unwrap().0 - 1],
            res,
            &expr[par.last().unwrap().0 + 2..]
        );
    }

    math_rules(&expr)
}

pub fn part1() -> i64 {
    crate::get_buffered_input(INPUT_PATH)
        .lines()
        .fold(0, |acc, expr| acc + eval(&expr.unwrap(), &left_to_right))
}

pub fn part2() -> i64 {
    crate::get_buffered_input(INPUT_PATH)
        .lines()
        .fold(0, |acc, expr| acc + eval(&expr.unwrap(), &advanced_math))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_part1() {
        assert_eq!(part1(), 36382392389406);
    }

    #[test]
    fn compute_part2() {
        assert_eq!(part2(), 381107029777968);
    }

    #[test]
    fn part1_example() {
        let expr1 = "1 + 2 * 3 + 4 * 5 + 6";
        let expr2 = "1 + (2 * 3) + (4 * (5 + 6))";
        let expr3 = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

        assert_eq!(eval(expr1, &left_to_right), 71);
        assert_eq!(eval(expr2, &left_to_right), 51);
        assert_eq!(eval(expr3, &left_to_right), 13632);
    }

    #[test]
    fn part2_example() {
        let expr1 = "1 + 2 * 3 + 4 * 5 + 6";
        let expr2 = "1 + (2 * 3) + (4 * (5 + 6))";
        let expr3 = "2 * 3 + (4 * 5)";
        let expr4 = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let expr5 = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let expr6 = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

        assert_eq!(eval(expr1, &advanced_math), 231);
        assert_eq!(eval(expr2, &advanced_math), 51);
        assert_eq!(eval(expr3, &advanced_math), 46);
        assert_eq!(eval(expr4, &advanced_math), 1445);
        assert_eq!(eval(expr5, &advanced_math), 669060);
        assert_eq!(eval(expr6, &advanced_math), 23340);
    }
}
