//! 1146. Snapshot Array
//! https://leetcode.com/problems/snapshot-array

pub struct SnapshotArray {
    snap_id: i32,
    data: Vec<Vec<(i32, i32)>>,
}

impl SnapshotArray {
    pub fn new(length: i32) -> Self {
        Self {
            snap_id: 0,
            data: vec![Vec::new(); length as usize],
        }
    }

    pub fn set(&mut self, index: i32, val: i32) {
        let snaps = &mut self.data[index as usize];
        match snaps.last_mut() {
            Some(last) if last.0 == self.snap_id => last.1 = val,
            Some(_) | None => snaps.push((self.snap_id, val)),
        };
    }

    pub fn snap(&mut self) -> i32 {
        let id = self.snap_id;
        self.snap_id += 1;
        id
    }

    pub fn get(&self, index: i32, snap_id: i32) -> i32 {
        let snaps = &self.data[index as usize];
        match snaps.binary_search_by_key(&snap_id, |v| v.0) {
            Err(0) => 0,
            Err(i) => snaps[i - 1].1,
            Ok(i) => snaps[i].1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_0() {
        // [5, 0, 0]
        // [6, 0, 0]
        let mut arr = SnapshotArray::new(3);
        arr.set(0, 5);
        assert_eq!(0, arr.snap());
        arr.set(0, 6);
        assert_eq!(5, arr.get(0, 0));
    }

    #[test]
    fn test_case_1() {
        // [15]
        // [^]
        // [^]
        // [^]
        // [^]
        // 0..=4 => 15
        let mut arr = SnapshotArray::new(1);
        arr.set(0, 15);
        assert_eq!(0, arr.snap());
        assert_eq!(1, arr.snap());
        assert_eq!(2, arr.snap());
        assert_eq!(15, arr.get(0, 2));
        assert_eq!(3, arr.snap());
        assert_eq!(4, arr.snap());
        assert_eq!(15, arr.get(0, 0));
    }

    #[test]
    fn test_against_overflow() {
        const I: i32 = 49999;
        const N: i32 = 50000;
        const V: i32 = 1000000000;
        let mut arr = SnapshotArray::new(N);
        for _ in 0..N {
            arr.set(I, V);
            arr.snap();
        }
    }
}
