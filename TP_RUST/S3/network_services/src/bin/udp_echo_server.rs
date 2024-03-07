use std::net::UdpSocket;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // Récupère les arguments de la ligne de commande
    if args.len() < 1 {
        eprintln!("Usage: udp_pong_serveur <server_port>"); // Affiche un message d'erreur si les arguments sont manquants
        return;
    }

    let server_port: u16 = match args[1].parse() { // Récupère le numéro de port du serveur à partir du deuxième argument
        Ok(port) => port,
        Err(_) => {
            eprintln!("Invalid server port"); // Affiche un message d'erreur si le numéro de port est invalide
            return;
        }
    };

    let socket = match UdpSocket::bind(format!("0.0.0.0:{}", server_port)) { // Crée une socket UDP en écoutant sur le port spécifié
        Ok(socket) => socket,
        Err(err) => {
            eprintln!("Failed to bind socket: {}", err); // Affiche un message d'erreur si la création de la socket échoue
            return;
        }
    };

    let mut buffer = [0; 1024]; // Crée un tampon pour stocker les données reçues

    loop {
        match socket.recv_from(&mut buffer) { // Attend de recevoir des données d'un client
            Ok((size, client_addr)) => {
                let message = &buffer[..size]; // Récupère le message reçu
                println!("Received message from {}: {}", client_addr, String::from_utf8_lossy(message));

                //let response = message.as_bytes(); // Convertit la chaîne de caractères "PONG" en tableau de bytes
                match socket.send_to(message, client_addr) { // Envoie la réponse au client
                    Ok(_) => println!("Sent back to {}", client_addr), // Affiche un message de succès si l'envoi est réussi
                    Err(err) => eprintln!("Failed to send back to {}: {}", client_addr, err), // Affiche un message d'erreur si l'envoi échoue
                }
            }
            Err(err) => eprintln!("Failed to receive message: {}", err), // Affiche un message d'erreur si la réception échoue
        }
    }
}

