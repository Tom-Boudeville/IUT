use structopt::StructOpt;
use port_scanner::{is_open_sync, Options};

fn main() {
    // Mesure le temps d'exécution
    let my_instant = std::time::Instant::now();

    // Parse les options de la ligne de commande
    let options = Options::from_args();
    let host = options.host;
    let port_min = options.port_min;
    let port_max = options.port_max;
    let timeout = options.timeout;

    // Parcourt les ports de port_min à port_max
    for port in port_min..=port_max {
        // Vérifie si le port est ouvert
        if is_open_sync(&host, port, timeout) {
            println!("{} is open", port);
        }
    }

    // Affiche le temps d'exécution
    println!("{:?}", my_instant.elapsed());
}
