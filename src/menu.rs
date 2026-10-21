use colored::Colorize;

pub fn afficher_menu() {
    println!("{}", r".................................".bright_green());
    println!(
        "{}",
        r"             Crackme_V1          ".bright_green().bold()
    );
    println!("{}", r".................................".bright_green());
    println!("{}", r"           \                     ".bright_green());
    println!("{}", r"            \                    ".bright_green());
    println!("{}", r"             _~^~^~_¨.           ".bright_blue());
    println!("{}", r"         \) /  o o  \ (/         ");
    println!("{}", r"           '_   -   _'           ");
    println!("{}", r"           / '-----' \           ".bright_red());
    println!("{}", r"                |                ".bright_green());
    println!("{}", r"                |                ".bright_green());
    println!("{}", r".................................".bright_green());
    println!("{}", r"              Menu               ".bright_yellow());
    println!("{}", r"         1°_Crackme_V1           ".bright_yellow());
    println!("{}", r"         2°_A propos.            ".bright_yellow());
    println!("{}", r"         0°_Quitter.             ".bright_yellow());
    println!("{}", r".................................".bright_green());
    println!("");
}
