var bouton1 = document.getElementById("dom");


bouton1.addEventListener('click',() =>{
    var text = document.getElementById("titre2").innerHTML;
    console.log(text);

    var longueur = document.getElementsByClassName("titre").length;
    console.log(longueur);

    var toutTexteTitre = document.getElementsByClassName("titre");
    // toutTexteTitre.array.forEach(element => {
    //     console.log(element.innerHTML);
    // });
    console.log(toutTexteTitre);


}
);

document.body.appendChild(bouton1);