fn main() {
    let n = 10;
    let x = nth_fibonacci(n);
    println!("{}th fibonnaci: {}", n, x)
}

fn nth_fibonacci(n: i32) -> i32 {
    let mut x = 0;
    let mut y = 1;
    let mut z = 0;

    println!("{}", x);
    println!("{}", y);
    for _ in 2..n {
        z = x + y;
        x = y;
        y = z;
        println!("{}", z);
    }
    // x 0 1 2
    // y 1 2 3
    // z 1 1 3

    z
}

