use indicatif::MultiProgress;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::sync::OnceLock;
use std::time::Duration;

// Progress bar

static MULTI: OnceLock<MultiProgress> = OnceLock::new();
static BAR: OnceLock<ProgressBar> = OnceLock::new();
static STATUS: OnceLock<ProgressBar> = OnceLock::new();

fn bar() -> &'static ProgressBar {
    BAR.get().expect("not initialized — call init() first")
}

fn status() -> &'static ProgressBar {
    STATUS.get().expect("not initialized — call init() first")
}

pub fn bar_start(total: u64, label: &str) {
    let multi = MultiProgress::new();

    let pb = multi.add(ProgressBar::new(total));
    pb.set_style(
        ProgressStyle::default_bar().tick_strings(&["|", "/", "–", "\\", "✔"])
            .template(&format!("{{spinner:.white}} {label} {{bar:40.green/white}} {{pos}}/{{len}} ({{eta}})"))
            .unwrap().progress_chars("=>·")
    );
    pb.enable_steady_tick(Duration::from_millis(150));

    let status = multi.add(ProgressBar::new(0));
    status.set_style(ProgressStyle::default_bar().template("{msg}").unwrap());

    MULTI.set(multi).expect("already initialized");
    BAR.set(pb).expect("already initialized");
    STATUS.set(status).expect("already initialized");
}

pub fn increase_position(n: u64) { bar().inc(n); }

pub fn set_position(pos: u64) { bar().set_position(pos); }

pub fn print_above(msg: &str) { bar().println(msg); }

pub fn print_below(msg: &str) { status().set_message(msg.to_string()); }

pub fn print_above_offset(msg: &str) { print_above(&format!("  {msg}")); }

pub fn print_below_offset(msg: &str) { print_below(&format!("  {msg}")); }

pub fn bar_finish(msg: &str) {
    bar().finish();
    status().finish_with_message(msg.to_string());
}

pub fn finish_offset(msg: &str) { bar_finish(&format!("  {msg}")); }

// Other output

pub fn error(msg: &str) { eprintln!("\x1b[1;31m{msg}\x1b[0m"); }

pub fn warn(msg: &str) { eprintln!("\x1b[1;33m{msg}\x1b[0m"); }

pub fn info(msg: &str) { eprintln!("\x1b[1;36m{msg}\x1b[0m"); }

pub fn normal(msg: &str) { println!("{}", msg); }


#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        eprintln!("\x1b[1;35m{}\x1b[0m", format!($($arg)*));
    }};
}