use std::time::Duration;

fn main() {
    fn build_vector() -> Vec<i16> {
        let mut v: Vec<i16> = Vec::<i16>::new();
        v.push(10i16);
        v
    }
    fn build_vector_2() -> Vec<i16> {
        let mut v = Vec::new();
        v.push(10);
        v
    }

    // let byte_lit = b'あ'; ←コンパイルエラー
    let byte_lit = b'"';
    let byte_lit_2 = b'A';

    assert_eq!(byte_lit, 34_u8);
    assert_eq!(byte_lit_2, 65_u8);

    // println!("{}", (-4).abs()); ←コンパイルエラー
    println!("{}", (-4_i32).abs());
    println!("{}", i32::abs(-4));
    // println!("{}", (4_u8).abs()); ←コンパイルエラー

    // let mut i = 1;
    // loop {
    //     i *= 10;
    //     println!("{}", i);
    //     std::thread::sleep(Duration::from_secs_f32(0.5));
    // }

    // wrap演算の例
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);
    assert_eq!(500_i16.wrapping_mul(500), -12144);

    // check付き演算の例
    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);
    let x = 100_u8;
    let y = 200_u8;
    // let sum = x.checked_add(y).unwrap(); ←panic

    // 飽和演算の例
    assert_eq!(253_u8.saturating_add(10), 255);

    // オーバーフロー演算の例
    assert_eq!(253_u8.overflowing_add(10), (253_u8.wrapping_add(10), true));

    // 浮動小数点数のリテラル
    let pi = 31415.92e-4f64;
    println!("{}", pi);

    // 正負の無限大、NAN, MINMAXがある
    assert_eq!(-1. / f32::INFINITY, 0.0);
    assert_eq!(-f32::MIN, f32::MAX);
}
