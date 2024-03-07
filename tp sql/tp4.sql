
/*
create database TP4_Boudeville;


create TABLE joueur(
    idjoueur int primary key,
    nom varchar(30),
    age int,
    nationalite varchar(3)
)

create table groupe(
    idgroupe int ,
    idjoueur int,
    PRIMARY key(idgroupe,idjoueur)
)

create table jeux(
    idjeux int primary key,
    titre varchar(40)
)

create table partie(
    idgroupe int, --FOREIGN key REFERENCES groupe(idgroupe),
    idjeux int FOREIGN key REFERENCES jeux(idjeux),
    datePartie date,
    score INT
)

create table avgscore(
    idjoueur int PRIMARY key FOREIGN key REFERENCES joueur(idjoueur),
    scoremaxavg NUMERIC
)

INSERT INTO JOUEUR VALUES
(1,'Eddy',32,'US'),
(2,'Malou',53,'US'),
(3,'Jean',99,'FR'),
(4,'Nietszeucheuh',58,'AL'),
(5,'Robert',45,'FR'),
(6,'Kévin',78,'UK'),
(7,'Gaspard',24,'FR')

INSERT INTO GROUPE VALUES
(1,1),
(1,2),
(1,3),
(2,3),
(2,4),
(3,1),
(3,3),
(3,5),
(3,7),
(4,1),
(5,6)

INSERT INTO JEUX VALUES
(1,'Tetris'),
(2,'Snake'),
(3,'Clash')

INSERT INTO PARTIE VALUES
(1,1,'02/05/2022',20),
(1,2,'14/11/2022',50),
(1,2,'15/11/2022',60),
(1,3,'17/11/2022',83),
(1,3,'18/11/2022',98),
(1,3,'19/11/2022',99),
(2,1,'07/08/2022',75),
(2,1,'07/09/2022',74),
(2,1,'08/10/2022',71),
(2,3,'09/10/2022',52),
(2,3,'10/10/2022',67),
(3,2,'08/02/2022',52),
(3,2,'10/02/2022',22),
(3,3,'04/02/2022',52),
(3,3,'07/12/2022',36),
(4,1,'07/12/2022',99),
(4,2,'08/12/2022',40)


le pb : on ne peut  pas faire l'import sur la table partie(idgroupe) a cause de la clé primaire
on peut résoudre avec une fonction check



create view checkSeul
as 
select partie.idGroupe, idJeux, datePartie, score from partie 
join groupe on partie.idgroupe=groupe.idgroupe
group by partie.idgroupe, idJeux, datePartie, score
having count(idJoueur)=1;
delete from partie
where partie.idGroupe = All(select idgroupe from checkSeul);
*/

select * from partie

INSERT INTO PARTIE VALUES
(4,1,'07/12/2022',99),
(4,2,'08/12/2022',40);