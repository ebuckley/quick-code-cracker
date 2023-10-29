use std::collections::HashMap;
use std::fs::read_to_string;

// code cracker program in rust..
// the goal is to find the index, which is basically a 1-26 of the alphabet.

fn main() {
    let dictionary_path = "/usr/share/dict/american-english";

    // code represents the letters known
    let mut code: [Option<char>; 26] = [
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some('R'),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ];

    let hint = Vec::from([1, 7, 7, 14, 7, 21]);


    let mut words = Vec::new();
    for line in read_to_string(dictionary_path).unwrap().lines() {
        // if it contains ' then strip it out and save it...
        let mut thisline = line.clone().to_string();
        if line.contains("'") {
            thisline = thisline.replace("'", "").to_string();
        }
        words.push(thisline.to_uppercase());
    }

    print_hint(hint.clone(), code);
    let hint = &hint;
    let opts = find_options(hint, code, words);
    for opt in opts.iter() {
        println!("{}", opt)
    }
}

fn print_hint(hint: Vec<i32>, code: [Option<char>; 26]) {
    let mut out = String::new();
    for elem in hint.iter() {
        let codeindex = (elem - 1) as usize;
        let opt = code[codeindex];
        match opt {
            Some(v) => {
                out = format!("{0} {1}", out, v)
            }
            None => {
                out = format!("{0} {1}", out, elem)
            }
        }
    }
    println!("{}", out)
}

fn to_letter_map(hint: &Vec<i32>, code: [Option<char>; 26]) -> HashMap<usize, char> {
    let mut out = HashMap::new();
    for n in 1..hint.len() {
        let elem = hint[n];
        let codeindex = (elem - 1) as usize;
        let opt = code[codeindex];
        match opt {
            Some(v) => {
                out.insert(n, v);
            }
            None => {
                continue
            }
        }
    }
    out
}

fn find_options(hint: &Vec<i32>, code:[Option<char>; 26], words: Vec<String>) -> Vec<String> {
    let word_length = hint.len();
    let word_vec = to_letter_map(hint, code);
    let mut options = Vec::new();
    'outer: for word in words.iter() {
        if word.len() != word_length {
            continue
        }
        for (index, word_char) in word_vec.iter() {

            if word.chars().nth(*index).is_some_and(|c| c != *word_char){
                continue 'outer
            }
        }
        // this might be it!
        options.push(word.clone())
    }
    options
}