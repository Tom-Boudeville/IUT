pub fn make_response(line: &str, text: &mut String) -> String {
    // Ajoute la ligne au texte existant
    text.push_str(line);
    
    // Ajoute la ligne d'étoiles avant et après le texte
    let response = format!("\n*************************\n{}*************************", text);

    // Efface le texte pour éviter de le conserver entre les appels
    text.clear();

    // Renvoie le résultat encadré entre deux lignes d'étoiles
    response
}

fn main() {
    let mut text_buffer = String::new();
    let line = "Hello, world!";
    
    let response = make_response(line, &mut text_buffer);

    println!("{}", response);
}
