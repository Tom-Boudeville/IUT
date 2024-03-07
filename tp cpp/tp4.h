//
// Created by tom-d on 19/10/2023.
//
#include <iostream>
#include <string>

#ifndef TP_CPP_TP4_H
#define TP_CPP_TP4_H

struct Noeud
{
    int valeur;
    Noeud *ag;
    Noeud *ad;
};

class Arbre {
    private:
        Noeud racine;
    public:
        Arbre(void) {
            &racine = NULL};
        void enracine(Noeud noeud, Arbre arbre1, Arbre arbre2) {
            noeud.ag = arbre1;
            noeud.ad = arbre2;

            return noeud
        }
        Arbre ag(Arbre arbre) {
            return arbre->racine.ag};
        Arbre ad(Arbre arbre) {
            return arbre->racine.ad};
        Noeud racine(Arbre arbre){
                return arbre.racine}
        bool est_vide(Arbre arbre) {
            return (racine == NULL)}
};


#endif //TP_CPP_TP4_H
