#include <string>
#include <iostream>
#include <stdlib.h>

using namespace std;

class Media{

        private :
            string titre;
            string nom_auteur;
            int date;

        public : 
            string get_titre(){
                return titre;
            }
            string get_nom_auteur(){
                return nom_auteur;
            }
            int get_date(){
                return date;
            }
            Media afficher_media();
};
