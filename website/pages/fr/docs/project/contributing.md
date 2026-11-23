---
title: Contribuer
description: L'installation, où placer les tests, comment ajouter une correspondance de crate ou un langage, et comment partager un portage réel.
---

# Contribuer

Merci de votre aide. Les rapports de bugs, les corrections, la prise en charge de nouveaux langages et les comptes rendus de portages réels sont tous
les bienvenus, et vous n'avez pas besoin de demander avant d'ouvrir une pull request pour corriger un bug.

## Avant de commencer

- **Un bug :** ouvrez une [issue](https://github.com/marprojector/everything-rust/issues/new/choose) avec
  la commande lancée, sa sortie, votre plateforme et la version de everything-rust
  (`everything-rust --version`). Un tout petit projet de reproduction aide le plus.
- **Un changement plus important** (un nouveau langage, une nouvelle commande, un changement de ce qu'est une unité ou un enregistrement) :
  ouvrez d'abord une issue pour que nous puissions nous mettre d'accord sur la forme avant que vous ne l'écriviez. Les raisons des
  choix passés se trouvent dans [DECISIONS.md](https://github.com/marprojector/everything-rust/blob/main/DECISIONS.md).
- **Un problème de sécurité :** n'ouvrez pas d'issue publique ; voir [SECURITY.md](https://github.com/marprojector/everything-rust/blob/main/SECURITY.md).

## Installation

Il vous faut Rust (stable, 1.97 ou plus récent). Les tests exécutent aussi le langage que gère chaque partie, et
ignorent une partie dont le runtime est absent, donc installez ceux que vous touchez :

| Pour | Installer |
| --- | --- |
| TypeScript/JavaScript | Node 24 (22.6+ fonctionne sauf pour `profile`) |
| Python | Python 3.9+ |
| C | un compilateur C (`cc`) |
| Go | Go 1.22+ |
| Ruby | Ruby 3.x |
| Java | un JDK (17+) |
| PHP | PHP 8 |
| C# | le SDK .NET |
| bridges wasm | `rustup target add wasm32-unknown-unknown` et le `wasm-bindgen-cli` correspondant |

```sh
git clone https://github.com/marprojector/everything-rust
cd everything-rust
git config core.hooksPath .githooks
cargo test
```

`cargo run -- doctor` liste les runtimes dont vous disposez parmi ceux-ci, leurs versions, et la commande
qui installe ou met à jour chacun d'eux. Pour qu'un runtime absent fasse échouer ses tests au lieu de les ignorer
(c'est ce que fait la CI), nommez-le dans `EVERYTHING_RUST_REQUIRE`, par exemple
`EVERYTHING_RUST_REQUIRE=node,python3,go,ruby,java cargo test`.

Le hook exécute `scripts/check-secrets.sh`, qui refuse les clés, les tokens et les chemins de dossier personnel
dans ce que vous commitez. La CI l'exécute sur chaque fichier.

## Faire le changement

- **Chaque correction s'accompagne d'un test qui échoue sans elle.** Lancez une fois le test sur l'ancien code pour
  vous en assurer.
- **Où placer les tests :**
  - `tests/analyze.rs` : détection du langage, unités, risques, dépendances, `--whole-app`.
  - `tests/loop.rs` : bridge, record, diff et shadow par langage, avec des substituts au lieu de builds.
  - `tests/native.rs` : bridges compilés pour de vrai, puis chargés et rejoués.
  - `tests/app.rs` et `tests/http.rs` : record, replay, porte d'entrée et lift d'application entière, pour les CLI et les
    serveurs.
  - `tests/library.rs` : portages de bibliothèques entières.
  - Les tests unitaires vivent à côté du code.
- **Les fixtures** sont de petites applications sous `tests/fixtures/`. Les fixtures CLI et HTTP implémentent les mêmes
  commandes et routes dans chaque langage (`tcli upper|len|div|words|copy|now`, `/hello`, `/upper`,
  `/div/{a}/{b}`, `/now`), donc gardez-en une nouvelle identique aux autres en comportement.
- **Le code généré** (les bridges, pilotes et portes d'entrée qu'écrit everything-rust) doit lui aussi passer
  `clippy -D warnings`, puisque `gate` l'exécute sur les crates des utilisateurs.
- **Style :** alignez-vous sur le code environnant. Gardez les commentaires pour ce que le code ne peut pas dire seul, et
  mettez plutôt le raisonnement derrière un changement dans la pull request.

## Ajouter une correspondance de crate

`analyze` nomme la crate Rust de chaque bibliothèque importée par le code à partir de `src/crates.toml`. Pour en ajouter
une, ajoutez une entrée sous son langage : `names` (noms de packages exacts) ou `prefixes` (le début d'un
nom, pour Java et C#), `crates`, `fit` (`good`, `partial` ou `none`) et une `note` quand l'adéquation
mérite une explication. La première entrée qui correspond l'emporte, donc placez les entrées spécifiques avant les préfixes larges. Une
bibliothèque absente de la table est signalée comme `unknown` ; ne devinez jamais une crate.

## Ajouter un langage

PHP et C# ont été ajoutés en préversion dans deux commits qui valent la peine d'être lus d'abord :
[e8ac243](https://github.com/marprojector/everything-rust/commit/e8ac243) (PHP) et
[caea59a](https://github.com/marprojector/everything-rust/commit/caea59a) (C#). Un nouveau langage commence
de la même manière :

1. Un plugin dans `src/lang/<name>.rs` : découverte, analyse (imports et risques), résolution et exports,
   plus les crates de ses bibliothèques sous forme d'entrées `[[<name>]]` dans `src/crates.toml`.
2. La détection et l'aiguillage dans `src/lang/mod.rs`.
3. La prise en charge des applications entières dans `src/whole_app.rs` : points d'entrée, serveurs, dépendances déclarées.
4. Les fixtures CLI et HTTP, une fixture de bibliothèque, et des tests dans `tests/analyze.rs`, `tests/app.rs`
   et `tests/http.rs`.
5. `bridge` refuse les unités du langage tant qu'aucun bridge n'existe, avec un message clair.

## Traduire le README

Les traductions sont les bienvenues et maintenues par la communauté. Une nouvelle traduction :

1. Se trouve dans `docs/README_<CODE>.md` (par exemple `docs/README_PT.md`) ; le `README.md` racine reste
   en anglais et n'est jamais renommé ni remplacé.
2. Utilise des chemins qui fonctionnent depuis `docs/` : `assets/logo.svg`, `../LICENSE`, `../README.md`.
3. Commence par la même ligne de langues que les autres traductions, avec sa propre langue en texte
   simple, suivie de la note indiquant qu'elle est maintenue par la communauté et peut être en retard sur le README
   anglais (copiez-la depuis `docs/README_DE.md` et traduisez-la).
4. Se place à la **fin** de la ligne de langues, dans `README.md` et dans chaque traduction. L'ordre des
   langues déjà listées ne change pas.

## Site de documentation

Le site de documentation se trouve dans `website/` (Void, `@void/md`, React). Beaucoup de ses pages incluent
des sections de `README.md`, `CONTRIBUTING.md` et `docs/*.md` au moment du build, donc modifiez ces fichiers et le
site suit. Lancez-le avec `cd website && npm ci --ignore-scripts && npm run dev`, et compilez-le avec
`npm run build`. La référence CLI vient d'un instantané de `--help` dans
`website/content/cli-help.json` ; après avoir modifié la CLI, rafraîchissez-le avec `cargo build --release` et
`npm run cli-help` dans `website/`.

## Ouvrir la pull request

Avant de pousser :

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

- Dites ce qui n'allait pas, comment le reproduire, et comment le changement le corrige.
- Liez l'issue (`Fixes #123`).
- Signalez ce que vous n'avez pas pu tester (un autre OS, un runtime que vous n'avez pas).
- Limitez-vous à un seul changement : deux corrections sont plus faciles à relire en deux pull requests.

La CI tourne sur Ubuntu et macOS avec tous les runtimes installés. Pour un premier contributeur, GitHub
attend qu'un mainteneur approuve l'exécution de la CI, donc attendez-vous à un court délai avant le démarrage des vérifications.

Les pull requests sont fusionnées en conservant leurs commits, donc votre historique reste le vôtre. Si un changement nécessite
de petits ajustements, un mainteneur peut ajouter un commit par-dessus plutôt que demander un tour de plus.

## Partager un portage réel

Les portages de vrais projets sont la meilleure preuve de ce qui marche et de ce qui casse. Si vous avez utilisé
everything-rust sur quelque chose de réel, une issue ou une pull request qui l'ajoute à
[docs/real-world.md](/docs/real-world/overview) est très bienvenue. Indiquez le projet, les commandes
lancées, le nombre de cas, et tout ce qui vous a surpris.

## Code de conduite

Chaque participant est tenu de respecter le [code de conduite](https://github.com/marprojector/everything-rust/blob/main/CODE_OF_CONDUCT.md).
