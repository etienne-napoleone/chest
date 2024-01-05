use std::{io::Write, process::exit};

use console::{Style, Term};
use once_cell::sync::Lazy;

pub(crate) static DANGER: Lazy<Style> = Lazy::new(|| Style::new().red());
pub(crate) static INFO: Lazy<Style> = Lazy::new(|| Style::new().blue());
pub(crate) static SUCCESS: Lazy<Style> = Lazy::new(|| Style::new().green());

pub(crate) fn fatal(msg: &str, code: i32) -> ! {
    let prefix = DANGER.apply_to("!");
    let out = Term::stdout();
    _ = out.write_line(&format!("{prefix} {msg}"));
    exit(code);
}

pub(crate) fn info(msg: &str) {
    let prefix = INFO.apply_to(">");
    let out = Term::stdout();
    _ = out.write_line(&format!("{prefix} {msg}"));
}

pub(crate) fn success(msg: &str) {
    let prefix = SUCCESS.apply_to(">");
    let out = Term::stdout();
    _ = out.write_line(&format!("{prefix} {msg}"));
}

pub(crate) fn prompt(msg: &str) -> String {
    let prefix = INFO.apply_to("?");
    let mut out = Term::stdout();
    _ = out.write(format!("{prefix} {msg}: ").as_bytes());
    flush(&out);
    let input = out
        .read_secure_line()
        .unwrap_or_default()
        .trim()
        .to_string();
    remove_last_lines(1);
    let prefix = SUCCESS.apply_to("?");
    _ = out.write_line(&format!("{prefix} {msg}: "));
    input
}

pub(crate) fn remove_last_lines(number: usize) {
    let out = Term::stdout();
    _ = out.clear_last_lines(number);
}

fn flush(term: &Term) {
    _ = term.flush();
}
