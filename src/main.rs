use chrono::{Duration, Local};
use std::fs::File;
use std::io::{self, Write};

const MAX_LINE_LENGHT: usize = 80;

fn ask_question(question: &str) -> String {
    // Promt a question for the user and returns the given answer trimed.
    let mut answer = String::new();
    print!("{question} ");
    io::stdout().flush().unwrap(); // Make sure prompt shows before input
    io::stdin().read_line(&mut answer).unwrap();
    answer.trim().to_string()
}

fn write_entry(
    // TODO Create struct
    // TODO i18n and translations
    at_date: String,
    a_how_day: String,
    a_remarkable: String,
    a_best: String,
    a_worst: String,
    a_new: String,
    a_learn: String,
    a_dream: String,
    a_breifing: String,
) -> std::io::Result<()> {
    let mut file = File::create(format!("./data/{at_date}.md"))?;

    writeln!(file, "# {}\n", at_date)?;

    writeln!(file, "## Stats\n")?;
    writeln!(file, "* **Nota**: {}/5\n\n", a_how_day)?;

    writeln!(file, "## Q&A\n")?;
    writeln!(file, "* **Lo más destacado**: {}", a_remarkable)?;
    writeln!(file, "* **Lo mejor**: {}", a_best)?;
    writeln!(file, "* **Lo peor**: {}", a_worst)?;
    writeln!(file, "* **He conocidos**: {}", a_new)?;
    writeln!(file, "* **He aprendido**: {}\n", a_learn)?;
    writeln!(file, "* **He soñado**: {}\n", a_dream)?;
    // TODO: format text to max 80 chars per line
    writeln!(file, "## Resumen\n")?;
    writeln!(file, "{}", a_breifing)?;

    Ok(())
}

fn format_line(line: String) -> String {
    // divide the line to the maximium number of available.
    let mut lines = String::new();
    let mut current_line = String::new();
    const SPACE: &str = " ";
    const EOL: &str = "\n";
    for word in line.split_whitespace() {
        if current_line.chars().count() + word.chars().count() + 1 > MAX_LINE_LENGHT {
            lines += &current_line;
            lines += EOL;
            current_line.clear();
        }
        current_line.push_str(SPACE);
        current_line.push_str(word);
    }
    // Last line
    lines += &current_line;
    lines
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

    let resp = write_entry(
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
