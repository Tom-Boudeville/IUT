use std::net::UdpSocket;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // Récupère les arguments de la ligne de commande
    if args.len() < 3 {
        eprintln!("Usage: udp_ping_client <server_ip> <server_port>"); // Affiche un message d'erreur si les arguments sont manquants
        return;
    }


    println!("le port est {}",args[2]);
    let server_ip = &args[1]; // Récupère l'adresse IP du serveur à partir du premier argument
    let server_port: u16 = match args[2].parse() { // Récupère le numéro de port du serveur à partir du deuxième argument
        Ok(port) => port,
        Err(_) => {
            eprintln!("Invalid server port"); // Affiche un message d'erreur si le numéro de port est invalide
            return;
        }
    };

    let socket = match UdpSocket::bind("0.0.0.0:0") { // Crée une socket UDP en écoutant sur un port aléatoire
        Ok(socket) => socket,
        Err(err) => {
            eprintln!("Failed to bind socket: {}", err); // Affiche un message d'erreur si la création de la socket échoue
            return;
        }
    };

    let message = "PING".as_bytes(); // Convertit la chaîne de caractères "PING" en tableau de bytes
    match socket.send_to(message, format!("{}:{}", server_ip, server_port)) { // Envoie le message au serveur
        Ok(_) => println!("Message sent successfully"), // Affiche un message de succès si l'envoi est réussi
        Err(err) => eprintln!("Failed to send message: {}", err), // Affiche un message d'erreur si l'envoi échoue
    }
}
