// Fonction de désactivation de l'affichage des "tooltips"
function deactivateTooltips() {
  var tooltips = document.querySelectorAll(".tooltip"),
    tooltipsLength = tooltips.length;

  for (var i = 0; i < tooltipsLength; i++) {
    tooltips[i].style.display = "none";
  }
}

// La fonction ci-dessous permet de récupérer la "tooltip" qui correspond à notre input

function getTooltip(elements) {
  while ((elements = elements.nextSibling)) {
    if (elements.className === "tooltip") {
      return elements;
    }
  }

  return false;
}

// Fonctions de vérification du formulaire, elles renvoient "true" si tout est ok
var check = {}; // On met toutes nos fonctions dans un objet littéral

//liste des champs
const fields = [
  {
    id: "sex",
    label: "Sexe",
    type: "input:radio",
    value: [
      { id: "H", value: "Homme" },
      { id: "F", value: "Femme" },
    ],
    alert: "Vous devez sélectionner votre sexe",
  },
  {
    id: "lastName",
    label: "Nom",
    alert: "Un nom ne peut pas faire moins de 2 caractères",
  },
  {
    id: "firstName",
    label: "Prénom",
    alert: "Un prénom ne peut pas faire moins de 2 caractères",
  },
  {
    id: "age",
    label: "Âge",
    type: "input:text",
    alert: "L'âge doit être nombre compris entre 5 et 130",
  },
  {
    id: "pseudo",
    label: "Pseudo",
    alert: "Le pseudo ne peut pas faire moins de 4 caractères",
  },
  {
    id: "pwd1",
    label: "Mot de passe",
    type: "input:password",
    alert: "Le mot de passe ne doit pas faire moins de 6 caractères",
  },
  {
    id: "pwd2",
    label: "Mot de passe (confirmation)",
    type: "input:password",
    alert:
      "Le mot de passe de confirmation doit être identique à celui d'origine",
  },
  {
    id: "country",
    label: "Pays",
    type: "select",
    alert: "Vous devez sélectionnez votre pays de résidence",
    value: [
      { id: "none", value: "Sélectionnez votre pays de résidence" },
      { id: "en", value: "Angleterre" },
      { id: "fr", value: "France" },
      { id: "us", value: "Etats-Unis" },
    ],
  },
  {
    id: "news",
    label: "Je désire recevoir la newsletter chaque mois",
    type: "input:checkbox",
  },
];
