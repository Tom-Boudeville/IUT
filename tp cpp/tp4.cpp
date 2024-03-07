//
// Created by tom-d on 19/10/2023.
//

#include "tp4.h"

using namespace std;

void ajouter_noeud(Noeud racine, int valeur){
    racine->valeur = valeur;
    racine.ad = NULL;
    racine.ag = NULL;
}

void prefixe(Noeud noeud){
    cout<<noeud->valeur;
    prefixe(noeud.ag);
    prefixe(noeud.ad);
}

int main(){
    Arbre arbre = new Arbre(void);

    ajouter_noeud(arbre->racine,3);

    ajouter_noeud(arbre.ag,5);
    ajouter_noeud(arbre.ad,8);

    Noeud noeud_actuel = arbre.ad(arbre.ad);

    ajouter_noeud(noeud_actuel.ad,1);
    ajouter_noeud(noeud_actuel.ag,7);

    prefixe(arbre->racine);
}