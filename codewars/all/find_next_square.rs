// My Solution
fn find_next_square(sq: u64) -> Option<u64> {
    let root_value = (sq as f64).sqrt();
    if root_value != root_value.floor()
    {
        return None
    }
    Some((root_value as u64 + 1).pow(2))
}
// Ex 1
fn find_next_square1(sq: u64) -> Option<u64> {
    let num = (sq as f64).sqrt();
    if num.fract() > 0f64 {
        return Option::None
    }
    let ret = (num as u64) + 1;
    return Option::Some(ret.pow(2))
}
// Ex 2
fn find_next_square2(sq: u64) -> Option<u64> {
    let dsquare = ((sq as f64).sqrt() + 1.0).powi(2);
    match dsquare.fract() {
        0.0 => Some(dsquare as u64),
        _ => None
    }
}
// Ex 3
fn find_next_square3(sq: u64) -> Option<u64> {
    let n = (sq as f64).sqrt();

    (n.fract() == 0.0).then(|| (n as u64 + 1).pow(2))
}

fn main() {
    assert_eq!(find_next_square(121), Some(144));
    assert_eq!(find_next_square(625), Some(676));
    assert_eq!(find_next_square(319_225), Some(320_356));
    assert_eq!(find_next_square(15_241_383_936), Some(15_241_630_849));
    assert_eq!(find_next_square(155), None);
    assert_eq!(find_next_square(342_786_627), None);
    println!("Hello, world!");
}
