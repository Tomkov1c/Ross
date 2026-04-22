use std::env;

use crate::handlers::output_handler;

pub fn main(listen: bool) {
    let (quote, link) = match rand::random_range(0..10) {
        0 => ( "Wiggle... wiggle wiggle", "https://youtu.be/VlucWfTUo1A?si=pW9MXse3NcBZzCjA&t=1311" ),
        1 => ( "Now then, let's give him... let's give him a little friend", "https://youtu.be/vbPdQ0w8ylg?si=1kZR9lvEN4U13s9l&t=647"),
        _ => ("Shake off the excess. *laughs* And just beat the devil out of it. This is where you take out all of your frustrations and hostilities and just have a good time.", "https://youtu.be/VlucWfTUo1A?si=DGVnKjoCq-DUSweD&t=256"
        ),
    };

    if listen {
        if let Err(e) = open::that(link) {
            output_handler::error(&format!("Failed to open link: {}", e));
        }
    }else {
        print_to_terminal(quote, link);
    }


}

fn print_to_terminal(quote: &str, link: &str) {
    let supports_links = env::var("TERM_PROGRAM").unwrap_or_default() != "Apple_Terminal";

    output_handler::new_line();
    output_handler::normal(&format!("\"{}\"", quote));

    if supports_links {
        output_handler::normal(&format!("  -- Bob Ross \x1b]8;;{}\x1b\\[Click to listen]\x1b]8;;\x1b\\", link));
    } else {
        output_handler::normal(&format!("  -- Bob Ross [Listen here: {}]", link));
    }
    output_handler::new_line();
}