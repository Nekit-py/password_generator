use clap::{App, Arg, ArgMatches};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::array;
use std::error::Error;
use std::process::Command;


// fn set_comandline_args<'a>() -> ArgMatches<'a>{
//    let matches = App::new("Password generator")
//        .version("0.1.0")
//        .author("Nekit S. <nekit-sns@yandex.ru>")
//        .about("Generates password for your needs.")
//        .arg(Arg::with_name("lenght")
//                 .short("l")
//                 .long("lenght")
//                 .takes_value(true)
//                 .help("Takes password lenght."))
//        .arg(Arg::with_name("Password difficulty.")
//                 .short("d")
//                 .long("difficulty")
//                 .takes_value(true)
//                 .help("Takes password difficulty (optional)."))
//        .get_matches();
//    matches
// }


const SYMBOLS: [char; 19] = [
    '!', '@', '#', '$', '%', '?', '*', '-', '_', '=', '+', ':', ';', '.', ',', '(', ')', '|', '/',
];

const NUMBERS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];


const LETTERS_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];


const LETTERS_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];


struct PasswordProperties {
    len: u8,
    difficulty: Vec<Vec<char>>,
}

impl Default for PasswordProperties {
    fn default() -> Self {
        PasswordProperties {
            len: 8,
            difficulty: vec![LETTERS_LOWER.to_vec(), LETTERS_UPPER.to_vec(), NUMBERS.to_vec()],
        }
    }
}

impl PasswordProperties {
    fn new(len: Option<u8>, difficulty: Option<Vec<Vec<char>>>) -> PasswordProperties {
        PasswordProperties  {
            len: len.unwrap_or(8),
            difficulty: difficulty.unwrap_or(vec![LETTERS_LOWER.to_vec(), LETTERS_UPPER.to_vec(), NUMBERS.to_vec()]),
        }
    }
}


struct Password {
    password: String,
}

impl Default for Password {
    fn default() -> Self {
        let properties = PasswordProperties::default();
        let mut pwd = String::new();
        let mut pwd_chars = Vec::with_capacity(properties.len as usize);

        for chars in properties.difficulty.iter() {
            pwd.push(*chars.choose(&mut thread_rng()).unwrap());
            pwd_chars = chars.into_iter().chain(chars.into_iter()).collect()
        }

        for _ in properties.difficulty.len() as u8..properties.len {
            pwd.push(**pwd_chars.choose(&mut thread_rng()).unwrap());
        }

        Password {
            password: pwd
        }
    }
}

impl Password {
    fn gen_pwd(properties: PasswordProperties) -> Password {
        let properties = properties;
        let mut pwd = String::new();
        let mut pwd_chars = Vec::new();

        for chars in properties.difficulty.iter() {
            pwd.push(*chars.choose(&mut thread_rng()).unwrap());
            pwd_chars = chars.into_iter().chain(chars.into_iter()).collect()
        }

        for _ in properties.difficulty.len() as u8..properties.len {
            pwd.push(**pwd_chars.choose(&mut thread_rng()).unwrap());
        }

        Password {
            password: pwd
        }
    }
}


fn main() {
    let pwd_default = Password::default();
    let pwd_easy = {
        let properties = PasswordProperties::new(
            Some(12),
            Some(vec![NUMBERS.to_vec(), LETTERS_LOWER.to_vec()])
        );
        Password::gen_pwd(properties)
    };

    let pwd_hard = {
        let properties = PasswordProperties::new(
            Some(12),
            Some(vec![LETTERS_LOWER.to_vec(), LETTERS_UPPER.to_vec(), NUMBERS.to_vec(), SYMBOLS.to_vec()])
        );
        Password::gen_pwd(properties)
    };

    println!("{:?}", pwd_easy.password);
    println!("{:?}", pwd_default.password);
    println!("{:?}", pwd_hard.password);
}
