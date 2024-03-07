var liste = document.querySelectorAll("#output li");

liste.forEach((element) => {
    element.addEventListener('click', () => {
        const valeur = prompt();
        if (valeur !=""){
            element.textContent=valeur;
        }

    });
})