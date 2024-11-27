use std::io;

fn main() {
    println!("Please enter a number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    println!(
        "The {input}{} fibonacci number is {}",
        get_number_ordinal(input),
        fibonacci_recursive(input)
    )
}

fn fibonacci_recursive(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

fn get_number_ordinal(num: u32) -> &'static str {
    if num < 20 && num > 10 {
        return "th";
    } else if num % 10 == 1 {
        return "st";
    } else if num % 10 == 2 {
        return "nd";
    } else if num % 10 == 3 {
        return "rd";
    } else {
        return "th";
    }
}
