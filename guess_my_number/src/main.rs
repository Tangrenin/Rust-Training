use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

fn main() {
    // Génération de nombres aléatoires
    let mut rng = thread_rng();
    let nombre_secret: u32 = rng.gen_range(1..101);
    println!("Le nombre à deviner : {}", nombre_secret);


    println!("Devine mon nombre ! (entre 1 et 100) \n\nSaisissez votre proposition : ");

    loop {
        // Saisie de l'utilisateur
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // On vérifie que l'utilisateur a rentré un nombre valide
        let input = match input.trim().parse::<u32>() {
            Ok(val) =>  val,
            Err(_) => {eprintln!("Erreur : entrez un nombre entier positif :"); continue}
        };

        // Comparaison avec le nombre secret
        match input.cmp(&nombre_secret) {
            Ordering::Less => println!("Votre proposition est trop petite ! Réessayez :"),
            Ordering::Equal => {println!("Bravo ! Vous avez trouvé !"); break},
            Ordering::Greater => println!("Votre proposition est trop grande ! Réessayez :"),
        }
    }


}