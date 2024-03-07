var div1 = document.createElement("div");
div1.id = "divTP2";

var p1 = document.createElement("p");

p1.appendChild(document.createTextNode("Langages basés sur ECMAScript :"));
div1.appendChild(p1);

var ul1 = document.createElement("ul");

var li1 = document.createElement("li");
li1.appendChild(document.createTextNode("Javascript"));

var li2 = document.createElement("li");
li2.appendChild(document.createTextNode("JScript"));

var li3 = document.createElement("li");
li3.appendChild(document.createTextNode("ActionScript"));

var li4 = document.createElement("li");
li4.appendChild(document.createTextNode("EX4"));

ul1.appendChild(li1);
ul1.appendChild(li2);
ul1.appendChild(li3);
ul1.appendChild(li4);

div1.appendChild(ul1);

document.body.appendChild(div1);