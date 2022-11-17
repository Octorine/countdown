use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

pub fn solve(numbers: &Vec<i32>, goal: i32) -> String {
    let mut m = HashMap::new();
    add_values(numbers, &mut m);
    match m.get(numbers).and_then(|m2| m2.get(&goal)) {
        Some(e) => format!("{} = {}", e, goal),
        None => "Not possible".to_string(),
    }
}
pub fn do_numbers_puzzle() {
    println!("Enter a collection of numbers with spaces in between.");
    let mut numbers = String::new();
    std::io::stdin().read_line(&mut numbers).unwrap();
    let numbers: Vec<i32> = numbers
        .split_ascii_whitespace()
        .filter_map(|n| n.trim().parse().ok())
        .collect();
    let mut goal_text = String::new();
    let mut goal: Option<i32> = None;
    while goal.is_none() {
        println!("Enter a goal number.");
        std::io::stdin().read_line(&mut goal_text).unwrap();
        goal = goal_text.trim().parse().ok();
    }
    let goal = goal.unwrap();
    println!("{}", solve(&numbers, goal));
}
#[derive(Clone, Debug)]
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}
#[allow(dead_code)]
impl Expr {
    fn eval(&self) -> i32 {
        use Expr::{Add, Div, Mul, Num, Sub};
        match &self {
            &Num(n) => *n,
            &Add(a, b) => a.eval() + b.eval(),
            &Sub(a, b) => a.eval() - b.eval(),
            &Mul(a, b) => a.eval() * b.eval(),
            &Div(a, b) => a.eval() / b.eval(),
        }
    }
    fn add(self, other: Expr) -> Expr {
        Expr::Add(Box::new(self), Box::new(other))
    }
    fn sub(self, other: Expr) -> Expr {
        Expr::Sub(Box::new(self), Box::new(other))
    }
    fn mul(self, other: Expr) -> Expr {
        Expr::Mul(Box::new(self), Box::new(other))
    }
    fn div(self, other: Expr) -> Expr {
        Expr::Div(Box::new(self), Box::new(other))
    }
    fn to_string_lvl(&self, level: usize) -> String {
        match &self {
            Expr::Num(n) => format!("{:?}", n),
            &Expr::Add(a, b) => {
                if level > 1 {
                    format!("({} + {})", a.to_string_lvl(2), b.to_string_lvl(1))
                } else {
                    format!("{} + {}", a.to_string_lvl(1), b.to_string_lvl(1))
                }
            }
            &Expr::Sub(a, b) => {
                if level > 1 {
                    format!("({} - {})", a.to_string_lvl(2), b.to_string_lvl(1))
                } else {
                    format!("{} - {}", a.to_string_lvl(1), b.to_string_lvl(1))
                }
            }
            &Expr::Mul(a, b) => format!("{} * {}", a.to_string_lvl(2), b.to_string_lvl(2)),
            &Expr::Div(a, b) => format!("{} / {}", a.to_string_lvl(2), b.to_string_lvl(2)),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.to_string_lvl(0))
    }
}

#[test]
fn test_eval() {
    assert_eq!((Expr::Num(1).add(Expr::Num(1))).mul(Expr::Num(3)).eval(), 6);
}
#[test]
fn test_display() {
    assert_eq!(format!("{}", Expr::Num(2).add(Expr::Num(2))), "2 + 2")
}

fn nth_partition(v: &Vec<i32>, n: usize) -> (Vec<i32>, Vec<i32>) {
    let mut l = Vec::new();
    let mut r = Vec::new();
    let mut n = n;
    for element in v.iter() {
        if n % 2 == 0 {
            l.push(*element);
        } else {
            r.push(*element);
        }
        n = n / 2;
    }
    (l, r)
}
fn splits(v: &Vec<i32>) -> Vec<(Vec<i32>, Vec<i32>)> {
    let mut result = Vec::new();
    let base: i32 = 2;
    for index in 1..(base.pow(v.len() as u32) - 1) {
        result.push(nth_partition(v, index as usize));
    }
    result
}

fn add_values(nums: &Vec<i32>, m: &mut HashMap<Vec<i32>, HashMap<i32, Expr>>) {
    if nums.len() == 1 {
        let mut vals = HashMap::new();
        vals.insert(nums[0], Expr::Num(nums[0]));
        m.insert(nums.clone(), vals);
    } else {
        let mut new_values: HashMap<i32, Expr> = HashMap::new();
        for (l, r) in splits(nums) {
            if !m.contains_key(&l) {
                add_values(&l, m);
            }
            if !m.contains_key(&r) {
                add_values(&r, m);
            }
            let left_values = m.get(&l).unwrap();
            let right_values = m.get(&r).unwrap();
            for (k, v) in left_values.iter().chain(right_values.iter()) {
                if !new_values.contains_key(k) {
                    new_values.insert(*k, v.clone());
                }
            }
            for (lk, lv) in left_values.iter() {
                for (rk, rv) in right_values.iter() {
                    if !new_values.contains_key(&(lk + rk)) {
                        new_values.insert(lk + rk, lv.clone().add(rv.clone()));
                    }
                    if !new_values.contains_key(&(lk * rk)) {
                        new_values.insert(lk * rk, lv.clone().mul(rv.clone()));
                    }
                    if lk >= rk {
                        if !new_values.contains_key(&(lk - rk)) {
                            new_values.insert(lk - rk, lv.clone().sub(rv.clone()));
                        }
                        if rk != &0 && lk % rk == 0 && !new_values.contains_key(&(lk / rk)) {
                            new_values.insert(lk / rk, lv.clone().div(rv.clone()));
                        }
                    } else {
                        if !new_values.contains_key(&(rk - lk)) {
                            new_values.insert(rk - lk, rv.clone().sub(lv.clone()));
                        }
                        if lk != &0 && rk % lk == 0 && !new_values.contains_key(&(rk / lk)) {
                            new_values.insert(rk / lk, rv.clone().div(lv.clone()));
                        }
                    }
                }
            }
        }
        m.insert(nums.clone(), new_values);
    }
}
