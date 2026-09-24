# Mini-shell

Un shell interactif basique écrit en Rust : lecture de commandes, exécution de programmes externes, et quelques built-ins essentiels.

## Fonctionnalités

- Boucle de lecture-exécution (REPL) avec prompt
- Parsing d'une ligne de commande en commande + arguments, avec **support des guillemets** (`echo "hello world"` traite `hello world` comme un seul argument)
- Exécution de commandes externes via `std::process::Command`
- Built-ins :
  - `cd` (changement de répertoire, avec fallback vers `$HOME` si aucun argument n'est fourni)
  - `pwd` (affiche le répertoire de travail courant)
  - `exit`
- Gestion d'erreurs sans crash : commande invalide, ligne vide, `cd` sans argument ni `$HOME` disponible, `cd` vers un chemin invalide

## Ce que ce projet m'a appris

**Ownership et lifetimes en pratique**
La première version de la fonction de lecture retournait des `&str` empruntés à une `String` locale à la fonction — code qui ne compile pas, puisque ces références auraient survécu à la destruction de la `String` dont elles dépendaient (_dangling reference_). Solution : convertir chaque mot en `String` possédée avant de le faire sortir de la fonction, pour que le `Vec<String>` retourné soit totalement indépendant.

**Gestion d'erreurs idiomatique**

- `?` pour propager les erreurs de lecture (`io::Result`)
- `let ... else` pour gérer un pattern réfutable — aussi bien sur un `Option` (`split_first()` sur une ligne vide, `home_dir()` absent) que sur un `Result` (`let Ok(()) = set_current_dir(path) else { ... }`)
- `match` pour distinguer succès et échec, sans jamais paniquer sur une entrée invalide
- `if let Err(e) = ...` comme forme idiomatique quand seul le cas d'erreur nécessite une action

**Pourquoi `cd` et `pwd` ne peuvent pas être des commandes externes**
`cd` doit modifier le répertoire de travail du shell lui-même, pas celui d'un sous-processus qui se termine immédiatement après. `pwd`, lui, _existe_ en tant qu'exécutable système sur la plupart des distributions Linux — ce qui a permis de repérer un bug concret : sans `continue` après le built-in, la commande s'exécutait deux fois (une fois via le built-in, une fois via le vrai exécutable `pwd` lancé juste après), avec le même résultat affiché en double. Bon rappel que "ça marche" ne veut pas dire "c'est correct".

**Un mini-parseur avec état (guillemets)**
`split_whitespace()` ne suffisait plus pour gérer les guillemets. Écriture d'un parseur caractère par caractère (`.chars()`) avec un état booléen (`is_quote`) : un espace termine un token seulement s'il est hors guillemets, sinon il est ajouté au token en cours. Point d'attention découvert en cours de route : le dernier token de la ligne n'est jamais "fermé" par un espace, donc il faut le pousser explicitement après la boucle — avec une vérification `!token.is_empty()` pour ne pas pousser un token vide en fin de ligne.

**`Command::new()` et `AsRef`**
Fonctionne directement avec `&String` et `&[String]` grâce au trait `AsRef<OsStr>`, sans conversion manuelle nécessaire.

## Lancer le projet

```bash
cargo run
```

## Pistes d'amélioration (à venir)

- Détecter et signaler un guillemet non fermé en fin de ligne
- Distinguer l'échec de lancement d'une commande de son code de sortie non-nul
- Support des variables d'environnement dans les commandes (`$HOME`, etc.)
