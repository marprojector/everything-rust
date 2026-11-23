---
title: Serveurs
description: Les mêmes commandes prouvent les serveurs HTTP. --http, --fresh, --arg et --pace, et une porte d'entrée axum qui sert de proxy pour chaque route à laquelle aucune unité ne répond.
---

# Serveurs

Les mêmes commandes prouvent les serveurs HTTP. `--http` indique que la commande démarre un serveur ; il prend
son port dans `PORT` ou dans `{port}` au sein de la commande (`--port` pour un serveur qui écoute toujours sur le même
port) :

```sh
everything-rust app record . --http -- node server.js
everything-rust app replay .
everything-rust app front-door .                                    # axum : chaque route passe par le proxy
everything-rust app wire . api --route 'GET /users/{id}' --route '*'
everything-rust app lift .
```

- Les requêtes viennent des lignes `curl` de la documentation de l'application (`curl -X POST -d ... http://localhost:8080/x`),
  de `--inputs` (`{"name", "method", "path", "headers", "body"}`) et de pièges : segments de chemin et valeurs de requête
  remplacés par les mêmes pièges que les arguments de CLI, corps remplacés par des pièges texte et JSON, et une
  route inconnue. Chaque exécution démarre le serveur à neuf et envoie les requêtes dans l'ordre des noms, donc l'état
  entre les requêtes est conservé.
- Un serveur qui garde son état dans des fichiers reçoit une copie neuve à chaque démarrage : `--fresh db.json` (un fichier
  ou un dossier, répétable) est capturé dans `http/files/` et copié dans un nouveau répertoire de travail
  à chaque démarrage de l'original, de la porte d'entrée ou d'un build ; `--arg db.json` est un
  argument que chaque démarrage reçoit après sa commande, donc le binaire Rust est démarré comme l'original.
  `--pace 50` attend 50 ms après chaque réponse, pour le travail qu'un serveur fait après avoir répondu (un observateur
  de fichiers qui recharge ce qu'il vient d'écrire).
- Les cas sont `rust/app-cases/http/<case>.toml` (la requête, le statut et les en-têtes qui comptent)
  et `<case>.body` (le corps de la réponse, avec `[..]` là où deux exécutions ont différé). Les en-têtes Date, Server et
  autres en-têtes propres à chaque réponse sont omis ; supprimez toute ligne d'en-tête qui ne vous intéresse pas.
- La porte d'entrée est un serveur axum qui démarre l'original sur un port à lui et sert de proxy pour
  chaque route à laquelle aucune unité ne répond. `wire --route` prend des routes axum : `METHOD /path/{param}`, `/path`
  pour n'importe quelle méthode, `*` pour tout le reste ; la glue reçoit la route, ses paramètres et la
  requête. `lift` retire le proxy ; les cas HTTP tournent alors en CI avec `app replay`.
## Un cas réel

[json-server](/docs/real-world/json-server) a eu besoin de `--fresh`, `--arg` et `--pace` : il réécrit sa base de données à chaque écriture, et son observateur de fichiers peut annuler une écriture qui suit de près.
