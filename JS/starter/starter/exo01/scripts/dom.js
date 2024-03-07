var divTP1 = document.createElement("div");
divTP1.id = "divTp1";

divTP1.appendChild(document.createTextNode("Le "));

var strong1 = document.createElement("strong");
strong1.appendChild(document.createTextNode("World Wide Web Consortium"));
divTP1.appendChild(strong1);

divTP1.appendChild(document.createTextNode(", abrégé par le sigle"));

var strong2 = document.createElement("strong");
strong2.appendChild(document.createTextNode(", est un"));
divTP1.appendChild(strong2);

var a1 = document.createElement("a");
a1.href = "http://fr.wikipedia.org/wiki/Organisme_de_normalisation";
a1.title = "Organisme de normalisation";
a1.appendChild(document.createTextNode("Organisme de normalisation"));

divTP1.appendChild(a1);

divTP1.appendChild(document.createTextNode("  à but non-lucratif chargé de promouvoir la compatibilité des technologies du"));

var a2 = document.createElement("a");
a2.href = "http://fr.wikipedia.org/wiki/World_Wide_Web";
a2.title = "World Wide Web";
a2.appendChild(document.createTextNode("World Wide Web"));

divTP1.appendChild(a2);
divTP1.appendChild(document.createTextNode("."));

document.body.appendChild(divTP1);