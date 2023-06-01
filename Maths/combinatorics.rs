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

fn ir(val: usize, n: usize) -> bool {
    val >= 0 && val <= n
}

fn nc_ijk_memoize(i: usize, j: usize, k: usize, mem: &mut Vec<Vec<Vec<u128>>>) -> u128 {
    if !ir(j, i) || !ir(k, j) {
        0
    } else if mem[i][j][k] > 0 {
        mem[i][j][k]
    } else if (j == 0 && k == 0) || (j == i && k == 0) || (j == i && k == i) {
        mem[i][j][k] = 1;
        mem[i][j][k]
    } else {
        mem[i][j][k] = nc_ijk_memoize(i.wrapping_sub(1), j, k, mem)
            + nc_ijk_memoize(i.wrapping_sub(1), j.wrapping_sub(1), k, mem)
            + nc_ijk_memoize(i.wrapping_sub(1), j.wrapping_sub(1), k.wrapping_sub(1), mem);
        mem[i][j][k]
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

#[test]
pub fn test_nc_ijk_memoize() {
    assert_eq!(
        nc_ijk_memoize(4, 2, 1, &mut vec![vec![vec![0; 5]; 5]; 5]),
        12
    );
    assert_eq!(
        nc_ijk_memoize(5, 3, 2, &mut vec![vec![vec![0; 6]; 6]; 6]),
        30
    );
}

pub fn main() {
    let mut p = vec![vec![vec![0; 11]; 11]; 11];
    for x in 0..10 {
        for y in 0..=x {
            for z in 0..=y {
                print!("{} ", nc_ijk_memoize(x, y, z, &mut p));
            }
            println!("");
        }
        println!("");
    }
}
