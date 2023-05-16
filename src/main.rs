use rand::Rng;
use std::io::{self, Write};
use clipboard::ClipboardProvider;

fn load_words() -> Vec<String> {
    let data = include_str!("eff_large_wordlist.txt");
    let lines: Vec<&str> = data.split('\n').collect();
    let words: Vec<String> = lines
        .iter()
        .map(|line| line.split_whitespace().nth(1))
        .filter_map(|word| word.map(|w| w.to_string()))
        .collect();
    words
}

fn generate_password(num_rolls: u32, words: &[String], custom_word: Option<&str>) -> String {
    let mut password = String::new();

    if let Some(word) = custom_word {
        password.push_str(word);
    }

    let mut rng = rand::thread_rng();

    for _ in 0..num_rolls {
        let index = rng.gen_range(0..words.len());
        if !password.is_empty() {
            password.push('-');
        }
        password.push_str(&words[index]);
    }

    password.trim().to_string()
}

fn ask_question(query: &str) -> io::Result<String> {
    print!("{}", query);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input.trim().to_string())
}

fn copy_to_clipboard(password: &str) {
    if let Err(err) = clipboard::ClipboardProvider::new()
        .and_then(|mut ctx: clipboard::ClipboardContext| ctx.set_contents(password.to_owned()))
    {
        eprintln!("Failed to copy to clipboard: {}", err);
    }
}

fn main() -> io::Result<()> {
    let words = load_words();
    
    loop {
        let num_rolls: u32 = match ask_question("Enter the number of dice rolls: ")?.parse() {
            Ok(num) if num >= 1 => num,
            _ => {
                eprintln!("Please provide a positive number for the number of dice rolls");
                continue;
            }
        };
        
        let custom_word = ask_question("Enter custom word to insert at the beginning of password (optional): ")?;
        
        let password = generate_password(num_rolls, &words, Some(&custom_word));
        println!("Generated Password: {}", password);
        
        let copy_option = ask_question("Copy password to clipboard? (Y/N): ")?;
        if copy_option.trim().to_lowercase() == "y" {
            copy_to_clipboard(&password);
            println!("Password copied to clipboard.");
        }
        
        let reroll_option = ask_question("Generate a new password? (Y/N): ")?;
        if reroll_option.trim().to_lowercase() != "y" {
            break;
        }
    }
    
    Ok(())
}
