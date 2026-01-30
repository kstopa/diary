use std::fs::File;
use std::io::Write;

const MAX_LINE_LENGHT: usize = 80;

pub fn write(
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
