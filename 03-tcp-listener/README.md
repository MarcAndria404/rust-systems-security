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
