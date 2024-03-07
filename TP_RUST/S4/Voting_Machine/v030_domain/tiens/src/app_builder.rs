use std::io;
use std::collections::HashMap;
use crate::Configuration; // Importer la structure Configuration du module racine
use std::str::SplitWhitespace; // Import du type SplitWhitespace
use anyhow::Result;


pub fn run_app(configuration: Configuration) -> anyhow::Result<()> {
    let mut candidats = configuration.candidates; // Utiliser la configuration passée en argument
    candidats.push("Blanc".to_string());
    candidats.push("Nul".to_string());
    let binding = candidats.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    let mut scores_map: HashMap<String, i32> = init_scores(&binding);

    let votants = vec!["Alice", "Bob", "Charlie", "Dave", "Eve", "Frank"];
    let mut votants_vote = Vec::<String>::new();

    loop {
        let mut input = String::new();

        // Lire l'entrée depuis stdin
        println!("Entrez quelque chose :");
        io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée");
        println!("Vous avez entré : {}", input);

        let mut input_parts = input.trim().split_whitespace();

        match input_parts.next() {
            Some("votants") => afficher_votants(&votants),
            Some("scores") => afficher_scores(&scores_map),
            Some("voter") => voter(&mut scores_map, &votants, &mut votants_vote, input_parts),
            _ => afficher_commande(),
        }
    }
}



fn voter(scores: &mut HashMap<String, i32>, votants: &[&str], votants_vote: &mut Vec<String>, mut input_parts: SplitWhitespace) {
    if let Some(input_votant) = input_parts.next() {
        if !votants.iter().any(|v| v == &input_votant.to_string()) {
            println!("Votant inconnu.");
            return;
        }
        
        if votants_vote.contains(&input_votant.to_string()) {
            println!("Vous avez déjà voté.");
            return;
        }

        let input_candidat = input_parts.next().unwrap_or("Blanc").to_string();
        if !scores.contains_key(&input_candidat) {
            println!("Candidat inexistant. Vote blanc.");
            return;
        }

        *scores.entry(input_candidat.clone()).or_insert(0) += 1;
        votants_vote.push(input_votant.to_string());
        println!("Vote enregistré pour {}.", input_candidat);
    } else {
        println!("Format de vote incorrect. Utilisez 'voter <nom_votant> <nom_candidat>'.");
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

fn afficher_scores(scores:&HashMap<String, i32>) {
    println!("Scores :");
    for (key, value) in scores {
        println!("{} : {}", key, value);
    }
}

fn init_scores<'a>(candidats: &'a [&'a str]) -> HashMap<String, i32> {
    let mut scores : HashMap<String, i32> = HashMap::new();
    for &candidat in candidats {
        scores.insert(candidat.to_string(), 0);
    }
    scores
}
