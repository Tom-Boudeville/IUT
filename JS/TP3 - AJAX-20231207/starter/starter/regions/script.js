const path = "https://geo.api.gouv.fr";
const regList = document.getElementById("regions");
const dptList = document.getElementById("departements");
const cityList = document.getElementById("villes");
const cityDetails = document.getElementById("cityDetails");

const displayError = (message, type = "alert") => {
  alert.log(message, type);
};

/**
 *
 * Mise à jour du compteur des listes
 * @param {*} list
 * @param {*} value
 */
const setCounter = (list, value) => {
  list.parentElement.parentElement.querySelector(".counter").innerText = value;
};

/**
 * Affichage des infos de la commune survolée
 * @param {*} e
 */
const displayInfo = (e) => {
  //Maj du contenu
  const elt = e.target;
  cityDetails.innerHTML = `<h2 class="border-b-2 border-gray-500  ">${
    elt.innerText
  }</h2><p class="text-sm text-gray-500">Population : ${elt.getAttribute(
    "elt-population"
  )}<br>Surface : ${elt.getAttribute("elt-surface")}</p>`;

  //Positionnement
  cityDetails.style.left =
    e.target.offsetLeft + e.target.offsetWidth - cityDetails.offsetWidth + "px";
  cityDetails.style.top = e.target.offsetTop - 10 + "px";
  cityDetails.style.display = "block";
};

/**
 * Fonction de changement du style d'un item sélectionné dans une liste
 * @param {*} e
 */
const setItemStatus = (e) => {
  const style = "bg-slate-100";
  //On désactive les autres régions actives
  const oldActive = e.currentTarget.querySelectorAll(`.${style}`);
  [...oldActive].forEach((element) => element.classList.remove(style));

  e.target.classList.add(style); //On modifie l'élement sélectionné
};
var tousLesElementsSpan = document.querySelectorAll('.counter');
var premierElementSpan = tousLesElementsSpan[0]; // Index 1 pour le deuxième élément (0-indexed)
var deuxiemeElementSpan = tousLesElementsSpan[1];
var troisiemeElementSpan = tousLesElementsSpan[2];



fetch("https://geo.api.gouv.fr/regions")
    .then(function (response) {
        return response.json();
    })
    .then(function (data) {
      /*on modifie le compteur de region*/
      premierElementSpan.textContent = data.length;

        /* on affiche les regions*/
        data.forEach((region) => {

            let li = document.createElement("li");
            li.textContent = region.nom;
            regList.appendChild(li);
            
            /*sur chaque clique de regions*/
            li.addEventListener("click", () => {

                fetch("https://geo.api.gouv.fr/regions/" + region.code + "/departements")
                    .then(function (response) {
                        return response.json();
                    })
                    /*on enleve les precedents resultats*/
                    .then(function (data) {
                      supprimerAncienResultat(dptList);
                      troisiemeElementSpan.textContent = 0;
                      supprimerAncienResultat(cityList);

                      deuxiemeElementSpan.textContent = data.length;
                        /*on met a jour les departements en fonction de la region*/
                        data.forEach((departement) => {
                            let li = document.createElement("li");
                            li.textContent = departement.nom;
                            dptList.appendChild(li);
                            li.addEventListener("click", () => {

                                fetch("https://geo.api.gouv.fr/departements/" + departement.code + "/communes")
                                    .then(function (response) {
                                        return response.json();
                                    })
                                    /*on enleve les precedents resultats*/
                                    .then(function (data) {
                                      console.log(data);
                                      supprimerAncienResultat(cityList);
                                      troisiemeElementSpan.textContent = data.length;

                                        /*on met a jour les villes en fonction du departement*/
                                        data.forEach((ville) => {
                                            let li = document.createElement("li");
                                            li.textContent = ville.nom;
                                            li.setAttribute("elt-population", ville.population);
                                            li.setAttribute("elt-surface", ville.surface);
                                            cityList.appendChild(li);
                                            li.addEventListener("mouseover", displayInfo);
                                            li.addEventListener("mouseout", () => {
                                              cityDetails.style.display = "none";
                                            });
                                        });
                                    })
                            })
                        });
                    })
            })
        });
    })

function supprimerAncienResultat(zone) {
  while (zone.firstChild) {
    zone.removeChild(zone.firstChild);
  }
}