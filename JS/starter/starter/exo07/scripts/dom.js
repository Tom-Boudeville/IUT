var figure = document.createElement("figure");

var image = document.createElement("img");
image.src="https://picsum.photos/200/300";
image.alt="Image générée aléatoirement";

var figcaption = document.createElement("figcaption");
figcaption.innerHTML="Image générée aléatoirement";

figure.appendChild(image);
figure.appendChild(figcaption);

document.body.appendChild(figure);