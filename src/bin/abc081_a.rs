use proconio::input;
fn main() {
    input! {
        s:String
    }
    let cnt = s.as_str().match_indices("1").count();
    println!("{}", cnt);
}
