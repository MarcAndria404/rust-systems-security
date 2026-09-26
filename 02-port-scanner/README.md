# Port Scanner

Un scanner de ports TCP en ligne de commande écrit en Rust : teste une plage de ports sur une IP donnée et affiche les ports ouverts.

## Fonctionnalités

- Scan séquentiel d'une plage de ports (`min_port` à `max_port`) sur une IP donnée
- Timeout configurable par tentative de connexion (évite qu'un port filtré ne bloque le scan indéfiniment)
- Interface en ligne de commande avec message d'aide (`--help`)
- Mesure et affichage du temps de scan

## Usage

```bash
cargo run -- <ip> <min_port> <max_port>
cargo run -- 127.0.0.1 1 1024
cargo run -- --help
```

## Ce que ce projet m'a appris

**Tester un port ouvert = tester une connexion TCP**
Le principe de base : tenter d'établir une connexion TCP vers `ip:port` avec `TcpStream::connect`. Si ça réussit, le port est ouvert ; si la connexion est refusée, il est fermé.

**`connect` vs `connect_timeout`**
`TcpStream::connect` seul n'a pas de limite de temps explicite — sur une IP distante dont les paquets seraient silencieusement ignorés (port filtré par un pare-feu), chaque tentative pourrait attendre le timeout système par défaut, potentiellement très long. `connect_timeout(&addr, duration)` permet de fixer une limite explicite, indispensable pour qu'un scan sur beaucoup de ports reste utilisable en pratique. Sur `127.0.0.1`, la différence n'est pas flagrante (le système répond "connexion refusée" quasi instantanément) — c'est sur une IP distante avec des ports filtrés que ça change tout.

**Construire un `SocketAddr` sans passer par du texte**
Première approche : reformater une `String` (`format!("{}:{}", ip, port)`) puis la reparser à chaque itération de la boucle — fonctionnel mais peu direct. Version finale : parser l'IP une seule fois en `IpAddr`, puis construire un `SocketAddr::new(ip, port)` directement pour chaque port testé, sans repasser par du texte à chaque tour.

**Lire des arguments CLI avec `std::env::args()`**
Le premier élément retourné par `env::args()` est le nom du programme lui-même, pas un argument fourni par l'utilisateur — donc les vrais arguments commencent à l'index 1. Piège rencontré : vérifier le contenu de `args[1]` dans la même expression logique que la vérification de longueur du `Vec`, sans que l'ordre d'évaluation garantisse que la longueur soit vérifiée en premier — resolu en s'appuyant sur l'évaluation court-circuitée de `||` (si la condition de longueur est vraie, `args[1]` n'est jamais évalué).

**Mesurer un temps d'exécution**
`std::time::Instant::now()` puis `.elapsed()` pour valider concrètement une intuition de performance, plutôt que de la supposer.

## Lancer le projet

```bash
cargo run -- 127.0.0.1 1 1024
```

## Pistes d'amélioration (v2, à venir)

- Parallélisation du scan avec des threads (actuellement séquentiel — port par port)
- Détection du service derrière un port ouvert (banner grabbing)
- Sortie au format JSON/CSV pour scripting

**v2 — Parallélisation avec des threads**
La v1 testait les ports séquentiellement, un par un. La v2 lance un thread par port avec `std::thread::spawn`, ce qui pose une contrainte d'ownership propre à la concurrence : une closure passée à `thread::spawn` doit être `'static`, donc ne peut pas simplement emprunter des variables externes — il faut les capturer par valeur avec `move`. Comme `IpAddr`, `u16` et `Duration` implémentent tous `Copy`, chaque thread reçoit sa propre copie indépendante, sans complexité de partage de données.

Point d'architecture important : les threads doivent être **lancés dans une première boucle** (tous les `spawn`, sans attendre), puis **attendus dans une seconde boucle séparée** (tous les `.join()`). Faire `spawn` puis `.join()` immédiatement dans la même itération annule tout le bénéfice du parallélisme — chaque thread serait attendu avant que le suivant ne démarre, ce qui revient à du séquentiel déguisé.

Autre observation concrète : l'ordre d'affichage des ports ouverts n'est plus garanti (non-déterminisme) — c'est le système d'exploitation qui décide de l'ordre réel d'exécution des threads, pas le programme.

**Limite connue** : lancer un thread par port devient coûteux sur une très large plage (ex: les 65535 ports) — chaque thread OS a un coût mémoire réel. Une v3 avec un pool de threads limité (nombre fixe de workers) serait la suite logique pour scanner de grandes plages efficacement.

**Banner grabbing**
Une fois un port détecté ouvert, lecture de ce que le service renvoie via `TcpStream::read(&mut buffer)` (buffer `[u8; 1024]`), converti en texte lisible avec `String::from_utf8_lossy(&buffer[..n])` — la troncature `[..n]` est essentielle, puisque `n` (le nombre d'octets réellement lus, retourné par `.read()`) est presque toujours plus petit que la taille du buffer.

Ajout indispensable : `stream.set_read_timeout(Some(duration))` avant la lecture. Sans lui, un service qui n'envoie jamais rien (par exemple un serveur HTTP, qui attend une requête du client avant de répondre) ferait attendre le thread indéfiniment. Avec le timeout, ce cas produit une erreur système claire (`EAGAIN`/`Resource temporarily unavailable`), gérée sans crash.

Ça a permis d'observer concrètement une distinction réseau réelle : certains services "parlent en premier" dès la connexion (SSH envoie sa bannière immédiatement), d'autres attendent une requête du client avant de répondre quoi que ce soit (HTTP).
