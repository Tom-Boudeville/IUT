const api = {
  path: "http://filmotheque.e-mingo.net/api",
  img: "https://image.tmdb.org/t/p/",
};
api.catalog = `${api.path}/catalog`;
api.genre = `${api.catalog}/genre`;
api.actor = `${api.catalog}/actor`;
api.movie = `${api.catalog}/movie`;

/**
 * Démarrage
 */
const init = async () => {
  try {
    //Configuration des élements du menu
    setMenu();

    //Chargement de la page d'accueuil
    displayPage("home");
  } catch (error) {
    displayMsg(error, "warning");
  }
};

/**
 *
 * Récupération des données en json
 * @param {*} src  point de terminaison
 * @returns
 */
const getData = async (src) => {
  try {
    const reponse = await fetch(`${src}`);
    return reponse.json();
  } catch (error) {
    throw new Error(`Erreur lors de la récupération des données : ${error.message}`);
  }
};


//test de la fonction getData
getData(api.movie).then((movie) => console.log(movie)); //10 premiers films

/**
 * Récupération de la liste des genre et mise à jour du menu
 */
const getGenres = async () => {
  try {
    const response = await getData(api.genre);
    return response;
  } catch (error) {
    throw new Error(`Erreur lors de la récupération des données : ${error.message}`);
  }
};

//test de la fonction getGenres
//getGenres().then((genre) => console.log(genre));

/**
 * Récupération des acteurs et mise à jour de la liste
 */
const getActors = async (name) => {
  try {
    if (name === undefined) {
      const response = await getData(api.actor);
      return response;
    }
    else{
      const response = await getData(`${api.actor}/${name}`);
      return response;
    };
  } catch (error) {
    throw new Error(`Erreur lors de la récupération des données : ${error.message}`);
  }
};
//test de la fonction getActors
//getActors().then((actor) => console.log(actor));
//getActors("Hamill").then((actor) => console.log(actor));

const displayMovie = async (movieIdOrName) => {
  try {
  const movie = getData(api.movie+"/"+movieIdOrName);
  const divmere = document.createElement("div");
  divmere.innerHTML = `
  <div class='container'>
    <div class='row'>
      <div class='col-12 col-md-6'>
        <img class='rounded' src='${getData(`${api.img}${movie.poster_path}`)}' alt='Affiche Principale'>
      </div>
      <div class='col-12 col-md-6'>
        <h1 class='title'>${movie.title}</h1>
        <h2 class='tagline'${movie.tagline}.</h2>
        <p class='overview'>${movie.overview}</p>
        <p class='release_date'>${movie.release_date}</p>
      </div>
    </div>
    <div class='row'>
      <div class='col-12'>
        <h2>Acteurs</h2>
      </div>
    </div>
    <div class="actors_info">
      <div class="head">
        <h3>Acteurs</h3>
        <p class="text-right"><a href="#">Tous</a></p>
      </div>

      <div class="people grid grid-cols-4">
        
      </div>
     </div> 
  </div>`
  ;

  
  console.log(api.movie+"/"+movieIdOrName);
  console.log(movie.credits);


  const tousLesElementsActeur = divmere.getElementsByClassName('actors_info');
  var deuxiemeElementActeur = tousLesElementsActeur[1];

  const acteurs = getData(api.path+ "/catalog/actor/movieIdOrName/movie");
  for (let i =0; i <= acteurs.length ;i++) {
    var acteur = document.getElementsByClassName("card");
    divActeur.innerHTML = `
    <span class="card rounded"
          ><a
            href="${getActors(acteur[i])}"
            ><img
              class="profile"
              src="${acteur.poster_path}"
              alt="${movie.credits.cast[i].name}"
          /></a>
          <p>
            <a
              href="${getActors(acteur[i])}"
              >${movie.credits.cast[i].name}</a
            >
          </p>
          <p class="character">${movie.credits.cast[i].character}</p> 
        </span>

      <div class='col-12 col-md-6'>
        <div class='card'>
          <img class='rounded' src='${getActors(acteur[i])}' alt=''>
          <p class='name'>${acteur.original_name}</p>
        </div>
      </div>`;
    deuxiemeElementActeur.appendChild(divActeur);
  }
  
  
  
  } catch (error) {
    throw new Error(`Erreur lors de la récupération des données : ${error.message}`);
  }
};
//test de la fonction getMovie
displayMovie(76600);
//displayMovie("coco")

/**
 *
 * @param {*} event  Evenement déclencheur
 */
const displayActors = (event) => {};

//lancement de la fonction init au chargement , pour alimenter le menu
document.addEventListener("DOMContentLoaded", init);