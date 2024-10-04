// format!: write formatted text to String
// print!: same as format! but the text is printed to the console (io::stdout).
// println!: same as print! but a newline is appended.
// eprint!: same as print! but the text is printed to the standard error (io::stderr).
// eprintln!: same as eprint! but a newline is appended.

fn main() {
    println!("{subject} {verb} {object}",
    subject="sphynx of black quartz",
    verb="judge",
    object="my vow");
}