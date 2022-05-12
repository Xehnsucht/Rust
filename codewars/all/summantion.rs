/*fn summation(n: i32) -> i32 {
   // factorial
   // 2 -> 3 : 1 + 2
    match n{
        0 => 1,
        1 => 1,
        _ => summation(n - 1) * n,
        }
}
*/
fn summation(n: i32) -> i32 {
    (0 .. n+1).fold(0, |a, b| a + b)
}
#[cfg(test)]
mod tests {
        use super::summation;
#[test]
    fn basic_tests() {
        assert_eq!(summation(1), 1);
        assert_eq!(summation(8), 36);
        assert_eq!(summation(22), 253);
        assert_eq!(summation(100), 5050);
        assert_eq!(summation(213), 22791);
    }
}
fn main()
{
    let x = summation(8);
    println!("x is {}",x);
    println!("x is {}", summation(1)); 
}
