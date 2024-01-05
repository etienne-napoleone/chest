use std::{io::Write, process::exit};

use console::{Style, Term};
use once_cell::sync::Lazy;

static DANGER: Lazy<Style> = Lazy::new(|| Style::new().red());
static INFO: Lazy<Style> = Lazy::new(|| Style::new().blue());
// static HIGHLIGHT: Lazy<Style> = Lazy::new(|| Style::new().yellow());

pub(crate) fn fatal(msg: &str, code: i32) -> ! {
    let prefix = DANGER.apply_to("!");
    println!("{prefix} {msg}");
    exit(code)
}

pub(crate) fn prompt(msg: &str) -> String {
    let prefix = INFO.apply_to("?");
    let mut out = Term::stdout();
    _ = out.write(format!("{prefix} {msg}: ").as_bytes());
    _ = out.flush();
    out.read_secure_line()
        .unwrap_or_default()
        .trim()
        .to_string()
}
