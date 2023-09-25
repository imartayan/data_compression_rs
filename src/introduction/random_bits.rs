#[test]
fn random_bits() {
    let n = 100;
    let mut x: i32 = 989511;
    for _ in 0..n {
        x = x.wrapping_mul(312523).wrapping_add(852596);
        print!("{}", if x > 0 { 1 } else { 0 });
    }
    println!();
}
