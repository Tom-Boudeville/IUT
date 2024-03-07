var M_S = document.getElementById("martine-stories");
var M_S_L = document.querySelector("#martine-stories li:last-child");
console.log(M_S_L);

var MAL = document.createElement("li");
MAL.appendChild(document.createTextNode("Matrine au lycée"));

M_S.insertBefore(MAL, M_S_L);

var MSM = document.createElement("li");
MSM.appendChild(document.createTextNode("Martine se marie"));

M_S.appendChild(MSM);