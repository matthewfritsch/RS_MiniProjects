use colored::*;
use std::io;
use std::io::Read;
use std::io::Write;
use std::fs;
use rand::prelude::Rng;

//use std::collections::HashMap;

use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};


fn main() {
    print_help(); //show help dialogue
    let filename = "words_fixed.txt".to_string();
    let file_words = get_file_contents(filename);
    let correct_word = get_word(&file_words);
    let mut guesses = 6;
    let mut user_guess = String::new();
    while guesses > 0 && user_guess != correct_word {
        user_guess = let_user_guess(&file_words); //let the user type in the letters
        let comparison_results = compare_user_guess(&user_guess, &correct_word);
        show_colorful_result(&user_guess, &comparison_results);
        guesses -= 1;
    }
    if user_guess != correct_word {
        println!("The correct word was '{}'", correct_word);
    }
    else {
        println!("You got it!");
    }
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

fn show_colorful_result(user_guess : &String, results : &[char; 5]) {
    let guess_letters : Vec<char> = user_guess.chars().collect();
    for idx in 0..5 {
        if results[idx] == 'c' {print!("{}", guess_letters[idx].to_string().green());}
        else if results[idx] == 'm' {print!("{}", guess_letters[idx].to_string().white());}
        else {print!("{}", guess_letters[idx].to_string().red());}
        print!("{}", "".to_string().normal());
    }
    println!();
    
}

// fetch word from file
fn get_word(file_contents : &Vec<String>) -> String {
    let mut random_generator = rand::thread_rng();
    let rand_word_idx = random_generator.gen::<usize>()%file_contents.len();
    let word : & String = &file_contents[rand_word_idx];
    word.to_string()
}

fn get_file_contents(filename:String) -> Vec<String> {
    let contents = fs::read_to_string(filename)
                    .expect("File error, could not fetch file contents");
    let mut words : Vec<String> = Vec::new();
    for l in contents.lines() {
        words.push(l.to_string());
    }
    words
}

/*
fn get_word_entries(word:String) -> HashMap<char, Vec<u8>> {
     let mut entries = HashMap::new();
    let word_vec : Vec<char> = word.chars().collect();
    for idx in 0..4{
        let letter = word_vec[idx];
        let idx_vec = entries.entry(letter).or_insert(Vec::new() as Vec<u8>);
        idx_vec.push(idx as u8);
    }
    entries
}
*/

// guess -> user entry, word -> the word to guess
// ensures 'guess' is in the list of words
// returns an array containing five elements with values 
//   "c" for correct, 
//   "m" for misplaced, and 
//   "w" for wrong

/*
fn let_user_guess(guess:String, word:String) -> [char; 5]{
    let mut results : [char; 5] = [' '; 5];
    let guess_vec : Vec<char> = guess.chars().collect();
    //create hashmap of char : list<u8>
    //each char = an entry in the word;
    //each u8 = an idx;
    let word_entries : HashMap<char, Vec<u8>> = get_word_entries(word);
    for idx in 0..4 {
        let guess_let = &guess_vec[idx];
        results[idx] = 'w';
        let idx8 = idx as u8;
        if !word_entries.get(guess_let).is_none() && word_entries.get(guess_let).unwrap().contains(&idx8) {
            results[idx] = 'c';
        }
        else if !word_entries.get(guess_let).is_none() {
            results[idx] = 'm';
        }
    }
    results
}
*/

//TODO reimplement hashmap 

// returns the string containing the user's guess
fn let_user_guess(file_contents : &Vec<String>) -> String {
    let mut x = String::new(); 
    while x.len() < 5 {
        let letter = let_user_type_letter();
        if letter as u8 == 127 {
            if x.len() > 0 {
                x = x[0..x.len()-1].to_string();
            }
        }
        else {
            x.push(letter);
        }
        //flush console line so the user can 
        print!("\r      ");
        io::stdout().flush().unwrap();
        print!("\r{}", x);
        io::stdout().flush().unwrap();
    }
    if !file_contents.contains(&x) {
        println!("\r{} is not in our dictionary. Please try another word.", x);
        return let_user_guess(file_contents)
    }
    x
}

fn compare_user_guess(guess:&String, word:&String) -> [char; 5] {

    let mut results = [' '; 5];
    let guess_vec : Vec<char> = guess.chars().collect();
    let mut correct_vec : Vec<char> = word.chars().collect();

    for idx in 0..5 {
        //println!("Are {} and {} the same?", guess_vec[idx], correct_vec[idx]);
        if guess_vec[idx] == correct_vec[idx] { results[idx] = 'c'; }
    }
    for idx in 0..5 {
        if results[idx] == 'c' {
            correct_vec[idx] = ' ';
            continue;
        };
        for jdx in 0..5 {
            if jdx == idx || results[jdx] == 'c' {continue};
            //println!("Is {} the same as {}?", guess_vec[idx], guess_vec[jdx]);
            if guess_vec[idx] == correct_vec[jdx] {
                results[idx] = 'm';
                correct_vec[jdx] = ' ';
                break;
            }
            else {
                results[idx] = 'w';
            }
        }
    }
    print!("\r");
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
    //print!("Hit a key ");
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    //println!("You entered: {:?}", buffer[0] );
    tcsetattr(stdin, TCSANOW, & termios).unwrap();

    buffer[0] as char
}

