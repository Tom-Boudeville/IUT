use std::io;
use std::collections::HashMap;

fn main() {
    let votants = vec!["Alice", "Bob", "Charlie", "Dave", "Eve", "Frank"];
    let candidats = vec!["Ali", "Bo", "Chaie", "Dve", "Ee", "Frak"];
    let mut votants_vote = Vec::new(); // Utilisez un Vec<&str> plutôt qu'une slice
    let mut scores_map = init_scores(&candidats);
    
    loop {
        let mut input = String::new();

        // Lire l'entrée depuis stdin
        println!("Entrez quelque chose :");
        io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée");
        println!("Vous avez entré : {}", input);

        match input.trim() {
            "" => afficher_commande(),
            "Votants" => afficher_votants(&votants),
            "Scores" => afficher_scores(&scores_map),
            "Voter" => voter(&mut scores_map, &votants, &mut votants_vote),
            _ => println!("Commande inconnue"),
        }
    }
}

fn voter(scores: &mut HashMap<&str, i32>, votants: &[&str], votants_vote: &mut Vec<String>) {
    println!("Qui est le votant ?");
    let mut input_votant = String::new();
    io::stdin().read_line(&mut input_votant).expect("Erreur lors de la lecture de l'entrée");
    let input_votant = input_votant.trim().to_string();

    if votants_vote.iter().any(|v| v == input_votant.as_str()) {
        println!("Vous avez déjà voté");
        return;
    } else if !votants.iter().any(|v| v == &input_votant.as_str()){
        println!("Votant inconnu");
        return;
    }

    println!("Entrez le nom du candidat :");
    let mut input_candidat = String::new();
    io::stdin().read_line(&mut input_candidat).expect("Erreur lors de la lecture de l'entrée");
    let input_candidat = input_candidat.trim();

    if let Some(score) = scores.get_mut(input_candidat) {
        *score += 1;
        votants_vote.push(input_votant.clone());
        println!("Vous avez voté pour {}", input_candidat);
    } else {
        println!("Ce candidat n'existe pas");
        println!("Vote blanc");
    }
    
}


fn afficher_commande() {
    println!("Commandes :");
    println!("Votants : afficher les votants");
    println!("Scores : afficher les scores");
    println!("Voter : voter pour un candidat");
}

fn afficher_votants(votants: &[&str]) {
    println!("Votants :");
    for votant in votants {
        println!("{}", votant);
    }
}

fn afficher_scores(scores: &HashMap<&str, i32>) {
    println!("Scores :");
    for (key, value) in scores {
        println!("{} : {}", key, value);
    }
}

fn init_scores<'a>(candidats: &'a [&'a str]) -> HashMap<&'a str, i32> {
    let mut scores = HashMap::new();
    for &candidat in candidats {
        scores.insert(candidat, 0);
    }
    scores
}
