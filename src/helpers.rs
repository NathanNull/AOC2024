#[allow(dead_code)]
pub fn first_n<const N: usize, T: Default + Copy>(iter: &mut impl Iterator<Item = T>) -> [T; N] {
    let mut ret: [T; N] = [T::default(); N];
    for idx in 0..N {
        ret[idx] = iter.next().unwrap();
    }
    ret
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

#[allow(dead_code)]
impl Direction {
    pub fn from_char(c: char) -> Self {
        match c {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            _ => panic!("Invalid direction"),
        }
    }

    pub fn offset(&self) -> (i64, i64) {
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Right => (1, 0),
            Left => (-1, 0),
        }
    }
}

#[allow(dead_code)]
pub fn lcm(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

#[allow(dead_code)]
fn gcd_of_two_numbers(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}