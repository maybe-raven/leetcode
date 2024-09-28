//! 864. Shortest Path to Get All Keys
//! https://leetcode.com/problems/shortest-path-to-get-all-keys

use std::{
    collections::VecDeque,
    ops::{BitAnd, Index, IndexMut},
    result,
};

#[derive(Debug, Clone, Copy)]
enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
}

impl TryFrom<char> for Key {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' => Ok(Self::A),
            'b' => Ok(Self::B),
            'c' => Ok(Self::C),
            'd' => Ok(Self::D),
            'e' => Ok(Self::E),
            'f' => Ok(Self::F),
            _ => Err(()),
        }
    }
}

impl From<Key> for usize {
    fn from(value: Key) -> Self {
        match value {
            Key::A => 0,
            Key::B => 1,
            Key::C => 2,
            Key::D => 3,
            Key::E => 4,
            Key::F => 5,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Lock {
    A,
    B,
    C,
    D,
    E,
    F,
}

impl TryFrom<char> for Lock {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::A),
            'B' => Ok(Self::B),
            'C' => Ok(Self::C),
            'D' => Ok(Self::D),
            'E' => Ok(Self::E),
            'F' => Ok(Self::F),
            _ => Err(()),
        }
    }
}

impl From<Lock> for usize {
    fn from(value: Lock) -> Self {
        match value {
            Lock::A => 0,
            Lock::B => 1,
            Lock::C => 2,
            Lock::D => 3,
            Lock::E => 4,
            Lock::F => 5,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Player,
    Ground,
    Wall,
    PickUp(Key),
    Locked(Lock),
    Unlocked(Lock),
}

impl Tile {
    fn is_wall(self) -> bool {
        matches!(self, Self::Wall)
    }

    fn is_traversable(self) -> bool {
        matches!(
            self,
            Self::Player | Self::Ground | Self::PickUp(_) | Self::Unlocked(_)
        )
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ground),
            '#' => Ok(Self::Wall),
            '@' => Ok(Self::Player),
            x => {
                if let Ok(key) = Key::try_from(x) {
                    Ok(Self::PickUp(key))
                } else if let Ok(lock) = Lock::try_from(x) {
                    Ok(Self::Locked(lock))
                } else {
                    Err(())
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

struct KeyLocks {
    keys: [Option<Coordinate>; 6],
    locks: [Option<Coordinate>; 6],
}

impl Index<Key> for KeyLocks {
    type Output = Option<Coordinate>;

    fn index(&self, index: Key) -> &Self::Output {
        self.keys.index(usize::from(index))
    }
}

impl Index<Lock> for KeyLocks {
    type Output = Option<Coordinate>;

    fn index(&self, index: Lock) -> &Self::Output {
        self.locks.index(usize::from(index))
    }
}

const MAX_DIMENSION: usize = 30;
type BoardArr<T> = [[T; MAX_DIMENSION]; MAX_DIMENSION];
struct Board(BoardArr<Tile>);

impl Board {
    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0.first().map(|row| row.len()).unwrap_or(0)
    }

    fn get_neighbors(&self, Coordinate { x, y }: Coordinate) -> [Option<Coordinate>; 4] {
        [
            if 0 < y {
                None
            } else {
                Some(Coordinate { x, y: y - 1 })
            },
            if 0 < x {
                None
            } else {
                Some(Coordinate { x: x - 1, y })
            },
            if y < self.height() - 1 {
                None
            } else {
                Some(Coordinate { x, y: y + 1 })
            },
            if x < self.width() - 1 {
                None
            } else {
                Some(Coordinate { x: x + 1, y })
            },
        ]
    }

    // fn get_neighbors(&self, Coordinate { x, y }: Coordinate) -> impl Iterator<Item = Coordinate> {
    //     if 0 < y {
    //         None
    //     } else {
    //         Some(Coordinate { x, y: y - 1 })
    //     }
    //     .into_iter()
    //     .chain(if 0 < x {
    //         None
    //     } else {
    //         Some(Coordinate { x: x - 1, y })
    //     })
    //     .chain(if y < self.height() - 1 {
    //         None
    //     } else {
    //         Some(Coordinate { x, y: y + 1 })
    //     })
    //     .chain(if x < self.width() - 1 {
    //         None
    //     } else {
    //         Some(Coordinate { x: x + 1, y })
    //     })
    // }

    fn generate_distance(&self, start: Coordinate) -> BoardArr<Option<u8>> {
        let mut results = BoardArr::default();
        results[start] = Some(0);
        let mut queue: VecDeque<Coordinate> = VecDeque::from([start]);

        while let Some(coord) = queue.pop_front() {
            let distance = results[coord].unwrap() + 1;
            for neighbor in self.get_neighbors(coord).into_iter().flatten() {
                if self.0[neighbor].is_traversable() && results[neighbor].is_none() {
                    results[coord] = Some(distance);
                    queue.push_back(neighbor);
                }
            }
        }

        results
    }
}

impl<T> Index<Coordinate> for BoardArr<T> {
    type Output = T;

    fn index(&self, index: Coordinate) -> &Self::Output {
        self.index(index.y).index(index.x)
    }
}

impl<T> IndexMut<Coordinate> for BoardArr<T> {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        self.index_mut(index.y).index_mut(index.x)
    }
}

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let grid: Vec<Vec<Tile>> = grid
            .into_iter()
            .map(|s| s.chars().map(Tile::try_from).collect())
            .collect::<Result<_, ()>>()
            .unwrap();

        let mut queue: Vec<Coordinate> = Vec::new();

        let row_masks: Vec<i32> = grid
            .iter()
            .map(|row| {
                row.iter()
                    .rev()
                    .enumerate()
                    .filter_map(|(i, x)| if x.is_wall() { Some(1 << i) } else { None })
                    .reduce(BitAnd::bitand)
                    .unwrap_or(0)
            })
            .collect();

        unimplemented!()
    }
}

pub struct Solution;
