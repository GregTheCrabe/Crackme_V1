use colored::Colorize;
use std::io;
use std::io::Write;
mod apropos;
mod menu;
mod password;

fn main() {
    loop {
        menu::afficher_menu();
        print!("{}", r"Votre choix ---> ".bright_yellow());
        io::stdout().flush().expect("Impossible de vider le tampon"); // pause du terminal
                                                                      //---------------------------Gestion de la saisie de l'utilisateur pour le menu ------------------------
        let mut saisie_choix = String::new();
        io::stdin()
            .read_line(&mut saisie_choix)
            .expect("Echec de la lecture ");
        //------------------------------------------------------------------------------------------------------

        //---------------------------On convertit cette chaine en nombre u32------------------------------------
        let choix: u32 = match saisie_choix.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("");
                println!("{}", r"Veuillez entrer un nombre valide! ".bright_red());
                continue;
            }
        };
        //------------------------------------------------------------------------------------------------------

        //--------------------------Programme Principal---------------------------------------------------------
        match choix {
            0 => {
                println!(
                    "{}",
                    "Vous avez choisi Quitter! Au revoir.".bright_magenta()
                );
                break;
            }
            1 => {
                password::mot_de_pass();
            }
            2 => {
                apropos::apropos();
                println!(
                    "\n{}",
                    "---> Appuyez sur entrée pour continuer! ".bright_green()
                );
                let mut pause = String::new();
                io::stdin()
                    .read_line(&mut pause)
                    .expect("echec de la lecture");
            }
            _ => {
                println!(
                    "{}",
                    r"Option inconnue !, Veuillez choisir en 0 et 2 ".bright_red()
                );
                println!(
                    "\n{}",
                    "-> Appuyez sur Entrée pour continuer...".bright_green()
                );
                let mut pause_inconnu = String::new();
                io::stdin().read_line(&mut pause_inconnu).unwrap();
            }
        }
    }
}
