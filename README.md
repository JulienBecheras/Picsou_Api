# Introduction

le projet Picsou vise la création d'une application mobile permettant la gestion simplifiée des dépense au sein d'un groupe de personnes.

Ce repo contient l'API du projet, écrite en Rust avec l'outil Rocket et l'ORM diesel.

# Déploiement

Cette API est déployée sur un raspberry pi 4 et connectée à une base de données postgresql.

Celle-ci est entièrement conteneurisée et facilement déployable sur n'importe quel serveur possédant docker et docker-compose.


installer les packages cargo et rustc