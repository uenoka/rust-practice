use proconio::input;
fn main() {
    input! {
        ab:[i32;2]
    }
    let ans;
    if ab[0] % 2 == 1 && ab[1] % 2 == 1 {
        ans = "Odd";
    } else {
        ans = "Even";
    }
    println!("{}", ans);
}
