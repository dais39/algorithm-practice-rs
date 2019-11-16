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

fn main(){
    let n: u32 = read();

    for i in 1..10{
        if n % i == 0 && n / i<= 9 {
            println!("Yes");
            return
        }else if i == 9{
            println!("No");
        }
    }
}