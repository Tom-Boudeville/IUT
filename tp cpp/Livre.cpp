#include <string>
#include <iostream>
#include <stdlib.h>
#include "media.cpp"

using namespace std;

class Livre : public Media{

        private :
            int nb_page;

        public:
            int get_nb_pages(){
                return nb_page;
            }
            Livre creer_livre(){
                Livre nv_livre;
                cout<<"Saisir le titre du livre :";
                cin>>nv_livre.get_titre();
                cout<<"Saisir le nom de l'auteur :";
                cin>>nv_livre.get_nom_auteur();
                cout<<"Saisir la date à laquelle le livre est sortie :";
                cin>>nv_livre.get_date();
                cout<<"Saisir le nombre de pages du livre :";
                cin>>nv_livre.get_nb_pages();
                return nv_livre;
            }
};


const int TAILLE = 250;
int indice = 0;

void afficher_media(Livre livre){
    cout<<get_titre()<<endl;
    cout<<get_nom_auteur()<<endl;
    cout<<get_date()<<endl;
    cout<<get_nb_pages()<<endl<<endl;   
}








