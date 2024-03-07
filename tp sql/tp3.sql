/*create database G4A_BOUDEVILLE_TOM_TP3_2;

CREATE TABLE ETUDIANT (
	idEtudiant INT PRIMARY KEY,
	nom VARCHAR(30) NOT NULL,
	prenom VARCHAR(30) NOT NULL
)

CREATE TABLE PROJET (
	idProjet INT PRIMARY KEY,
	titre VARCHAR(30) NOT NULL,
	description VARCHAR(30) NULL
)

CREATE TABLE MATIERE (
	idMatiere INT PRIMARY KEY,
	intitule VARCHAR(30) NOT NULL,
	coefficient NUMERIC(2,1) NOT NULL
)

CREATE TABLE GROUPE (
	idEtudiant INT FOREIGN KEY REFERENCES ETUDIANT(idEtudiant) NOT NULL,
	idProjet INT FOREIGN KEY REFERENCES PROJET(idProjet) NOT NULL,
	PRIMARY KEY (idEtudiant, idProjet)
)

CREATE TABLE NOTEPROJET (
	idProjet INT FOREIGN KEY REFERENCES PROJET (idProjet) NOT NULL,
	noteProjet NUMERIC(4,2) NOT NULL,
	semestre INT NOT NULL,
	PRIMARY KEY (idProjet)
)

CREATE TABLE NOTE (
	idEtudiant INT FOREIGN KEY REFERENCES ETUDIANT(idEtudiant) NOT NULL,
	idMatiere INT FOREIGN KEY REFERENCES MATIERE(idMatiere) NOT NULL,
	note NUMERIC(4,2) NOT NULL,
	semestre INT NOT NULL
)




insert into ETUDIANT values (1,'Dupont','Joe')
insert into ETUDIANT values (2,'Maro','Mike')
insert into ETUDIANT values (3,'Matin','Martine')
insert into ETUDIANT values (4,'Toulouse','Michel')
insert into ETUDIANT values (5,'Olive','Olivier')
insert into ETUDIANT values (6,'Rocky','John')

insert into PROJET values (10,'Projet Web','super projet web')
insert into PROJET values (20,'Projet PPP','super projet PPP')
insert into PROJET values (30,'Projet SQL','super projet SQL')
insert into PROJET values (40,'Projet ALGO','super projet ALGO')
insert into PROJET values (50,'Projet Forum','super projet Forum')

insert into MATIERE values (100,'MATH',5.5)
insert into MATIERE values (200,'INFO',7)
insert into MATIERE values (300,'ANGLAIS',3)

insert into GROUPE values (1,10)
insert into GROUPE values (2,10)
insert into GROUPE values (3,20)
insert into GROUPE values (4,20)
insert into GROUPE values (5,30)
insert into GROUPE values (5,40)
insert into GROUPE values (1,40)

insert into NOTEPROJET values (10,12,1)
insert into NOTEPROJET values (20,14,1)
insert into NOTEPROJET values (30,16,2)
insert into NOTEPROJET values (40,18,3)

insert into NOTE values (1,100,11,1)
insert into NOTE values (1,200,9,1)
insert into NOTE values (1,300,3,1)
insert into NOTE values (1,100,10,2)
insert into NOTE values (1,200,8,2)
insert into NOTE values (1,300,7,2)
insert into NOTE values (1,100,4,3)
insert into NOTE values (1,200,5,3)
insert into NOTE values (1,300,18,3)

insert into NOTE values (2,100,18,1)
insert into NOTE values (2,200,19,1)
insert into NOTE values (2,300,15,1)
insert into NOTE values (2,100,11,2)
insert into NOTE values (2,200,8,2)
insert into NOTE values (2,300,5,2)
insert into NOTE values (2,100,17,3)
insert into NOTE values (2,200,12,3)
insert into NOTE values (2,300,4,3)

insert into NOTE values (3,100,15,1)
insert into NOTE values (3,200,10,1)
insert into NOTE values (3,300,9,1)
insert into NOTE values (3,100,5,2)
insert into NOTE values (3,200,8,2)
insert into NOTE values (3,300,4,2)
insert into NOTE values (3,100,14,3)
insert into NOTE values (3,200,13,3)
insert into NOTE values (3,300,18,3)


select top 5 E.nom, E.prenom, avg(note) as moyenne from ETUDIANT E
join note n on e.idEtudiant=n.idEtudiant
where n.semestre=2
group by e.idEtudiant, e.nom, e.prenom
order by moyenne desc


select E.nom, E.prenom, noteProjet from ETUDIANT E
join groupe on GROUPE.idEtudiant=e.idEtudiant
join NOTEPROJET on noteProjet.idProjet=groupe.idProjet
where semestre=1 and noteProjet>= all (select  max(noteprojet) from NOTEPROJET where semestre = 1)

drop view vue;
*/
CREATE VIEW VueMoyennesSemestre AS
SELECT
    N.semestre AS Semestre,
    N.idEtudiant,
    AVG(N.note) AS MoyenneMatiere,
    AVG(NP.noteProjet) AS MoyenneProjet
FROM NOTE N
LEFT JOIN GROUPE G ON N.idEtudiant = G.idEtudiant
LEFT JOIN NOTEPROJET NP ON G.idProjet = NP.idProjet AND N.semestre = NP.semestre
GROUP BY N.semestre, N.idEtudiant;

select * from VueMoyennesSemestre;

