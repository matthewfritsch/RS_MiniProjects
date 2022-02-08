//use std::env;
use std::fs;
use std::io::Write;

fn main() {
    
    let filename = "words.txt";
    
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");

    //println!("lines are \n{:?}", split_lines);

    let mut vec_words : Vec<String> = Vec::new();

    for l in contents.lines(){
        //println!("{} ", l.chars().count());
        if l.chars().count() == 5 {
            vec_words.push(l.to_string());
        }
    }

    let mut file = fs::File::create("words_fixed.txt").unwrap();

    for l in vec_words {
        writeln!(&mut file, "{}", l).unwrap();
    }

}
