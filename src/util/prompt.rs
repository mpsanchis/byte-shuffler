use std::io::{self, Write};

pub fn ask_yes_no(prompt: &str) -> bool {
    // Loop until we get a valid answer
    loop {
        print!("{prompt} [y/N]: ");
        io::stdout().flush().unwrap(); // make sure the prompt is shown immediately

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim().to_lowercase();

        match trimmed.as_str() {
            "" => return false,         // default = No
            "y" | "yes" => return true, // accept yes
            "n" | "no" => return false, // accept no
            _ => {
                println!("Please answer 'y' or 'n'.");
            }
        }
    }
}
