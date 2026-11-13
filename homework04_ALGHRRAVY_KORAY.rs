#[derive(Debug, PartialEq)]
enum Either<L, R> { 
    Left(L), 
    Right(R) 
}

impl<L, R> Either<L, R> {
    fn new(x: R) -> Either<L, R> {
        Either::Right(x)
    }

    fn map<U>(self, f: fn(R) -> U) -> Either<L, U> {
        match self {
            Either::Right(val) => Either::Right(f(val)),
            Either::Left(err) => Either::Left(err),
        }
    }

    fn flat_map<U>(self, f: fn(R) -> Either<L, U>) -> Either<L, U> {
        match self {
            Either::Right(val) => f(val),
            Either::Left(err) => Either::Left(err),
        }
    }
}

fn divide(a: f64, b: f64) -> Either<String, f64> {
    if b == 0.0 {
        Either::Left(format!("cannot divide {} by 0", a))
    } else {
        Either::Right(a / b)
    }
}

fn reciprocal(x: f64) -> Either<String, f64> {
    divide(1.0, x)
}

fn sqrt(x: f64) -> Either<String, f64> {
    if x < 0.0 {
        Either::Left(format!("cannot take the square root of {}", x))
    } else {
        Either::Right(x.sqrt())
    }
}

fn main() {
    let test1 = divide(1.0, 16.0).flat_map(reciprocal).flat_map(sqrt);
    println!("Test 1 (1, 16): {:?}", test1); 

    let test2 = divide(8.0, 0.0).flat_map(reciprocal).flat_map(sqrt);
    println!("Test 2 (8, 0): {:?}", test2);

    let test3 = divide(0.0, 5.0).flat_map(reciprocal).flat_map(sqrt);
    println!("Test 3 (0, 5): {:?}", test3);

    let test4 = divide(-1.0, 4.0).flat_map(reciprocal).flat_map(sqrt);
    println!("Test 4 (-1, 4): {:?}", test4); 
}
