use std::io;
use tokio::time::{sleep, Duration};

async fn greetings_and_goodbyes(i: i32) {
    println!("Bonjour {} !", i);
    // Vous pouvez ajouter d'autres opérations ici si nécessaire
    println!("Aurevoir {} !", i);
}

#[tokio::main]
async fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée");

    let n: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Veuillez saisir un nombre valide !");
            return;
        }
    };

    let mut handles = Vec::new();

    for i in 1..=n {
        let handle = tokio::spawn(greetings_and_goodbyes(i));
        handles.push(handle);
    }

    // Attendre que tous les fils d'exécution se terminent
    for handle in handles {
        handle.await.expect("Erreur lors de l'attente du fil d'exécution");
    }
}
