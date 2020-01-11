#[macro_use] extern crate prettytable;

use std::env::args;
use prettytable::{Table, format};


fn main() {
    if args().len() == 1 {
        println!("At least one number needs to be passed.");
        return;
    }

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row![bc => "Decimal", "Hexadecimal", "Octal", "Binary"]);

    // for each argument (skip first because it's the program name)
    for arg in args().skip(1){
        // try converting to dec and then print in table
        match {
            if arg.len() <= 2{
                usize::from_str_radix(&arg, 10)
            } else {
                let base = match &arg[..=1] {
                "0b" => 2,
                "0o" => 8,
                "0x" => 16,
                _ => 10,
                };

                if base != 10 {
                    usize::from_str_radix(&arg[2..].to_string(), base)
                } else {
                    usize::from_str_radix(&arg, 10)
                }

            }
        }
        // actual matching starts here
        {
            // Value successfully converted
            Ok(value) => 
                table.add_row(row![r => format!("{}", value),
                    format!("{:#X}", value),
                    format!("{:#o}", value),
                    format!("{:#b}", value)]),

            // ERROR while converting
            _ => table.add_row(row![c => "Error", "while", "converting", format!("\"{}\"", &arg)]),
        };
    }

    table.printstd();
}
