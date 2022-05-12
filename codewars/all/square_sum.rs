fn main()
{
    println!("Hello, world!");
    test!();
}
// My Solution
fn square_sum(vec: Vec<i32>) -> i32
{
    let result = vec.iter().map(|x| x.pow(2)).sum::<i32>();
    print!("Result:= {}\n", result);
    return result
}
// Varian 1
fn square_sum1(vec: Vec<i32>) -> i32 {
    vec.iter().map(|s| s * s).sum()
}
// Variant 2
fn square_sum2(vec: Vec<i32>) -> i32 {
    vec.iter().fold(0, |t,i| t + i*i)
}
// Variant 3
mod foo {
    pub trait SquareExt {
        type Output;
        fn square(self) -> Self::Output;
    }
    use std::ops::Mul;
    impl<T> SquareExt for T where T: Mul + Copy, {
        type Output = <T as Mul>::Output;
        fn square(self) -> Self::Output { self * self }
    }
}
fn square_sum3(vec: Vec<i32>) -> i32 {
    use crate::foo::SquareExt;
    vec.iter().fold(0, |a, b| a + b.square())
}
// Variant 4
fn square_sum4(vec: Vec<i32>) -> i32 {
    let mut squared_sum: i32 = 0;
    for num in vec {
        squared_sum += num*num;
    }
    squared_sum
}

#[test]
fn returns_expected() {
    assert_eq!(square_sum(vec![1, 2]), 5);
    assert_eq!(square_sum(vec![-1, -2]), 5);
    assert_eq!(square_sum(vec![5, 3, 4]), 50);
}