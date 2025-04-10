# Introduction

le projet Picsou vise la création d'une application mobile permettant la gestion simplifiée des dépense au sein d'un groupe de personnes.

Ce repo contient l'API du projet, écrite en Rust avec l'outil Rocket et l'ORM diesel.

# Déploiement de ce projet

## Plateforme

Cette API est déployée sur un raspberry pi 4 et connectée à une base de données postgresql.

Celle-ci est entièrement conteneurisée et facilement déployable sur n'importe quel serveur possédant docker et docker-compose.

## Sécurité

L'api et la base de données fonctionnent independent sur deux conteneurs séparés.

Un pare-feu est mis en place pour restreindre l'accès à l'api uniquement.

Un certificat SSL a été créé à partir de Let's Encrypt pour sécuriser les échanges entre le client et l'api.

Le serveur du raspberry pi a été configuré avec apache2.

## Déploiement

Ce projet est en déploiement continu. Ainsi, chaque commit sur main déclenche un pull sur le raspberry pi, un build du projet et de l'image, execution des migration et déploiement de la nouvelle image. 

# Installation

Vous pouvez deployer l'api sur votre machine en clonant le projet et en exécutant la commande suivante :

```bash
docker-compose up
```

Cela créera un conteneur avec l'api et un conteneur avec la base de données. Veillez à créer un fichier `.env` à la racine du projet.