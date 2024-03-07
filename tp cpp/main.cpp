#include "Livre.cpp"
#include <iostream>
#include "media.cpp"

void afficher_choix(){
    cout<<"- afficher l'ensemble des livres de la bibliothèque"<<endl
    <<"- ajouter un nouveau livre"<<endl
    <<"- supprimer un livre"<<endl
    <<"- rechercher un livre (par le titre)"<<endl
    <<"- quitter"<<endl;
}

void supprimer_media(Media tab[], string nom){
    for (int i = 0; i<= indice-1; i++){
        if(tab[i].get_titre() == nom){
            for(int j = 0; j<= indice -1; j++){
                tab[i] = tab[i+1];
            }
            indice = indice -1;
        }
    }
}

void quitter(){
    exit;
}

void rechercher_media(Media tab[], string nom){
    bool trouve = false;
    for (int i = 0; i<= indice-1; i++){
        if(tab[i].get_titre() == nom){
            afficher_media();
            trouve = true;
        }
    }
    if (trouve == false){
        cout << "pas trouvé";
    }
}

void afficher_media_biblio(Livre tab[]){
    if (indice ==0){
        cout<<"le tableau est vide \n";
    }
    else{
        for (int i = 0; i <= indice-1; i++)
        {
            afficher_media()
    }
}
int main(){

    Livre bibliotheque[TAILLE]; 
    string nom_media_supprimmer;
    string nom_media_rechercher;
    int choix;

    afficher_choix();
    
    while (true){
        
        cout<<"entrer une valeur :  \n";
        cin>>choix;
        
        switch ( choix )
        {
            case 1:
                afficher_media_biblio(bibliotheque);
                break;
            case 2:
                ajout_media(bibliotheque);
                break;
            case 3:
                cout<<"livre a supprimer : \n";
                cin >> nom_livre_supprimmer;
                supprimer_livre(bibliotheque, nom_media_supprimmer);
                break;
            case 4:
                cout<<"livre a rechercher : \n";
                cin >> nom_livre_rechercher;
                rechercher_media(bibliotheque, nom_media_rechercher);
                break;
            case 5:
                quitter();
                break;
            default:
                break;
                cout << "saisie invalide \n";
        }
    }
}