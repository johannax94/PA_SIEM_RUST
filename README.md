# PA_SIEM_RUST
Creation of a complete SIEM using Rust language. 

Backend: Rust

Frontend: React

Structure : 
- Une API pour collecter les logs et les stocker (stockage dans BDD PostgreSQL)
- Un module 'rules' pour implémenter toutes les règles de détection (moteur SIEM)
- Un système d'alerte 
- Une interface graphique utilisateur et une partie administrateur pour gérer les comptes (gestion abonnements clients)

Stockage des logs: par défaut 1 an mais cette durée sera configurable par les administrateurs
Pour que notre SIEM puisse collecter différents types de logs (réseau, systèmes, poxy, FW etc.) , nous allons adapter la section 'metadata' de notre structure Log car c'est plus optimal que de créer une structure pour chaque type.


Alertes : 
Avant de créer une alerte, vérifier si une alerte similaire existe déjà dans la dernière minute.


Structure du projet: 

Ingestion API
        ↓
Queue (plus tard)
        ↓
Base de données (PostgreSQL)
        ↓
Moteur de règles
        ↓
Alertes
        ↓
Frontend

Relier une mailbox pour les alertes (notification par mail)