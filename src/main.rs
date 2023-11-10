use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;
use std::num::ParseIntError;
use std::panic::panic_any;
use log::{warn};

// code cracker program in rust..
// the goal is to find the index, which is basically a 1-26 of the alphabet.
#[derive(Debug)]
struct Problem {
    code: [Option<char>; 26],
    hints: Vec<Vec<i32>>
}

fn main() {
    env_logger::init();
    let dictionary_path = "/usr/share/dict/american-english";

    let problem_path = "/home/ersin/RustroverProjects/codecracker/src/example-problem.txt";

    let mut problem = load_problem(problem_path).unwrap();
    println!("loaded probelem: {:?}", problem);
    let words = get_dictionary(dictionary_path);

    for hint in problem.hints.iter() {

        let mut row1 = String::new();
        let mut row2 = String::new();
        for (i, c) in problem.code.iter().enumerate() {
            let c = c.unwrap_or('_');
            row2 = row2 + &*format!("{: ^3}", c);
            row1 = row1 + &*format!("{: ^3}", i + 1);
        }

        println!("{}\n{}", row1, row2);

        print_hint(hint.clone(), problem.code);
        let hint = &hint;
        let opts = find_options(hint, problem.code, &words);
        for (i, opt) in opts.iter().enumerate() {
            println!("{}: {}", i, opt)
        }
        println!("Choose an option");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Should have been able to read from stdin");

        let guess = match guess.trim().parse::<i32>(){
            Ok(v) => v,
            Err(err) => panic!("Error parsing {}:\n{}", guess, err)
        };

        let guess = opts.iter().nth(guess as usize).unwrap();
        println!("chose {}", guess);
        for (i, c) in guess.chars().enumerate() {
            let codeIndex = (hint.iter().nth(i).unwrap() - 1) as usize;
            problem.code[codeIndex] = match problem.code[codeIndex] {
                None => Some(c),
                Some(currentValue) => {
                    if currentValue != c {
                        warn!("shouldn't have been set")
                    }
                    Some(currentValue)
                }
            };
        }
        println!("-------------------------------------------");

    }


}

fn get_dictionary(dictionary_path: &str) -> Vec<String> {
    let mut words = Vec::new();
    for line in read_to_string(dictionary_path).unwrap().lines() {
        // if it contains ' then strip it out and save it...
        let mut thisline = line.to_string();
        if line.contains("'") {
            thisline = thisline.replace("'", "").to_string();
        }
        words.push(thisline.to_uppercase());
    }
    words
}

fn load_problem(path: &str) -> Option<Problem> {
    let lines =  match read_to_string(path) {
        Ok(v) => v,
        Err(e) => {
            warn!("did not expect error reading {} to string: {}", path, e);
            return None
        }
    };
    println!("GOT HERE!?!?!");
    let mut code: [Option<char>; 26] = [None; 26];
    let mut hints  = Vec::new();
    for (i, line) in lines.lines().enumerate() {
        // the first line is always the hint of the character code, subsequent are encoded hints
        if i == 0 {
            let mut parts = line.split(' ');

            let idx = match parts.next()?.parse::<i32>() {
                Ok(v) => (v - 1) as usize,
                Err(e) => {
                    warn!("Should not error parsing int: {}", e);
                    return None
                }
            };
            // TODO -- get char from value parsed out of the split..
            let val = parts.next()?;
            let val = val.to_string().chars().next().unwrap();
            code[idx] = Some(val);
            continue
        }
        let parts = line.split(' ');
        let mut hint = Vec::new();
        for part in parts {
            let code = part.to_string().parse::<i32>().unwrap();
            hint.push(code)
        }
        hints.push(hint)
    }
    return Some(Problem{
        hints,
        code
    })
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

// to_letter_map will return a map of the index in this word to a known character based on the code given
fn to_letter_map(hint: &Vec<i32>, code: [Option<char>; 26]) -> HashMap<usize, char> {
    let mut out = HashMap::new();
    for n in 0..hint.len() {
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

fn find_options(hint: &Vec<i32>, code:[Option<char>; 26], words: &Vec<String>) -> Vec<String> {
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

        let matches_pattern = check_word(hint, code, word);
        if !matches_pattern {
            continue 'outer
        }
        // this might be it!
        options.push(word.clone())
    }
    options
}

fn check_word(hint: &Vec<i32>, code:[Option<char>; 26], word: &String) -> bool {
    assert_eq!(hint.len() , word.len(), "should be the same length");

    let mut code = code.clone();

    for (i, c) in hint.iter().enumerate() {
        let current_word_char = word.chars().nth(i).unwrap();
        let id = (c-1)as usize;
        let matches_code = code[id];
        match matches_code {
            Some(code_value) => {
                // if the current value exists as a code but doesn't match the current word
                if current_word_char != code_value {
                    return false
                }

            },
            None => {
                code[id] = Some(current_word_char);
            }
        }
    }
    return true
}

mod tests {
    use crate::{find_options, print_hint};
    use std::fs::read_to_string;

    #[test]
    fn test_hint_print() {
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
            let mut thisline = line.to_string();
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
}