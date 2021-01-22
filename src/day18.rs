use std::io::BufRead;

const INPUT_PATH: &str = "input/day18";

fn eval_no_parentheses(expr: &str) -> i64 {
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

fn eval(expr: &str) -> i64 {
    fn helper(expr: &str) -> i64 {
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
            let res = helper(&extracted);

            expr = format!(
                "{}{}{}",
                &expr[..par.first().unwrap().0 - 1],
                res,
                &expr[par.last().unwrap().0 + 2..]
            );
        }

        eval_no_parentheses(&expr)
    }

    helper(expr)
}

pub fn part1() -> i64 {
    crate::get_buffered_input(INPUT_PATH)
        .lines()
        .fold(0, |acc, expr| acc + eval(&expr.unwrap()))
}

pub fn part2() -> i32 {
    0
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
        println!("{}", part2());
        assert_eq!(part2(), 0);
    }

    #[test]
    fn part1_example() {
        let expr1 = "1 + 2 * 3 + 4 * 5 + 6";
        let expr2 = "1 + (2 * 3) + (4 * (5 + 6))";
        let expr3 = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

        assert_eq!(eval(expr1), 71);
        assert_eq!(eval(expr2), 51);
        assert_eq!(eval(expr3), 13632);
    }

    #[test]
    fn part2_example() {
        assert_eq!(true, true);
    }
}
