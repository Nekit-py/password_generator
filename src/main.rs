use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

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
            difficulty: vec![
                LETTERS_LOWER.to_vec(),
                LETTERS_UPPER.to_vec(),
                NUMBERS.to_vec(),
            ],
        }
    }
}

impl PasswordProperties {
    fn new(len: Option<u8>, difficulty: Option<Vec<Vec<char>>>) -> PasswordProperties {
        PasswordProperties {
            len: len.unwrap_or(8),
            difficulty: difficulty.unwrap_or(vec![
                LETTERS_LOWER.to_vec(),
                LETTERS_UPPER.to_vec(),
                NUMBERS.to_vec(),
            ]),
        }
    }
}

trait Show {
    fn show_password(&self);
}

struct Password {
    password: String,
}

impl Default for Password {
    fn default() -> Self {
        let properties = PasswordProperties::default();
        let mut prefix = String::new();
        let mut pwd_chars = Vec::with_capacity(properties.len as usize);

        for vec in properties.difficulty.iter() {
            prefix.push(*vec.choose(&mut thread_rng()).unwrap());
            for char in vec.iter() {
                pwd_chars.push(char.clone())
            }
        }
        let mut pwd = pwd_chars
            .choose_multiple(
                &mut rand::thread_rng(),
                properties.len as usize - properties.difficulty.len() as usize,
            )
            .into_iter()
            .collect::<String>();

        pwd = [prefix, pwd].join("");

        Password { password: pwd }
    }
}

impl Show for Password {
    fn show_password(&self) {
        println!("New password is -> {}", self.password);
    }
}

impl Password {
    fn gen_pwd(properties: PasswordProperties) -> Password {
        let mut prefix = String::new();
        let mut pwd_chars = Vec::with_capacity(properties.len as usize);

        for vec in properties.difficulty.iter() {
            prefix.push(*vec.choose(&mut thread_rng()).unwrap());
            for char in vec.iter() {
                pwd_chars.push(char.clone())
            }
        }
        let mut pwd = pwd_chars
            .choose_multiple(
                &mut rand::thread_rng(),
                properties.len as usize - properties.difficulty.len() as usize,
            )
            .into_iter()
            .collect::<String>();

        pwd = [prefix, pwd].join("");

        Password { password: pwd }
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input
}

fn main() {
    println!("Please set the password length (max length is 255):");
    let length = {
        let input_string = get_user_input();
        input_string.trim().parse::<u8>().unwrap_or(8)
    };

    println!("Please set the password difficulty:");
    let difficulty = {
        match get_user_input().as_str().trim() {
            "easy" | "simple" | "low" | "short" => {
                Some(vec![NUMBERS.to_vec(), LETTERS_LOWER.to_vec()])
            }
            "medium" | "default" | "middle" | "standart" => None,
            "hard" | "strong" | "expert" | "big" => Some(vec![
                LETTERS_LOWER.to_vec(),
                LETTERS_UPPER.to_vec(),
                NUMBERS.to_vec(),
                SYMBOLS.to_vec(),
            ]),
            _ => None,
        }
    };

    let properties = PasswordProperties::new(Some(length), difficulty);
    let password = Password::gen_pwd(properties);
    password.show_password();
}
