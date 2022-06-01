fn main() {
    let number = 3;

    // if
    // - if expect bool values only.
    // - expressions like let number = 3; if number ... // doesn't compile
    if number < 5 {
        println!("condition true");
    } else {
        println!("condition false");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0{
        println!("number is divisible by 3");
    } else if number % 2 == 0{
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if let
    // - as if is an expression, we can use it in the right side of a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // loop
    // - loop can have a label, starting with '
    // - In this example, the outer loop will count from 0 to 2, the inner loop down from 10 to 9.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // Returning Values from Loops
    // - As loop is an expression
    // - Set a value to return after the break keyword
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!();
    println!("The result is {}", result);

    // Conditional Loops with while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!");

    // Looping Through a Collection with for
    // - This approach is error prone as the program may panic if the index value or test condition
    // are incorrect. Also slow as the compiler adds runtime code to perform the conditional check
    // of whether the index is within the bounds of the array on every iteration.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5 {
        println!("the value in while is: {}", a[index]);
        index += 1;
    }

    // we can use a for
    for element in a {
        println!("the value in for is: {}", element);
    }

    // even to count
    // using the range construct
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!")



}
