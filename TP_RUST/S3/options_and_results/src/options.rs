fn main(){
    /*
    let sentence1 = "Bonjour Limoges";
    let sentence2 = "";

    print_first_word1(sentence1);
    print_first_word1(sentence2 );
    */
    let string1 = "42";
    let string2  = "Oui";
    //convert_to_int2(string1);
    //convert_to_int2(string2);

    match  convert_to_int3(string1){
        Ok(nombre) => println!("Conversion réussie : {}", nombre),
        Err(erreur) => eprintln!("Erreur de conversion : {}", erreur),
    }

    match  convert_to_int3(string2){
        Ok(nombre) => println!("Conversion réussie : {}", nombre),
        Err(erreur) => eprintln!("Erreur de conversion : {}", erreur),
    }
}

fn print_first_word1(chaine : &str){
    
    let mots :Vec<&str> = chaine.split_whitespace().collect();

    let mut iterateur = mots.iter();

    if let Some(premier_element) = iterateur.next(){
        println!("Le premier mot est : {}", premier_element);
    }
    else{
        println!("Chaîne vide");
    }
}

fn convert_to_int2(chaine : &str){
    let nombre: i32 = chaine.parse().expect("Échec de la conversion");

    println!("Conversion réussie : {}", nombre);

}

fn convert_to_int3(chaine : &str) -> Result<i32>{
    let nombre : i32 = chaine.parse()?;

    Ok(nombre)

}
