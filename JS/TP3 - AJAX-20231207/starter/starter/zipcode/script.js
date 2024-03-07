var codePostal =  document.getElementById("zipcode");

codePostal.addEventListener("input", () => {
  var codePostal = document.getElementById("zipcode");

  codePostal.addEventListener("input", function () {
    if (codePostal.value.length == 5) {
      fetch("https://geo.api.gouv.fr/communes?codePostal=" + codePostal.value)
      
      .then(function (response) {
          return response.json();
        })

        .then(function (data) {
          data.forEach((element) => {
            console.log(element.nom);
          });

          ajouterVille(data);
        });

      console.log(codePostal.value);
    }
})
});



function ajouterVille(villes) {
  let city = document.getElementById("city");

  while (city.firstChild) {
    city.removeChild(city.firstChild);
  }

  if (villes.length == 0) {
    let option = document.createElement("option");
    option.value = "Aucune ville";
    option.textContent = "Aucune ville";
    city.appendChild(option);
  } 
  else {
    villes.forEach((element) => {
      let option = document.createElement("option");
      option.value = element.nom;
      option.textContent = element.nom;
      city.appendChild(option);
    });
  }
}