use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Right,
    Left,
    Inc,
    Dec,
    OutRaw,
    OutChar,
    In,
    StartLoop,
    EndLoop,
    StartThread(String),
    EndThread,
    CheckThread(String),
    EndCheckThread,
}

#[derive(Debug, Clone)]
pub struct Threads {
    pub name: String,
    pub tokens: Vec<Tokens>,
}

impl Threads {
    pub fn name(&mut self) -> Self {
        self.name = match self.tokens[0].clone() {
            Tokens::StartThread(name) => name,
            _ => panic!("Thread name not found"),
        };
        self.tokens.remove(0);
        return Self {
            name: self.name.clone(),
            tokens: self.tokens.clone(),
        };
    }
}

pub fn tokenize(input: String) -> Vec<Threads> {
    // Regexes for StartThread and EndThread
    lazy_static! {
        static ref THREAD: Regex = Regex::new(r#"[{, \(][a-z, A-Z, [0-9]*]*\|"#).unwrap();
    }

    let mut threads = vec![];

    let mut tokens = vec![];
    let mut current_checked: String = String::new();
    for x in input.chars() {
        match x {
            '>' => tokens.push(Tokens::Right),
            '<' => tokens.push(Tokens::Left),
            '+' => tokens.push(Tokens::Inc),
            '-' => tokens.push(Tokens::Dec),
            ':' => tokens.push(Tokens::OutRaw),
            '.' => tokens.push(Tokens::OutChar),
            ',' => tokens.push(Tokens::In),
            '[' => tokens.push(Tokens::StartLoop),
            ']' => tokens.push(Tokens::EndLoop),
            '|' => {
                current_checked.push('|');
                let regex_match = &THREAD.captures_iter(current_checked.as_str()).next().unwrap()[0];
                let first_char = regex_match.chars().next().unwrap();
                let name = regex_match.chars().skip(1).take_while(|&x| x != '|').collect::<String>();
                if first_char == '{' {
                    tokens.push(Tokens::StartThread(name));
                } else {
                    tokens.push(Tokens::CheckThread(name));
                }
                current_checked = String::new();
            },
            '}' => {
                threads.push(Threads {
                    name: String::new(),
                    tokens: tokens.clone(),
                }.name());
                tokens = vec![];
            },
            ')' => tokens.push(Tokens::EndCheckThread),
            _ => current_checked.push(x),
        }
    }
    // println!("{:#?}", threads);
    return threads;
}