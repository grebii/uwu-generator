/*
this program is really epic,
it takes whatever is in your clipboard and translates it to "uwu"
AND THEN IT GIVES IT BACK TO YOUR CLIPBOARD AD WAD
        ITS SO EPIC BROOO
*/

use std::env;

use clipboard::{
    ClipboardProvider,
    ClipboardContext,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {              
    // create a the clipboard
    let mut clipboard = ClipboardContext::new()?;

    // get input from enviroment argument or the clipboard
    let input = env::args().nth(1)
        .unwrap_or_else(|| clipboard.get_contents()
            .unwrap_or_else(|e| e.to_string()) );
    
    println!("\n    << {}", input);
    println!(r#"        _   ___      ___   _        "#);
    println!(r#"       | | | \ \ /\ / / | | |       "#);
    println!(r#"       | |_| |\ V  V /| |_| |       "#);
    println!(r#"        \__,_| \_/\_/  \__,_|       "#);

    // convert input to uwu
    let cringe: String = input.chars().map(|c| {
        match c {
            'l' | 'r' => 'w',
            'L' | 'R' => 'W',
            c => c,
        }
    }).collect();

    println!("\n    >> {}", cringe);

    // copy the cringerino message
    clipboard.set_contents(cringe)?;
    println!("\n    (copied to clipboard)");
    Ok(())
}