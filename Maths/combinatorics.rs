fn factorial(n: usize) -> u128 {
    if n <= 1 {
        1
    } else {
        (n as u128) * factorial(n - 1)
    }
}

fn derangement(n: usize) -> u128 {
    let mut result: i128 = 0;
    let mut mul = 1;
    let mut sign: i128 = if n & 1 == 0 { 1 } else { -1 };
    for x in ((0 as i128)..=(n as i128)).rev() {
        result += sign * mul;
        sign = -sign;
        mul *= x;
    }
    result as u128
}

fn ncr(n: usize, r: usize) -> u128 {
    if r > n {
        0
    } else if r == 0 || r == n {
        1
    } else {
        ncr(n - 1, r - 1) + ncr(n - 1, r)
    }
}

fn ncr_memoize(n: usize, r: usize, mem: &mut Vec<Vec<u128>>) -> u128 {
    if r > n {
        0
    } else if r == 0 || r == n {
        1
    } else if mem[n][r] == 0 {
        mem[n][r] = ncr_memoize(n - 1, r, mem) + ncr_memoize(n - 1, r - 1, mem);
        mem[n][r] as u128
    } else {
        mem[n][r] as u128
    }
}

#[test]
pub fn test_derangement() {
    assert_eq!(derangement(9), 133496);
    assert_eq!(derangement(8), 14833);
    assert_eq!(derangement(10), 1334961);
}

#[test]
pub fn test_ncr() {
    assert_eq!(ncr(10, 5), 252);
}

#[test]
pub fn test_factorial() {
    assert_eq!(factorial(10), 3628800);
}

#[test]
pub fn test_ncr_memoize() {
    assert_eq!(
        ncr_memoize(50, 25, &mut vec![vec![0; 51]; 51]),
        126410606437752
    );
    assert_eq!(ncr_memoize(15, 10, &mut vec![vec![0; 51]; 51]), ncr(15, 10));
}

pub fn main() {
    println!("{}", derangement(9));
}
