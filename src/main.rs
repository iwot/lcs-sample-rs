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
    println!("Longest Common Subsequence: {}", res);

    let res = calc_ld(&s, &t);
    println!("Levenshtein distance: {}", res);
    Ok(())
}

// 最長共通距離部分列問題
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

// レーベンシュタイン距離
fn calc_ld(s: &Vec<char>, t: &Vec<char>) -> usize {
    if s.len() == 0 {
        return t.len();
    }
    if t.len() == 0 {
        return s.len();
    }
    if s[0] == t[0] {
        return calc_ld(&s[1..].to_vec(), &t[1..].to_vec());
    }

    let res1 = calc_ld(s, &t[1..].to_vec());
    let res2 = calc_ld(&s[1..].to_vec(), t);
    let res3 = calc_ld(&s[1..].to_vec(), &t[1..].to_vec());

    1 + cmp::min(res1, cmp::min(res2, res3))
}