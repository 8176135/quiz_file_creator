extern crate rustyline;
extern crate regex;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use regex::Regex;

use std::fs;
use std::io;

fn print_default_screen(question_num: usize, data: &Vec<Vec<String>>, status: &str, prev_cmd: &str) {
    println!("Current Question Number: {}", question_num);
    println!();
    println!("  0 -- Question: {}", data[question_num][0]);
    println!();
    println!("Possible answers: ");
    println!("    1 -- Wrong: {}", data[question_num][1]);
    println!("    2 -- Wrong: {}", data[question_num][2]);
    println!("    3 -- Wrong: {}", data[question_num][3]);
    println!("    4 -- Correct: {}", data[question_num][4]);

    println!();
    println!("Last Command: {}, Status: {}", prev_cmd, status);
    println!();

    println!("Commands:");
    println!("    'Edit <x>' to edit values");
    println!("    'Next'/'Prev' to go to next or previous questions");
    println!("    'List' to list all questions");
    println!("    'Go <x>' to go to specific question number");
}

fn save_file(data: &Vec<Vec<String>>) {

}

fn main() {
    let quiz_data_file_path = std::env::args().nth(1).unwrap_or("QuizData.dat".to_owned());
    let data = fs::read_to_string(quiz_data_file_path).expect("Incorrect file path")
        .lines().map(|c| c.to_owned()).collect::<Vec<String>>();
    let data = data.chunks(5).map(|c| c.to_owned()).collect::<Vec<_>>();

    let mut rl = Editor::<()>::new();
    //let stdin = io::stdin();

    let mut current_question_number: i32 = 0;
    let mut mode = 0;
    let mut status = String::new();
    let mut input = String::new();

    let cmd_match = regex::Regex::new(r"([a-zA-Z]+) (\d+)?").unwrap();

    loop {
        print!("{}[2J", 27 as char); // Clear terminal
        if current_question_number >= data.len() as i32 {
            status = "Question number outside of range".to_owned();
            current_question_number = (data.len() - 1) as i32;
        } else if current_question_number < 0 {
            status = "Question number outside of range".to_owned();
            current_question_number = 0;
        }

        match mode {
            0 => print_default_screen(current_question_number as usize, &data, &status, input.trim()),
            _ => ()
        }
        status.clear();
        input.clear();
        input = match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(&line);
                line.trim().to_owned()
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("EOF");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        };

//        if let Some(captures) = cmd_match.captures(input.as_ref()) {
//            let cmd = captures.get(1).unwrap().as_str().to_lowercase();
//            if let Some(param) = captures.get(2) {
//                param.as_str().parse().unwrap()
//            } else {
//                match cmd.as_ref() {
//                    "next" => current_question_number += 1,
//                    "prev" => current_question_number -= 1,
//                }
//            }
//        } else {
//            status = "Unknown Command".to_owned();
//        }
        input = input.to_lowercase();
        let mut tokens = input.split(' ');
        let cmd = tokens.next();
        let param = tokens.next();

        match cmd {
            Some("next") => current_question_number += 1,
            Some("prev") => current_question_number -= 1,
            Some("go") => if let Some(param) = param {
                if let Ok(param) = param.parse() {
                    current_question_number = param;
                } else {
                    status = "Go with invalid parameter".to_owned();
                }
            } else {
                status = "Go with no parameter".to_owned();
            }
            _ => status = "Unknown Command".to_owned()
        }
    }
}
