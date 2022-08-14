use std::{collections::HashMap, process::exit, io::stdin};

// const VALID_ARGS: &[&str; 5] = &["-c", "-p", "-C", "-P", "-h"];

enum Filters {
    Contains(Vec<char>),
    NotContains(Vec<char>),
    CharAt(Vec<(usize, char)>),
    CharNotAt(Vec<(usize, char)>),
}

impl Filters {
    fn from_hash_map(map: HashMap<char, Vec<String>>) -> Vec<Self> {
        let mut filters = vec![];
        for (key, val) in map {
            match key {
                'c' => filters.push(Self::Contains(Self::parse_to_chars(val))),
                'p' => filters.push(Self::CharAt(Self::parse_to_tuples(val))),
                'C' => filters.push(Self::NotContains(Self::parse_to_chars(val))),
                'P' => filters.push(Self::CharNotAt(Self::parse_to_tuples(val))),
                _ => panic!("got invalid argument")
            }
        }

        filters
    }

    // Accepts strings like "a b c d"
    // if inputs like "abcd" are provided, they will be split
    // non alphabetical characters will be ignored
    // empty inputs will be ignored
    fn parse_to_chars(values: Vec<String>) -> Vec<char> {
        let mut parsed_values = vec![];
        for value in values {
            for c in value.chars() {
                if c.is_alphabetic() {
                    parsed_values.push(c);
                } else {
                    eprintln!("Got non alphabetic char, will be ignored");
                }
            }
        }
        parsed_values
    }

    // accepts inputs like 1 a 2 b 3 c
    // if character is longer than one, it will be ignored
    // always start from number, then parse char, if order is not adhered to, skip to next numerical input
    // number can be between 1 and 5 both inclusive
    // all other inputs will be ignored
    fn parse_to_tuples(values: Vec<String>) -> Vec<(usize, char)>{
        let mut is_num = true;
        let mut parsed_vals = vec![];
        let mut tp = (0, 'n');
        for value in values {
            if value.len() != 1 {
                eprintln!("Only number or single chars allowed");
                continue;
            }
            let c = value.chars().next().unwrap(); // we checked that length is exactly one
            if is_num {
                if !c.is_ascii_digit() {
                    eprintln!("Got Invalid number.");
                    continue;
                }
                let d = c.to_digit(10).unwrap(); // we checked if it is a digit
                if d > 5 || d < 1 {
                    eprintln!("Digit must be in range 0 to 5");
                    continue;
                } 
                tp.0 = d as usize - 1;
                is_num = false;
            } else {
                if !c.is_alphabetic() {
                    eprintln!("Should be alphabetic char");
                    continue;
                }
                tp.1 = c;
                is_num = true;
                parsed_vals.push(tp);
                tp = (0, 'c');
            }
        }

        parsed_vals
    }
}

fn main() {

    let parsed_args = parse_args();
    if parsed_args.is_err() {
        println!("{}", parsed_args.unwrap_err());
        exit(1);
    }
    if parsed_args.as_ref().unwrap().contains_key(&'h') {
        print_help();
        exit(0);
    }
    let mut words = create_word_list_2();
    let unwraped_args = parsed_args.unwrap();
    if unwraped_args.len() > 0 {
        // do first filter according from input variables
        words = filer_one_round(words, Filters::from_hash_map(unwraped_args));
        println!("Current options:\n {:?}", words);
    }

    // main loop
    while words.len() > 1 {
        let mut filters = vec![];

        println!("Enter new letters that are in the word:");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("got bad input");
        filters.push(Filters::Contains(Filters::parse_to_chars(input.trim().split(' ').map(|string| string.to_string()).collect())));

        println!("Enter letters and known positions:");
        input = String::new();
        stdin().read_line(&mut input).expect("got bad input");
        filters.push(Filters::CharAt(Filters::parse_to_tuples(input.trim().split(' ').map(|string| string.to_string()).collect())));

        println!("Enter new letters that are not in the word:");
        input = String::new();
        stdin().read_line(&mut input).expect("got bad input");
        filters.push(Filters::NotContains(Filters::parse_to_chars(input.trim().split(' ').map(|string| string.to_string()).collect())));

        println!("Enter letters and positions, which are known to be false:");
        input = String::new();
        stdin().read_line(&mut input).expect("got bad input");
        filters.push(Filters::CharNotAt(Filters::parse_to_tuples(input.trim().split(' ').map(|string| string.to_string()).collect())));

        words = filer_one_round(words, filters);
        println!("Current options:\n {:?}", words);
    }

    println!("Only one option left, exiting");
}

fn filer_one_round(mut words: Vec<String>, filters: Vec<Filters>) -> Vec<String> {
    for filter in filters {
        words = match filter {
            Filters::Contains(vals) => filter_if_contains(words, &vals),
            Filters::NotContains(vals) => filter_if_not_contains(words, &vals),
            Filters::CharAt(vals) => filter_by_char_at_pos(words, &vals),
            Filters::CharNotAt(vals) => filter_by_char_not_at_pos(words, &vals),
        }
    }

    words
}

fn parse_args() -> Result<HashMap<char, Vec<String>>, String> {
    let args: Vec<String> = std::env::args().collect();
    let mut arg_map = HashMap::new(); 
    // check for invalid arguments
    for (i, arg) in args.iter().enumerate() {
        if arg.starts_with('-') {
            if let Some(mut v) = match arg.as_str() {
                "-c" => arg_map.insert('c', get_args(&args[i + 1..])),
                "-p" => arg_map.insert('p', get_args(&args[i + 1..])),
                "-C" => arg_map.insert('C', get_args(&args[i + 1..])),
                "-P" => arg_map.insert('P', get_args(&args[i + 1..])),
                "-h" => arg_map.insert('h', vec![]),
                opt => return Err(format!("Invalid opt: {}", opt))
            } {
                let vec = arg_map.get_mut(&arg[1..].chars().next().unwrap()).unwrap();
                vec.append(&mut v)
            }
        }
    }

    Ok(arg_map)
}

fn get_args(args: &[String]) -> Vec<String> {
    let mut options = vec![];
    for opt in args {
        if opt.starts_with('-') {
            break;
        } else {
            options.push(opt.clone());
        }
    }
    options
}

fn filter_if_contains(words: Vec<String>, pattern: &[char]) -> Vec<String> {
    if pattern.len() == 0 {
        words
    } else {
        words.into_iter().filter(|line| line.contains(pattern)).collect()
    }
}

fn filter_if_not_contains(words: Vec<String>, pattern: &[char]) -> Vec<String> {
    if pattern.len() == 0 {
        words
    } else {
        words.into_iter().filter(|line| !line.contains(pattern)).collect()
    }
}

fn filter_by_char_at_pos(mut words: Vec<String>, patterns: &[(usize, char)]) -> Vec<String> {
    if patterns.len() == 0 {
        words
    } else {
        for (pos, c) in patterns {
            words = words.into_iter().filter(|line| line.find(*c) == Some(*pos)).collect()
        }
        words
    }
}

fn filter_by_char_not_at_pos(mut words: Vec<String>, patterns: &[(usize, char)]) -> Vec<String> {
    if patterns.len() == 0 {
        words
    } else {
        for (pos, c) in patterns {
            words = words.into_iter().filter(|line| line.find(*c) != Some(*pos)).collect()
        }
        words
    }
}

fn create_word_list_2() -> Vec<String> {
    include_str!("data/words.txt").split('\n').map(|line| line.trim().to_string()).filter(|line| line.len() == 5).collect()
}

fn print_help() {
    let help_str = "
***********
* WHELPER *
***********    

A small helper to solve wordle puzzles. The application may be started with commandline arguments listed below under 'Usage'.


General tipps: 

It is always good to start with the word 'yeast' for reasons Jannika explained to me, which I have since long forgotten.


Description:

After starting the Application, the user is asked to provide certain inputs in order to filter the possible words. 
Inputs can be 'letters' and 'positions'.
A 'letter' is a single alphabetic ascii character. Several letters can be provided by separating them with spaces (' ').
A 'position' is a number between 1 and 5 (both inclusive) which indicates a certain position in a word.
Some options allow the user to specify letters at certain position. A single letter position pair is separated with a space, e.g. '1 b'. This would indicate that the word should start with the letter b.
Note that positions are not zero indexed, but start from one. Several position letter tuples can be entered 

The user is asked to enter inputs in the following order:

1. Enter new letters that are in the word:
    The user should enter letters that are known to be in the word, with an unknown position. 
    Example: If the user knows that the letters 'b' and 'c' are in the word but not where, he can provide an input like 'b c'.
2. Enter letters and known positions:
    The user should enter position letter pairs, which are known to be at a certain position.
    Example: If the user knows that the letter c is the third letter of the word, and h the fourth, he can provide and input like '3 c 4 h' 
3. Enter new letters that are not in the word:
    The user should enter letters that are known to not be in the word, with unknown positions. This works similar to (1).
4. Enter letters and positions, which are known to be false:
    The user should enter position letter pairs, which are known to be NOT at a certain position.
    Example: If the user knows that the letter i is not the fourth letter in the word, he can provid an input like '4 i'.

    
Usage:
    whelper [OPTIONS]

OPTIONS:
    -c (letter )*           List of letters separated by one space that are known to be in the word (e.g. -c a b c)
    -C (letter )*           List of letters separated by one space that are known to NOT be in the word (e.g. -C x y z)   
    -p (pos letter)*        List a pair of position and letters for which the position in the word are known (e.g. -p 1 a 5 b)
    -P (pos letter)*        List a pair of position and letters for which are known not to be at the specified position (e.g. -P 2 x 4 y)
    -h                      Print this help text";

    println!("{}", help_str);
}

fn _create_word_list() -> Vec<String> {
    // read in file
    let lines: Vec<String> = include_str!("data/input_raw.txt").split('\n').map(|line| line.trim().to_ascii_lowercase()).collect();
    // get words
    let filter_fn = _produce_filter_fn(5);
    lines.iter().filter(filter_fn).map(|line| line[4..9].to_string()).collect()
}

fn _produce_filter_fn(word_length: usize) -> impl FnMut(&&String) -> bool {
    move |line| line.starts_with("<li>") && line.ends_with("</li>") && line.len() == 4 + 5 + word_length
}

#[test]
fn test_string_filter() {
    let line = "<li>yeast</li>";
    assert!(line.starts_with("<li>") && line.ends_with("</li>"));
    assert_eq!(line.len(), 14);
}