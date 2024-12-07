pub struct Equation {
    pub res: i64,
    rhs: Vec<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Mul,
    Cat,
}

impl Equation {
    pub fn has_solution(&self, ops: &[Op]) -> bool {
        self.solve(ops).is_some()
    }

    fn solve(&self, operators: &[Op]) -> Option<Vec<Op>> {
        let solutions: Vec<Vec<Op>> = perms(operators, self.rhs.len() - 1);
        for solution in solutions {
            if self.apply(&solution) == self.res {
                return Some(solution);
            }
        }
        None
    }

    fn apply(&self, solution: &Vec<Op>) -> i64 {
        assert!(solution.len() == self.rhs.len() - 1);
        let mut res = self.rhs[0];
        for (i, op) in solution.iter().enumerate() {
            res = match op {
                Op::Add => res + self.rhs[i + 1],
                Op::Mul => res * self.rhs[i + 1],
                Op::Cat => concat(res, self.rhs[i + 1]),
            }
        }
        res
    }
}

fn perms(ops: &[Op], n: usize) -> Vec<Vec<Op>> {
    if n == 0 {
        return vec![Vec::new()];
    }
    let mut res = Vec::new();
    for op in ops.iter() {
        for mut sol in perms(ops, n - 1) {
            sol.insert(0, op.clone());
            res.push(sol);
        }
    }
    res
}

fn concat(a: i64, b: i64) -> i64 {
    let n_digits = b.checked_ilog10().unwrap_or(0) + 1;
    a * 10_i64.pow(n_digits) + b
}

impl From<&str> for Equation {
    fn from(s: &str) -> Self {
        let (lhs, rhs) = s.split_once(": ").unwrap();
        let res = lhs.parse().unwrap();
        let rhs = rhs.split(" ").map(|s| s.parse().unwrap()).collect();
        Equation { res, rhs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let s = "123: 1 2 3";
        let eq = Equation::from(s);
        assert_eq!(eq.res, 123);
        assert_eq!(eq.rhs, vec![1, 2, 3]);
    }

    #[test]
    fn test_perms() {
        let ops = &[Op::Add, Op::Mul];
        let res = perms(ops, 2);
        assert_eq!(
            res,
            vec![
                vec![Op::Add, Op::Add],
                vec![Op::Add, Op::Mul],
                vec![Op::Mul, Op::Add],
                vec![Op::Mul, Op::Mul]
            ]
        );
        let res = perms(ops, 3);
        assert_eq!(
            res,
            vec![
                vec![Op::Add, Op::Add, Op::Add],
                vec![Op::Add, Op::Add, Op::Mul],
                vec![Op::Add, Op::Mul, Op::Add],
                vec![Op::Add, Op::Mul, Op::Mul],
                vec![Op::Mul, Op::Add, Op::Add],
                vec![Op::Mul, Op::Add, Op::Mul],
                vec![Op::Mul, Op::Mul, Op::Add],
                vec![Op::Mul, Op::Mul, Op::Mul]
            ]
        );
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(123, 456), 123456);
        assert_eq!(concat(456, 123), 456123);
    }

    #[test]
    fn test_apply() {
        let eq = Equation {
            res: 123,
            rhs: vec![1, 2, 3],
        };
        assert_eq!(eq.apply(&vec![Op::Add, Op::Add]), 6);
        assert_eq!(eq.apply(&vec![Op::Add, Op::Mul]), 9);
        assert_eq!(eq.apply(&vec![Op::Mul, Op::Add]), 5);
    }
}
