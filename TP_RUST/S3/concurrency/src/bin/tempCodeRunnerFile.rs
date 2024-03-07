use std::io;


fn main() {

    let mut input :_ = String::new();
    io::stdin().read_line(&mut input);
    let n :i32 = input.parse().expect("Échec de la conversion");



    for i in 1..n-1{
        println!("Bonjour {} !", i);
        // Vous pouvez ajouter d'autres opérations ici si nécessaire
        println!("Aurevoir {} !", i);
    }
}