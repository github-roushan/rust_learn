use std::io::{self, Write};

// const values are computed at compile time
const HOURS_IN_SECONDS: u32 = 60 * 60;

fn main() {
    let mut x = String::new();
    let total_hours: u32;

    print!("Enter the number of hours (e.g., 1, 5, 12):");
    io::stdout().flush().unwrap();
    loop {
        x.clear();
        io::stdin()
            .read_line(&mut x)
            .expect("Failed to read the variable");
        let parsed_value = x.trim().parse::<u32>();
        total_hours = match parsed_value {
            Ok(num) => num,
            Err(e) => {
                println!("Invalid input: {e}");
                continue;
            }
        };
        break;
    }

    println!(
        "Total Seconds in {} hours is {}",
        total_hours,
        total_hours * HOURS_IN_SECONDS
    );
}
