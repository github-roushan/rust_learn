use std::io::{self, Write};

// const values are computed at compile time
const HOURS_IN_SECONDS: u32 = 60 * 60;

fn main() {
    let mut x = String::new();
    let total_hours: u32;

    loop {
        print!("Enter the number of hours (e.g., 1, 5, 12): ");
        io::stdout().flush().unwrap();
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

    // Data Types
    // Integer types
    let _ia = -1i8;
    let _ua =  1u8;
    // and similary for i16, i32, i64, i128, isize and their unsigned variants

    // Types of Number Literals
    let _dec = 89_1233;
    let _hex = 0xff;
    let _oct = 0o27;
    let _bin = 0b01_10_11;

    println!("{_dec} {_hex} {_oct}");

    // overflow handling methods

    let a: u8 = 250;
    let b: u8 = 10;

    let _result = a.wrapping_add(b);
    let _result = a.checked_add(b);
    let _result = a.overflowing_add(b);
    let _result = a.saturating_add(b);


    let _def_float_var = 124.566;
    let _other_float_var: f32 = 1.245;

    // numeric operations
    let _sum = 4 + 10;
    let _diff = 5 - 2;
    let _product = 99 * 73;
    let _remained = 67 % 3;

    // boolean type
    let _tbool = true;

    // Character Types
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");
}
