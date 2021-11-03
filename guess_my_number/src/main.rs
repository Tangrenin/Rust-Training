use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use colored::*;

fn main() {
    // Génération de nombres aléatoires
    let mut rng = thread_rng();
    let nombre_secret: u32 = rng.gen_range(1..101);

    println!("{}","\n\nDevine mon nombre ! (entre 1 et 100) \n\nSaisissez votre proposition : ".purple());

    loop {
        // Saisie de l'utilisateur
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // On vérifie que l'utilisateur a rentré un nombre valide
        let input = match input.trim().parse::<u32>() {
            Ok(val) =>  val,
            Err(_) => {eprintln!("{}", "Erreur : entrez un nombre entier positif !".red()); continue}
        };

        // Comparaison avec le nombre secret
        match input.cmp(&nombre_secret) {
            Ordering::Less => println!("{}", "Votre proposition est trop petite ! Réessayez :".red()),
            Ordering::Equal => {println!("{}", "Bravo ! Vous avez trouvé !".green()); break},
            Ordering::Greater => println!("{}", "Votre proposition est trop grande ! Réessayez :".red()),
        }
    }


}