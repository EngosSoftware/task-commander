use antex::{ColorMode, StyledText, Text};
use task_commander::run;

fn main() {
    let cm = ColorMode::default();
    match run() {
        Ok(()) => {}
        Err(err) => {
            eprintln!(
                "{}: {}",
                Text::new(cm).red().bold().s("error").reset(),
                err.as_text(cm)
            );
            std::process::exit(1);
        }
    }
}
