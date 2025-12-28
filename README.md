# 🎓 Nschool - Student Management System

Système de gestion d'étudiants en ligne de commande développé en Rust.

## 📋 Description

Nschool est une application CLI (Command Line Interface) pour la gestion des étudiants, matières et notes. Le système nécessite une authentification administrateur pour accéder aux fonctionnalités.

## ✨ Fonctionnalités

- **Authentification** : Système de connexion sécurisé pour les administrateurs
- **Gestion des étudiants** : Visualisation de tous les étudiants ou recherche par matricule
- **Gestion des matières** : Consultation des matières et de leurs coefficients
- **Gestion des notes** : Visualisation des notes par étudiant ou par matière
- **Affichage en tableaux** : Présentation claire des données avec `comfy-table`

## 🚀 Installation

### Prérequis

- Rust (version 1.70 ou supérieure)
- Cargo (inclus avec Rust)

### Étapes d'installation

1. Clonez le repository :
```bash
git clone <repository-url>
cd nschool
```

2. Compilez le projet :
```bash
cargo build --release
```

3. Exécutez l'application :
```bash
cargo run
```

Ou utilisez la version release :
```bash
./target/release/nschool
```

## 🔐 Connexion

L'application nécessite une authentification. Voici les comptes administrateurs disponibles :

| Login | Password |
|-------|----------|
| admin | admin |
| admin1 | password1 |
| admin2 | password2 |
| admin3 | password3 |
| admin4 | password4 |
| admin5 | password5 |
| admin6 | password6 |
| admin7 | password7 |
| admin8 | password8 |
| admin9 | password9 |

## 📖 Utilisation

### Menu principal

Une fois connecté, vous accédez au menu principal avec les options suivantes :

1. **Voir tous les étudiants** - Affiche la liste complète des étudiants
2. **Voir un étudiant (par matricule)** - Recherche un étudiant spécifique et affiche ses notes
3. **Voir toutes les matières** - Liste toutes les matières disponibles
4. **Voir une matière (par code)** - Affiche les détails d'une matière et les notes associées
5. **Voir toutes les notes** - Affiche toutes les notes du système
6. **Voir les notes d'un étudiant** - Recherche les notes d'un étudiant par matricule
7. **Voir les notes d'une matière** - Affiche toutes les notes pour une matière donnée
8. **Déconnexion** - Se déconnecter et retourner à l'écran de connexion
9. **Quitter** - Fermer l'application

## 📁 Structure du projet

```
nschool/
├── src/
│   ├── main.rs              # Point d'entrée de l'application
│   ├── app.rs               # État de l'application et gestion des repositories
│   ├── auth/
│   │   └── mod.rs           # Module d'authentification
│   ├── cli/
│   │   └── mod.rs           # Gestion de l'interface en ligne de commande
│   ├── views/
│   │   └── mod.rs           # Affichage des données en tableaux
│   └── db/
│       ├── mod.rs           # Initialisation de la base de données
│       ├── bootstrap.rs     # Ouverture de la base de données
│       ├── migrate.rs       # Système de migrations
│       ├── seeder.rs        # Système de seeding
│       └── repositories/    # Repositories pour les entités
│           ├── student_repository.rs
│           ├── subject_repository.rs
│           ├── note_repository.rs
│           └── admin_repository.rs
├── sql/
│   ├── migrations/          # Scripts de migration SQL
│   │   ├── 001_init.sql
│   │   └── 002_add_admin.sql
│   └── seeders/             # Scripts de seeding SQL
│       ├── 001_initial_seed.sql
│       └── 002_admin_seed.sql
├── database/                # Base de données SQLite (créée à l'exécution)
│   └── nschool.sqlite
├── Cargo.toml               # Configuration du projet Rust
└── README.md                # Ce fichier
```

## 🗄️ Base de données

Le système utilise SQLite pour stocker les données. La base de données est automatiquement créée et initialisée lors du premier lancement de l'application.

### Tables

- **students** : Informations sur les étudiants (matricule, nom, prénom, classe)
- **subjects** : Matières (code, nom, classe, coefficient)
- **notes** : Notes des étudiants (id, valeur, matricule, code matière)
- **admins** : Comptes administrateurs (id, login, password)

### Migrations et Seeders

Les migrations sont exécutées automatiquement au démarrage pour créer les tables. Les seeders remplissent la base de données avec des données initiales si elle est vide.

## 🛠️ Développement

### Exécuter les tests

```bash
cargo test
```

### Compiler en mode debug

```bash
cargo build
```

### Compiler en mode release

```bash
cargo build --release
```

## 📦 Dépendances

- **rusqlite** : Driver SQLite pour Rust
- **comfy-table** : Bibliothèque pour afficher des tableaux dans le terminal

## 🔧 Configuration

La base de données est créée dans le dossier `database/` avec le nom `nschool.sqlite`. Vous pouvez modifier le chemin dans `src/main.rs` si nécessaire.

## 📝 Notes

- Les mots de passe sont stockés en clair (non hashés) pour le moment. Pour un environnement de production, il faudrait implémenter un système de hashage.
- La base de données est créée automatiquement au premier lancement.
- Les seeders ne s'exécutent que si la base de données est vide (sauf pour les admins qui sont toujours vérifiés).

## 🤝 Contribution

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou une pull request.

## 📄 Licence

[Spécifiez votre licence ici]

## 👤 Auteur

Strife-Cyber

---

**Version** : 1.0.0

