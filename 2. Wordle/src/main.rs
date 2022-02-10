extern crate termios;
use std::io;
use std::io::Read;
use std::io::Write;
use std::collections::HashMap;

use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

fn main() {
    print_help();
    let mut x = String::new(); 
    println!("{}", x);
    x.push(let_user_type_letter());
    println!("{}", x);
    x.push(let_user_type_letter());
    println!("{}", x);
    x.push(let_user_type_letter());
    println!("{}", x);
    x.push(let_user_type_letter());
    println!("{}", x);
}

fn print_help() {
    println!("This is a mimic of the game 'Wordle', where a new five-letter word is guessed each day.");
    println!("Below is a series of underscores, indicating your place to type.");
    println!("Once you have entered in your five-letter word, each character will change colors. The color indicates:");
    println!(" RED   - The letter is not in the word");
    println!(" WHITE - The letter is in the wrong place, but is in the word");
    println!(" GREEN - The letter is in the right place");
    println!("\nGood luck!");
    println!("");
}

// after the word has been entered, the colored results are displayed in terminal
fn show_entry(){
}

// fetch word from file
fn get_word() {
}


fn getWordEntries(word:String) -> HashMap<char, Vec<u8>> {
    let mut entries = HashMap::new();
    let wordVec : Vec<char> = word.chars().collect();
    for idx in 0..4{
        let letter = wordVec[idx];
        let idxVec = entries.entry(letter).or_insert(Vec::new() as Vec<u8>);
        idxVec.push(idx as u8);
    }
    entries
}

// guess -> user entry, word -> the word to guess
// ensures 'guess' is in the list of words
// returns an array containing five elements with values 
//   "c" for correct, 
//   "m" for misplaced, and 
//   "w" for wrong

fn let_user_guess(guess:String, word:String) -> [char; 5]{
    let results : [char; 5];
    let guessVec : Vec<char> = guess.chars().collect();
    //create hashmap of char : list<u8>
    //each char = an entry in the word;
    //each u8 = an idx;
    let mut wordEntries : HashMap<char, Vec<u8>> = getWordEntries(word);
    for idx in 0..4 {
        let guessLet = &guessVec[idx];
        results[idx] = 'w';
        let idx8 = idx as u8;
        if !wordEntries.get(guessLet).is_none() && wordEntries.get(guessLet).unwrap().contains(&idx8) {
            results[idx] = 'c';
            wordEntries.get(guessLet).unwrap().remove(idx);
        }
        else if !wordEntries.get(guessLet).is_none() {
            results[idx] = 'm';
        }
    }
    results
}

fn let_user_type_letter() -> char {
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();

    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];
    print!("Hit a key ");
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    println!("You entered: {:?}", buffer[0] );
    tcsetattr(stdin, TCSANOW, & termios).unwrap();

    buffer[0] as char
}

