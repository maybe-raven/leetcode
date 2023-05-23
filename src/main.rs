use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
};

use evaluate_divisions::Solution;

mod evaluate_divisions;
mod graph;

fn add<'a: 'b, 'b>(set: &mut HashSet<&'b str>, value: &'a str) {
    set.insert(value);
}

fn inc<T: DerefMut<Target = i32>>(mut input: T) {
    *input += 1;
}

fn main() {
    let mut i = 10;
    inc(&mut i);
    println!("{}", i);

    // let equations = vec![
    //     vec!["x1".to_string(), "x2".to_string()],
    //     vec!["x2".to_string(), "x3".to_string()],
    //     vec!["x3".to_string(), "x4".to_string()],
    //     vec!["x4".to_string(), "x5".to_string()],
    // ];
    // let values = vec![3.0, 4.0, 5.0, 6.0];
    // let queries = vec![
    //     vec!["x1".to_string(), "x5".to_string()],
    //     vec!["x5".to_string(), "x2".to_string()],
    //     vec!["x2".to_string(), "x4".to_string()],
    //     vec!["x2".to_string(), "x2".to_string()],
    //     vec!["x2".to_string(), "x9".to_string()],
    //     vec!["x9".to_string(), "x9".to_string()],
    // ];
    // let results = Solution::calc_equation(equations, values, queries);
    // println!("{:?}", results);

    // let mut map = HashMap::new();
    // let key = "Shit";
    // map.insert("Shit", 4);
    // let entry = map.entry(key);
    // println!("{}", key);
    // println!("{:?}", entry);
    // map.insert("Fuck", 4);
}
