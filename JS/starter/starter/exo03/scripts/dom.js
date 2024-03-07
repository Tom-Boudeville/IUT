// On crée l'élément conteneur
var mainDiv = document.createElement("div");
mainDiv.id = "divTP3";

// On place le texte dans des objets, eux-mêmes placés dans un tableau
// Par facilité, la création des nœuds textuels se fera dans la boucle
var languages = [
  {
    t: "JavaScript",
    d: "JavaScript est un langage de programmation de scripts principalement utilisé dans les pages web interactives mais aussi coté serveur.",
  },
  {
    t: "JScript",
    d: "JScript est le nom générique de plusieurs implémentations d'ECMAScript 3 créées par Microsoft.",
  },
  {
    t: "ActionScript",
    d: "ActionScript est le langage de programmation utilisé au sein d'applications clientes (Adobe Flash, Adobe Flex) et serveur (Flash media server, JRun, Macromedia Generator).",
  },
  {
    t: "EX4",
    d: "ECMAScript for XML (E4X) est une extension XML au langage ECMAScript.",
  },
];

var p1 = document.createElement("p");
p1.appendChild(document.createTextNode("Langages basés sur ECMAScript :"));
mainDiv.appendChild(p1);

var dl = document.createElement("dl");

languages.forEach(element => {
  var key = element.t;
  var val = element.d;
  dt = document.createElement("dt");
  dt.appendChild(document.createTextNode(key));
  
  dd = document.createElement("dd");
  dd.appendChild(document.createTextNode(val));

  dl.appendChild(dt);
  dl.appendChild(dd);
});

mainDiv.appendChild(dl)

document.body.appendChild(mainDiv);