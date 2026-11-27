use antex::{ColorMode, StyledText, Text};
use std::process::exit;

fn main() {
    let cm = ColorMode::default();
    let code = match task_commander::run() {
        Ok(code) => code,
        Err(err) => {
            let error_label = Text::new(cm).red().bold().s("error:").r(' ');
            eprintln!("{error_label}{}", err.as_text(cm));
            1
        }
    };
    exit(code);
}
