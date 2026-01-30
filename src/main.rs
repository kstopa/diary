use chrono::{Duration, Local};
use std::io::{self, Write};
mod diary;

use diary::entry;

fn ask_question(question: &str) -> String {
    // Promt a question for the user and returns the given answer trimed.
    let mut answer = String::new();
    print!("{question} ");
    io::stdout().flush().unwrap(); // Make sure prompt shows before input
    io::stdin().read_line(&mut answer).unwrap();
    answer.trim().to_string()
}

fn main() {
    // TODO enable to set the date from args
    // TODO write formatted.
    //
    let yesterday_date = Local::now() - Duration::days(1);
    let at_date = yesterday_date.format("%Y-%m-%d");
    println!("Welcome to the diary on {at_date}");
    println!("---------------------------------");

    //let lorem = String::from(
    //    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequ. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariat. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
    //);
    //let lines = format_line(lorem);
    //println!("{}", lines);
    //return;

    let answer_how_day = ask_question("How was your day? (1-5):");

    let answer_remarkable = ask_question("What was the most remarkable:");

    let answer_best = ask_question("What was the best:");

    let answer_worst = ask_question("What was the worst:");

    let answer_who_new = ask_question("Who you've met?");

    let answer_learn = ask_question("What new you've leanred?");

    let answer_breif = ask_question("Day briefing");

    let answer_dream = ask_question("What you dreamt about?");

    let resp = entry::write(
        at_date.to_string(),
        answer_how_day,
        answer_remarkable,
        answer_best,
        answer_worst,
        answer_who_new,
        answer_learn,
        answer_dream,
        answer_breif,
    );
    match resp {
        Ok(_) => {
            println!("Writing new entry at data/{}.md", at_date);
            io::stdout().flush().unwrap();
        }
        Err(e) => {
            println!("Error {} writing a new entry at data/{}.md", e, at_date);
        }
    }
}
