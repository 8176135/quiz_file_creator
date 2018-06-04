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
    println!("    'edit <x> <new text>' to edit values");
    println!("    'next'/'prev' to go to next or previous questions");
    println!("    'add'/'del' to add question, or delete current question");
    println!("    'list' to list all questions");
    println!("    'go <x>' to go to specific question number");
    println!("'save' to save, Ctrl-C to save and quit, Ctrl-D discard and quit. (Probably want to save often, not sure how stable this is)");
}

fn print_list(question_num: usize, data: &Vec<Vec<String>>, status: &str, prev_cmd: &str) {
    println!("All questions:");
    println!();
    println!("    Num -- Question");
    for (index, datum) in data.iter().enumerate() {
        if index == question_num {
            print!(" --> ");
        } else {
            print!("     ");
        }
        println!("{}:   {}", index, datum[0]);
    }

    println!();
    println!("Last Command: {}, Status: {}", prev_cmd, status);
    println!();
    println!("Commands:");
    println!("    'edit <x> <new text>' to edit values");
    println!("    'next'/'prev' to go to next or previous questions");
    println!("    'add'/'del' to add question, or delete current question");
    println!("    'list' to toggle list mode");
    println!("    'go <x>' to go to specific question number");
    println!("'save' to save, Ctrl-C to save and quit, Ctrl-D discard and quit. (Probably want to save often, not sure how stable this is)");
}

fn save_file<P: AsRef<std::path::Path>>(file_path: P, data: &Vec<Vec<String>>) {
    fs::copy(&file_path, file_path.as_ref().to_str().unwrap().to_owned() + ".bak");//.expect("File backup failed");

    fs::write(&file_path, &data.iter()
        .flat_map(|c| c.iter()
            .flat_map(|v| (v.clone() + "\n").into_bytes()))
        .collect::<Vec<u8>>())
        .expect("Save failed");
}

fn main() {
    let quiz_data_file_path = std::env::args().nth(1).unwrap_or("QuizData.dat".to_owned());

    let data = fs::read_to_string(&quiz_data_file_path).unwrap_or("1+1\n5\n9\n1337\n2".to_owned())
        .lines().map(|c| c.to_owned()).collect::<Vec<String>>();
    let mut data = data.chunks(5).map(|c| c.to_owned()).collect::<Vec<_>>();

    let mut rl = Editor::<()>::new();
    //let stdin = io::stdin();

    let mut current_question_number: i32 = 0;
    let mut mode = 0;
    let mut status = String::new();
    let mut input = String::new();

    let cmd_match = regex::Regex::new(r#"(?:(next))|(?:(prev))|(?:(go) (\d+))|(?:(edit) ([0-5]) (.+)|(?:(add))|(?:(del))|(?:(list))|(?:(save)))"#).unwrap();

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
            1 => print_list(current_question_number as usize, &data, &status, input.trim()),
            _ => ()
        }
        status.clear();
        input.clear();
        input = match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(&line);
                line.trim().to_owned()
            }
            Err(ReadlineError::Interrupted) => {
                println!("Saving...");
                save_file(&quiz_data_file_path, &data);
                println!("Saved");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Changes discarded");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
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
        //input = input.to_lowercase();

        let captures = cmd_match.captures(&input).map(|captures| {
            captures
                .iter() // All the captured groups
                .skip(1) // Skipping the complete match
                .flat_map(|c| c) // Ignoring all empty optional matches
                .map(|c| c.as_str()) // Grab the original strings
                .collect::<Vec<_>>() // Create a vector
        });

        match captures.as_ref().map(|c| c.as_slice()) {
            Some(["next"]) => current_question_number += 1,
            Some(["prev"]) => current_question_number -= 1,
            Some(["add"]) => {
                data.push(vec!["1+1".to_owned(), "1".to_owned(), "4".to_owned(), "9000".to_owned(), "2".to_owned()]);
                current_question_number = data.len() as i32 - 1;
                status = "Added new question".to_owned();
            }
            Some(["del"]) => {
                if data.len() == 1 {
                    status = format!("Can't remove last question");
                } else {
                    data.remove(current_question_number as usize);
                    status = format!("Removed question number {}", current_question_number);
                }
            }
            Some(["list"]) => {
                mode = if mode == 1 { 0 } else { 1 };
            },
            Some(["save"]) => {
                save_file(&quiz_data_file_path, &data);
                status = "Saved".to_owned();
            }
            Some(["go", x]) => {
                let x = x.parse().unwrap();
                current_question_number = x;
            }
            Some(["edit", x, text]) => {
                let x: usize = x.parse::<usize>().unwrap();
                data[current_question_number as usize][x] = text.to_owned().to_owned();
            }
            _ => status = "Unknown command".to_owned(),
        }
    }
}
