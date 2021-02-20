use std::cmp;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut s = String::new();
    let mut t = String::new();

    std::io::stdin().read_line(&mut s)?;
    std::io::stdin().read_line(&mut t)?;
    let s: Vec<char> = s.trim().chars().collect();
    let t: Vec<char> = t.trim().chars().collect();
    let x = s.len();
    let y = t.len();

    let mut dp = vec![vec![-1; y+1]; x+1];
    let res = calc_lcs(&s, &t, x, y, &mut dp);
    println!("{}", res);
    Ok(())
}

fn calc_lcs(s: &Vec<char>, t: &Vec<char>, x: usize, y:usize, dp:&mut Vec<Vec<i64>>) -> i64 {
    if x == 0 && y == 0 {
        return 0;
    }
    if dp[x][y] != -1 {
        return dp[x][y];
    }
    let mut res = 0;
    if x > 0 {
        res = cmp::max(res, calc_lcs(s, t, x-1, y, dp));
    }
    if y > 0 {
        res = cmp::max(res, calc_lcs(s, t, x, y-1, dp));
    }
    if x > 0 && y > 0 && s[x-1] == t[y-1] {
        res = cmp::max(res, calc_lcs(s, t, x-1, y-1, dp) + 1);
    }

    dp[x][y] = res;
    res
}