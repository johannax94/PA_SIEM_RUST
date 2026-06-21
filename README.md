# PA_SIEM_RUST
Creation of a complete SIEM using Rust language. 

# MedTech SIEM

## Démarrage

Prérequis :

* Docker
* Docker Compose

### Démarrage

```bash
docker compose up --build
```

### Accès

Frontend :

http://localhost:5173

Backend :

http://localhost:3000

### API

Logs :

GET /logs

Alertes :

GET /alerts

Ingestion :

POST /logs

```
```


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
Queue - on met les logs d sune queue pour éviter de saturer la DB 
        ↓
Base de données stockage
        ↓
Moteur de règles 
        ↓
Alertes si règles correspondante
        ↓
Frontend

Relier une mailbox pour les alertes (notification par mail)

Les colonnes correspondantes à chaque types de logs seront gérées par le frontend (si logs proxy : url, timestamp etc.)

Pour les tests finaux :

        1 - générateur de logs en python ? !!!
        2 - importer des vrais fichiers de logs (proxy squid, linux auth.log, pfsense) et créer un parser JSON ? Meilleure solution
        3 - simuler une infra (relou)

NEXT STEP : 

queue ingestion
batch insert
rule engine async

API REST : plus maléable , plus sécurisé - quelles autres raisons ? 

Problématique  : comment gérer un nombre colossale de logs ? 

Fonctionnement des SIEM : HTTP → Queue → Worker → Batch DB → Rule Engine async POURQUOI ? 

Client
   ↓
API (rapide)
   ↓
Queue
   ↓
Worker
   ↓
DB insert (stockage)
   ↓
Rule engine (moteur SIEM )
   ↓
Alerts DB

SIEM final : comment l'incorporer dans une architecture d'entreprise ? 

Premier user : admin -> peut créer d'autres users, connecte le SIEM à son infra etc.
Quelles mesures de sécurité ont été prises pour notre SIEM (implémentation, langages choisis etc.)

Interdire la suppression du dernier admin

ne pas déployer, créer package à installer après paiement licence 


Changer le lien de localhost:5173/ A FAIRE 

Créer archi virtualisée pour intégrer notre SIEM à un cas réel. 
Créer un paquet python pr installer notre SIEM
Quel sera le prix de la licence de notre SIEM  ? 

Mise en place d'un catalogue de détection (pour ranger les alertes en grandes catégories)



DANS LES LOGS préciser timestamp UTC etc. A FAIRE

Implémenter une recharge automatique toutes les 5/15/60 min au choix - FAIT 

une vraie Landing Page moderne (style Datadog / CrowdStrike / Splunk),
une section "Features",
une section "Architecture",
une section "Screenshots",
une page "About",
une page "Contact",
un pied de page professionnel.