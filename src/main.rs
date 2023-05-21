use concrete::Solution;

pub mod concrete;

// use std::cmp::Ordering;
// use std::fmt::Debug;
// use std::marker::Copy;
// use std::ops::Mul;

// trait Dimension: Mul<Output = Self> + Copy + PartialEq + PartialOrd + Debug {}
// impl<T: Mul<Output = T> + Copy + PartialEq + PartialOrd + Debug> Dimension for T {}

// #[derive(Debug, Clone, Copy)]
// struct Rectangle<T: Dimension> {
//     width: T,
//     height: T,
// }

// impl<T: Dimension> Rectangle<T> {
//     fn square(dimension: T) -> Self {
//         Rectangle {
//             width: dimension,
//             height: dimension,
//         }
//     }

//     fn area(&self) -> T {
//         self.width * self.height
//     }

//     fn mix_up(rect1: &mut Self, rect2: &mut Self) {
//         let height = rect1.height;
//         rect1.height = rect2.height;
//         rect2.height = height;
//     }

//     fn bigger<'a>(rect1: &'a Self, rect2: &'a Self) -> &'a Self {
//         if rect1.area() > rect2.area() {
//             rect1
//         } else {
//             rect2
//         }
//     }

//     fn switcheroo(&mut self, other: &mut Self) {
//         Self::mix_up(self, other);
//         Self::mix_up(self, other);
//     }
// }

// impl<T: Dimension> PartialEq for Rectangle<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.width.eq(&other.width) && self.height.eq(&other.height)
//     }
// }

// impl<T: Dimension> PartialOrd for Rectangle<T> {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         if self.eq(&other) {
//             Some(Ordering::Equal)
//         } else if self.width > other.width && self.height > other.height {
//             Some(Ordering::Greater)
//         } else if self.width < other.width && self.height < other.height {
//             Some(Ordering::Less)
//         } else {
//             None
//         }
//     }
// }

fn main() {
    let equations = vec![
        vec!["x1".to_string(), "x2".to_string()],
        vec!["x2".to_string(), "x3".to_string()],
        vec!["x3".to_string(), "x4".to_string()],
        vec!["x4".to_string(), "x5".to_string()],
    ];
    let values = vec![3.0, 4.0, 5.0, 6.0];
    let queries = vec![
        vec!["x1".to_string(), "x5".to_string()],
        vec!["x5".to_string(), "x2".to_string()],
        vec!["x2".to_string(), "x4".to_string()],
        vec!["x2".to_string(), "x2".to_string()],
        vec!["x2".to_string(), "x9".to_string()],
        vec!["x9".to_string(), "x9".to_string()],
    ];
    let results = Solution::calc_equation(equations, values, queries);
    println!("{:?}", results);
}
