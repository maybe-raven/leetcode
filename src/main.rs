use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    ops::DerefMut,
};

use evaluate_divisions::Solution;

mod evaluate_divisions;
mod graph;

trait RefIntegrityStud<'a>
where
    Self: 'a,
    &'a Self: 'a,
{
}

trait RefIntegrity
where
    for<'a> Self: RefIntegrityStud<'a>,
{
}

impl<'a, T> RefIntegrityStud<'a> for T
where
    T: 'a,
    &'a T: 'a,
{
}
impl<T> RefIntegrity for T where for<'a> Self: RefIntegrityStud<'a> {}

trait IntoIntoIter<T: RefIntegrity, V>: RefIntegrity
where
    for<'a> &'a Self: Into<&'a T>,
    for<'a> &'a T: IntoIterator,
    for<'a> <&'a T as IntoIterator>::Item: Into<&'a V>,
{
    fn first(&self) -> Option<&V> {
        let t: &T = <&Self as Into<&T>>::into(self);
        let mut iter: <&T as IntoIterator>::IntoIter = <&T as IntoIterator>::into_iter(t);
        let next: Option<<&T as IntoIterator>::Item> =
            <&T as IntoIterator>::IntoIter::next(&mut iter);
        if let Some(result) = next {
            Some(<<&T as IntoIterator>::Item as Into<&V>>::into(result))
        } else {
            None
        }
    }
}

struct MyStruct<V> {
    name: String,
    count: i32,
    list: Vec<V>,
}

impl<'a, V> From<&'a MyStruct<V>> for &'a Vec<V> {
    fn from(value: &'a MyStruct<V>) -> Self {
        &value.list
    }
}

impl<V: RefIntegrity> IntoIntoIter<Vec<V>, V> for MyStruct<V> {}

fn f(input: Option<i32>) -> Option<i32> {
    Some(input? * 2)
}

fn main() {
    let s = MyStruct {
        name: "Yo Mama".to_string(),
        count: 69,
        list: vec![1.1, 2.2, 3.3, 4.4],
    };
    println!("{}", s.first().unwrap());
    println!("{:?}", f(Some(42)));
    println!("{:?}", f(None));

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
}
