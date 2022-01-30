use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{thread, cell};

use crate::lexer::{Threads, Tokens};


pub fn parse(input: Vec<Threads>) {
    let cells_arc = Arc::new(Mutex::new(vec![0]));
    let threads_status_arc = Arc::new(Mutex::new(HashMap::new()));
    for x in input.clone() {
        (*threads_status_arc.lock().unwrap()).insert(x.name.clone(), true);
    }
    for threads in input {
        let cells = cells_arc.clone();
        let threads_status = threads_status_arc.clone();
        let thread_join_handle = thread::spawn(move || {
            let mut loop_start = vec![0];
            let mut loop_deep = 0;
            let mut x = 0;
            let mut current_cell = 0isize;
            while x != threads.tokens.len() {
                match threads.tokens[x].clone() {
                    Tokens::Right => {
                        current_cell += 1;
                        let mut locked = cells.lock().unwrap();
                        if current_cell > locked.len() as isize - 1 {
                            (*locked).push(0);
                        } 
                    },
                    Tokens::Left => {
                        if current_cell - 1 != -1 {
                            current_cell -= 1;
                        } else {
                            (*cells.lock().unwrap()).push(0);
                        }
                    },
                    Tokens::Inc => {
                        (*cells.lock().unwrap())[current_cell as usize] += 1
                    },
                    Tokens::Dec => {
                        (*cells.lock().unwrap())[current_cell as usize] -= 1;
                    },
                    Tokens::OutRaw => {
                        println!("{}", (*cells.lock().unwrap())[current_cell as usize]);
                    },
                    Tokens::OutChar => {
                        print!("{}", (*cells.lock().unwrap())[current_cell as usize] as u8 as char);
                    },
                    Tokens::In => {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).expect("Failed to read line");
                        (*cells.lock().unwrap()).push(input.trim().parse::<isize>().unwrap());
                    },
                    Tokens::StartLoop => {
                        loop_start.push(x);
                        loop_deep += 1;
                    },
                    Tokens::EndLoop => {
                        if (*cells.lock().unwrap())[current_cell as usize] != 0 {
                            x = loop_start[loop_deep];
                        } else {
                            loop_deep -= 1;
                        }
                    },
                    Tokens::CheckThread(name) => {
                        while (*threads_status.lock().unwrap()).get(&name).unwrap() == &true {
                            thread::sleep(std::time::Duration::from_millis(100));
                        }
                    }
                    _ => ()
                }
                x += 1;
            }
            // println!("Thread '{}' finished: {:?}", threads.name, cells.lock().unwrap());
            (*threads_status.lock().unwrap()).insert(threads.name.clone(), false);
        });
        let res = thread_join_handle.join().unwrap();
    }
    // println!("End: {:?}", cells_arc.clone().lock().unwrap());
}