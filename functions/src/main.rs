fn main() {

}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fib(n-1) + fib(n-2);
}

fn factorial(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }
    return n * factorial(n-1);
}

fn fb_loop(n: i32) {
    let mut i = 0;
    loop {
        if i == n {
            break;
        }
        fizz_buzz(i);
        i += 1;
    }
}

fn fizz_buzz(n: i32) {
    if n % 3 == 0 && n % 5 != 0 {
        println!("Fizz");
    }
    if n % 3 != 0 && n % 5 == 0 {
        println!("Buzz");
    }
    if n % 3 == 0 && n % 5 == 0 {
        println!("FizzBuzz");
    }
    if n % 3 != 0 && n % 5 != 0 {
        println!("None!");
    }
}