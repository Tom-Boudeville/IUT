var divTP4 = document.createElement("div");
divTP4.id = "divTP4";

var form1 = document.createElement("form");
form1.enctype = "multipart/form-data";
form1.method="post";
form1.action="upload.php";

var fieldset = document.createElement("fieldset");

var legend = document.createElement("legend");
legend.appendChild(document.createTextNode("Uploader une image"));

var div1 = document.createElement("div");
div1.style="text-align: center";

var label = document.createElement("label");
label.appendChild(document.createTextNode("Image à uploader : "));
label.for="inputUpload";

div1.appendChild(label);

var input1 = document.createElement("input");
input1.type="file";
input1.name="inputUpload";
input1.id="inputUpload";

div1.appendChild(input1);

div1.appendChild(document.createElement("br"));
div1.appendChild(document.createElement("br"));

var input2 = document.createElement("input");
input2.type="submit";
input2.value="Envoyer";

div1.appendChild(input2);

fieldset.appendChild(legend);

fieldset.appendChild(div1);

form1.appendChild(fieldset);

divTP4.appendChild(form1);





















document.body.appendChild(divTP4);