use std::io;
use std::thread;

fn main() {
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
        let handle = thread::spawn(move || {
            println!("Bonjour {} !", i);
            // Vous pouvez ajouter d'autres opérations ici si nécessaire
            println!("Aurevoir {} !", i);
        });

        handles.push(handle);
    }

    // Attendre que tous les fils d'exécution se terminent
    for handle in handles {
        handle.join().expect("Erreur lors de l'attente du fil d'exécution");
    }
}
