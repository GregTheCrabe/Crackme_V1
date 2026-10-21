use colored::Colorize;
use std::io;
use std::io::Write;
pub fn mot_de_pass() {
    //-----------------------<création du mot de pass --------------------------
    let mot_de_pass_secret = "Rustacean-2026";
    //--------------------------------------------------------------------------

    println!(
        "{}",
        r"-------------BIENVENUE DANS CRACKME_V1 🔒-------------"
            .bright_cyan()
            .underline()
            .bold()
    );
    println!(
        "{}",
        r".  ENTRER LE MOT DE PASS POUR CRACKER LE PROGRAMME ".bright_cyan()
    );
    println!("");
    loop {
        print!("{}", r"Mot de pass ici ---> ".bright_yellow());
        io::stdout().flush().expect("Impossible de vider le tampon");

        let mut tentative = String::new();
        io::stdin()
            .read_line(&mut tentative)
            .expect("Echec de la lecture {}");

        if tentative.trim() == mot_de_pass_secret {
            println!("");
            println!("{}", r"********************************".bright_green());
            println!("{}", r"👍 ACCES AUTORISER ✅ : BRAVO ".bright_green());
            println!("{}", r"********************************".bright_green());

            break;
        } else {
            println!("");
            println!("{}", r"*********************".bright_red());
            println!("{}", r"⛔️ ACCES REFUSER ⛔️ ".bright_red());
            println!("{}", r"*********************".bright_red());
            println!("");
            println!("{}", r"Voulez-vous réessayer ? o/n :".bright_yellow());
            io::stdout().flush().expect("Impossible de vider le tampon");

            //------------------Oui / non pour sortir du crackme---------------------
            let mut reponse = String::new();
            io::stdin()
                .read_line(&mut reponse)
                .expect("Echec de la lecture");

            //---------------------vérification de la réponse-------------------------
            if reponse.trim().to_lowercase() == "n" {
                println!("{}", r"Retour au menu principal .... ".bright_magenta());
                break; // n = on brise la boucle
            }
        }
    }
}
