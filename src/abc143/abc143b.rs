use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let n: u32 = read();
    let d_list: Vec<u32> = (0..n).map(|_| read()).collect();
    let len = d_list.len();

    let mut ans = 0;

    for i in 0..len {
        for j in 0..i{
            if i == j {
                continue;
            }
            ans += d_list[i] * d_list[j];
        }
    }

    println!("{}", ans);
}
