use proconio::input;
fn main() {
    input! {
        a:i32,
        bc:[i32;2],
        s:String
    }
    let sum = a + bc[0] + bc[1];
    println!("{} {}", sum, s);
}
