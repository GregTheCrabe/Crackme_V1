use colored::Colorize;

pub fn apropos() {
    println!(
        "{}",
        r"       INFORMATION ".bright_cyan().bold().underline()
    );
    println!("{}", r"- Nom         ----> Crackme_V1".bright_cyan());
    println!("{}", r"- Niveau      ----> Débutant".bright_cyan().bold());
    println!("{}", r"- Langage     ----> RUST".bright_cyan().bold());
    println!("{}", r"- Sécurité    ----> ❌".bright_cyan().bold());
    println!("{}", r"- Developpeur -----> Gr€g".bright_cyan());
    println!("");
}
