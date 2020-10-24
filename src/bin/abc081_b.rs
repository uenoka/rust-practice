use proconio::input;
fn main() {
    input! {
        n:u32,
        mut a:[u32;n],
    }
    let mut cnt = 0;
    'outer: loop {
        for ai in a.iter_mut() {
            if *ai % 2 == 1 {
                break 'outer;
            }
            *ai = *ai / 2;
        }
        cnt += 1;
    }
    println!("{}", cnt);
}
