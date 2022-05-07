use clap::{App, Arg, ArgMatches};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::array;
use std::error::Error;
use std::process::Command;


// fn set_comandline_args<'a>() -> ArgMatches<'a>{
//    let matches = App::new("PLE generator")
//        .version("0.1.0")
//        .author("Nekit S. <nekit-sns@yandex.ru>")
//        .about("Generates for your needs: login, password, pin code and email.")
//        .arg(Arg::with_name("password")
//                 // .short("p")
//                 .long("password")
//                 .takes_value(true)
//                 .help("Generates password. Takes password lenght."))
//        .arg(Arg::with_name("Password difficulty.")
//                 .short("d")
//                 .long("difficulty")
//                 .takes_value(true)
//                 .help("Takes password difficulty (optional)."))
//        .arg(Arg::with_name("Password lenght.")
//                 .short("l")
//                 .long("lenght")
//                 .takes_value(true)
//                 .help("Takes password lenght (optional)."))
//        .arg(Arg::with_name("PIN code")
//                 .short("pin")
//                 .long("pin_code")
//                 .takes_value(true)
//                 .help("Generates pin code. Takes pin code lenght and difficulty"))
//        .arg(Arg::with_name("login")
//                 .short("l")
//                 .long("login")
//                 .takes_value(false)
//                 .help("Generates login."))
//        .arg(Arg::with_name("email")
//                 .short("e")
//                 .long("email")
//                 .takes_value(false)
//                 .help("Generates email."))
//        .get_matches();
//    matches
// }
//

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
    difficulty: String,
}

impl Default for PasswordProperties {
    fn default() -> Self {
        PasswordProperties {
            len: 8,
            difficulty: String::from("medium"),
        }
    }
}

impl PasswordProperties {
    fn new(len: Option<u8>, difficulty: Option<String>) -> PasswordProperties {
        PasswordProperties  {
            len: len.unwrap_or(8),
            difficulty: difficulty.unwrap_or(String::from("medium")),
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
        let mut chars = Vec::new();
        pwd.push(*LETTERS_UPPER.choose(&mut thread_rng()).unwrap());
        pwd.push(*LETTERS_LOWER.choose(&mut thread_rng()).unwrap());
        pwd.push(*NUMBERS.choose(&mut thread_rng()).unwrap());
        chars = NUMBERS.to_vec().into_iter()
            .chain(LETTERS_LOWER.to_vec().into_iter())
            .chain(LETTERS_UPPER.to_vec().into_iter())
            .collect();
        for _ in 3..properties.len {
            pwd.push(*chars.choose(&mut thread_rng()).unwrap());
        }
        Password {
            password: pwd
        }
    }
}

impl Password {
    fn hard(properties: PasswordProperties) -> Password {
        let mut pwd = String::new();
        let mut chars = Vec::new();
        pwd.push(*LETTERS_UPPER.choose(&mut thread_rng()).unwrap());
        pwd.push(*LETTERS_LOWER.choose(&mut thread_rng()).unwrap());
        pwd.push(*SYMBOLS.choose(&mut thread_rng()).unwrap());
        pwd.push(*NUMBERS.choose(&mut thread_rng()).unwrap());
        chars = NUMBERS.to_vec().into_iter()
            .chain(LETTERS_LOWER.to_vec().into_iter())
            .chain(LETTERS_UPPER.to_vec().into_iter())
            .chain(SYMBOLS.to_vec().into_iter())
            .collect();
        for _ in 4..properties.len {
            pwd.push(*chars.choose(&mut thread_rng()).unwrap());
        }
        Password {
            password: pwd,
        }
    }
}

impl Password {
    fn easy(properties: PasswordProperties) -> Password {
        let mut pwd = String::new();
        let mut chars = Vec::new();
        pwd.push(*LETTERS_UPPER.choose(&mut thread_rng()).unwrap());
        pwd.push(*LETTERS_LOWER.choose(&mut thread_rng()).unwrap());
        chars = LETTERS_UPPER.to_vec().into_iter()
            .chain(LETTERS_LOWER.to_vec().into_iter())
            .collect();
        for _ in 2..properties.len {
            pwd.push(*chars.choose(&mut thread_rng()).unwrap());
        }
        Password {
            password: pwd,
        }
    }
}

fn main() {
    let pwd_default = Password::default();
    let pwd_easy = {
        let properties = PasswordProperties::new(Some(12), Some(String::from("easy")));
        Password::easy(properties)
    } ;
    let pwd_hard = {
        let properties = PasswordProperties::new(Some(12), Some(String::from("hard")));
        Password::hard(properties)
    };
    println!("{:?}", pwd_easy.password);
    println!("{:?}", pwd_default.password);
    println!("{:?}", pwd_hard.password);
}
