# TCP Client/Serveur

Un serveur TCP multi-clients écrit en Rust, capable d'accepter plusieurs connexions simultanées et d'échanger des messages avec chacune.

## Fonctionnalités (v1)

- Serveur TCP écoutant en continu sur un port (`127.0.0.1:7878`)
- Une connexion par thread : chaque client est géré indépendamment, sans bloquer les autres
- Lecture du message envoyé par le client, réponse fixe en retour

## Usage

```bash
cargo run
# dans un autre terminal :
nc 127.0.0.1 7878
```

## Ce que ce projet m'a appris

**`TcpListener` vs `TcpStream`**
Jusqu'ici (scanner de ports), j'étais toujours côté client (`TcpStream::connect`). Ici, le serveur utilise `TcpListener::bind(...)` pour réserver un port et écouter dessus, puis `.incoming()` pour obtenir un flux continu de connexions entrantes — chacune donnant, comme côté client, un `TcpStream` sur lequel lire/écrire.

**`.accept()` vs `.incoming()`**
`.accept()` traite une seule connexion et s'arrête. `.incoming()` retourne un itérateur infini de connexions (`io::Result<TcpStream>` à chaque itération), adapté à un serveur qui doit tourner en continu et accepter un nombre indéterminé de clients.

**Un thread par connexion**
Chaque connexion acceptée est déléguée à un thread dédié (`thread::spawn(move || { ... })`), pour qu'un client lent (ou une longue conversation) ne bloque jamais les autres. Contrairement au scanner (où on attendait tous les threads via `.join()`), ici les threads sont volontairement laissés tourner en "fire and forget" — le serveur ne s'arrête jamais tant qu'il tourne.

**`Copy` sur les tableaux de taille fixe**
Point subtil : un buffer `[u8; 1024]` déclaré avant la boucle peut être capturé par `move` dans chaque itération sans erreur d'ownership, alors qu'un `move` ne devrait normalement transférer une valeur qu'une seule fois. Raison : `[T; N]` implémente `Copy` dès lors que `T` l'est (le cas de `u8`) — chaque thread reçoit donc sa **propre copie indépendante** du buffer, pas la même mémoire partagée. Un `Vec<u8>` n'aurait pas eu ce comportement (pas `Copy`), et aurait nécessité un vrai partage de données entre threads.

## Lancer le projet

```bash
cargo run
```

## Pistes d'amélioration (à venir)

- Vrai chat : relayer les messages entre clients connectés (nécessite un état partagé entre threads — `Arc`/`Mutex`)
- Gestion propre de la déconnexion d'un client

**Partage d'état entre threads avec `Arc<Mutex<T>>`**
Pour un vrai chat (diffuser un message à tous les clients connectés), chaque thread a besoin d'accéder à une **même** liste partagée de connexions — contrairement au buffer de lecture (`[u8; 1024]`, `Copy`), qui pouvait être dupliqué sans risque, un `Vec<TcpStream>` doit être **réellement partagé**, pas copié.

- `Arc<T>` ("Atomically Reference Counted") permet à plusieurs threads de partager la possession d'une même donnée — chaque `Arc::clone(&shared)` incrémente un compteur de références et pointe vers la même donnée sous-jacente, sans la dupliquer.
- `Mutex<T>` protège l'accès concurrent : un seul thread à la fois peut modifier la donnée, via `.lock()`, qui bloque jusqu'à obtenir l'accès exclusif et relâche automatiquement le verrou à la fin de son scope.

**`TcpStream::try_clone()` : séparer lecture et écriture**
Un même client doit à la fois pouvoir être lu (dans son propre thread) et recevoir des messages écrits par d'autres threads (diffusion). `try_clone()` donne un second descripteur pointant vers la même connexion réseau sous-jacente — le stream d'origine reste dédié à la lecture, le clone rejoint la liste partagée pour l'écriture depuis les autres threads.

**Ordre des opérations : rejoindre le groupe avant d'écouter**
Premier essai bugué : le client n'était ajouté à la liste partagée qu'après avoir lui-même envoyé un message — donc invisible pour les autres clients tant qu'il n'avait pas parlé. Correction : cloner le stream et le pousser dans la liste partagée **immédiatement** à la connexion, avant toute lecture.

**Robustesse de la diffusion**
Écrire vers un client déconnecté produit une erreur (`write_all` échoue, ex: "broken pipe"). Un `.unwrap()` sur cette écriture ferait paniquer le thread au premier client mort dans la liste, interrompant la diffusion aux clients suivants. Remplacé par `if let Err(e) = ...` pour logger sans interrompre la boucle.

## Pistes d'amélioration (à venir)

- Exclure l'expéditeur de la diffusion (actuellement il reçoit son propre message en echo)
- Retirer proprement un client de la liste partagée à sa déconnexion (actuellement les streams morts s'accumulent)
- Identifiants/pseudos par client
