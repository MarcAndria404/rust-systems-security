# Rust · Programmation système · Cybersécurité

Parcours d'auto-formation en programmation système et sécurité offensive, construit projet par projet en Rust. L'objectif : comprendre en profondeur comment fonctionnent les processus, la mémoire et le réseau, pour évoluer vers la cybersécurité offensive (pentest / red teaming).

Chaque projet est documenté avec ce qui a été appris, les choix techniques faits, et les difficultés rencontrées — pas seulement le résultat final.

## Pourquoi Rust

Rust force à raisonner explicitement sur la mémoire (ownership, borrowing, lifetimes) tout en donnant accès aux mêmes capacités bas niveau qu'un langage comme C. C'est aussi un langage de plus en plus utilisé dans l'outillage offensif moderne (implants, outils d'évasion), ce qui en fait un choix pertinent pour ce domaine.

## Projets

| #   | Projet                            | Ce qu'il couvre                             | Statut                            |
| --- | --------------------------------- | ------------------------------------------- | --------------------------------- |
| 01  | [Mini-shell](./01-mini-shell)     | Processus, I/O système, ownership appliqué  | Terminé                           |
| 02  | [Port Scanner](./02-port-scanner) | Sockets TCP, timeouts réseau, arguments CLI | v1 terminée, v2 (threads) à venir |

## Compétences développées au fil du parcours

- Ownership, borrowing et lifetimes en situation réelle (pas juste en théorie)
- Gestion d'erreurs idiomatique (`Result`, `Option`, `?`, `match`, `let ... else`)
- Interaction avec l'OS : processus, entrées/sorties, système de fichiers
- _(à compléter au fil des projets suivants : réseau bas niveau, exploitation, outils offensifs)_

## À propos

Étudiant en Bachelor Développement d'application / Cybersécurité, en direction d'un Bachelor DIAC (Développement, IA & Cybersécurité). Parcours autodidacte en parallèle de la formation, orienté vers un stage puis une alternance en cybersécurité.
