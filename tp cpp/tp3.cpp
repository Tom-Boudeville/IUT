#include <iostream>
#include <string>
#include "maillon.cpp"

using namespace std;

class Chaine
{
private:
    Maillon tete;
    Maillon queue;
public:
    Chaine(void);
    ~Chaine();
    void adjtete(int valeur);
    void suptete();
    void adjqueue(int valeur);
    void supqueue();
    int longueur() const;
    int element(int position) const;
    void adjpos(int valeur, int position);
    void suppos(int position);
    void vide();
    bool est_vide() const;
    void affiche();
    };

Chaine::chaine(void)
{
    tete= NULL;
    queue = NULL;
}

Chaine::adjtete(int valeur){
    maillon nouv_maillon = maillon();
    maillon ancient_maillon = chaine->tete;
    tete = nouv_maill0n;
    mouv_maillon->suivant = ancient_maillon;
}

Chaine::suptete(){
    maillon maillon_supp = tete.suivant;
    tete = tete->suivant->suivant;
    ~maillon_supp->~maillon();   
}

Chaine::adjqueue(int valeur){
    maillon ancient_maillon = queue;
    maillon nouv_maillon = maillon();
    nouv_maillon->valeur = valeur;
    queue = nouv_maillon;
    ancient_maillon->suivant = nouv_maillon;
}

Chaine::supqueue(){
    maillon = tete;
    while (maillon->suivant->suivant != NULL)
    {
        maillon = maillon->suivant;
    }
    queue = maillon;
    maillon->suivant->~maillon();
}

Chaine::const longueur(){
        int compteur = 0;
        maillon = tete;
    while (maillon->suivant != NULL)
    {
        maillon = maillon->suivant;
        compteur++;
    }
    return compteur;
}

Chaine::0element(int position){
    maillon = tete;
    for(int i=0, i<=position; i++){
        maillon = maillon->suivant;
    }
    return maillon->valeur;
}

Chaine::adjpos(int valeur, int position){
    maillon_avant = tete;
    for(int i=0, i<position; i++){
        maillon_avant = maillon_avant->suivant;
    }
    maillon_apres = maillon_avant->suivant;
    maillon maillon_inserer = maillon();
    maillon->valeur = valeur;
    maillon_inserer->suivant = maillon_apres;
    maillon_avant->suivant = maillon;
}

Chaine::suppos(int position){
    maillon maillon_avant = tete;
    for(int i=0, i<position; i++){
        maillon_avant = maillon_avant->suivant;
    }
    maillon maillon_supp = maillon_avant->suivant;
    maillon maillon_apres = maillon_supp->suivant;
    maillon_avant->suivant = maillon_apres;
    maillon = ~maillon();
}

Chaine::void vide(){
    maillon maillon = tete;
    while(maillon->suivant != NULL)
    {
        maillon maillon_suivant = maillon->suivant;
        maillon->~maillon();
        maillon = maillon_suivant;   
    }
    maillon = ~maillon();
}

Chaine::bool const est_vide(){
    return (tete == NULL);
}

Chaine::void affiche(){
    if(est_vide){
        printf("C'est vide");
    }
    else if(longueur()==1){
        printf(maillon->valeur);
    }
    else{
        while(maillon.suivant != NULL){
            printf(maillon->valeur);
        }
    }
}