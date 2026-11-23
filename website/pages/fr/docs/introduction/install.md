---
title: Installation
description: Installez everything-rust depuis crates.io, npm ou un binaire de release, ainsi que les runtimes dont chaque langage a besoin.
---

# Installation

## Depuis crates.io, npm ou une release

::: tabs
== cargo
```sh
cargo install everything-rust
everything-rust setup
```
== npm
```sh
npm install -g everything-rust
everything-rust setup
```
:::

npm installe le même binaire, pour les projets JavaScript.

Des binaires précompilés pour macOS, Linux et Windows sont joints à chaque [release GitHub](https://github.com/marprojector/everything-rust/releases). Dans Claude Code, vous pouvez aussi l'installer comme plugin (le serveur MCP et le skill ; la CLI doit être dans votre `PATH`) :

```text
/plugin install everything-rust --marketplace marprojector/everything-rust
```

## Depuis les sources

```sh
cargo install --path .
```

`record`, `diff` et `bench` exécutent l'original dans son propre runtime : Node 22.6+ pour TS (il exécute
directement les sources `.ts`), Python 3.9+ pour Python et C (ctypes), Go 1.21+, Ruby 3+, JDK 17+
(`javac` et `java`). `profile` sous Node exige Node 24+, qui autorise `--cpu-prof` dans `NODE_OPTIONS`. C nécessite aussi un compilateur C. Les bridges wasm nécessitent en plus `rustup target add wasm32-unknown-unknown` et
`cargo install wasm-bindgen-cli --version <the crate's wasm-bindgen version> --locked` ; `build` indique
la version exacte.

## Windows

La CI Windows native utilise la cible Rust `x86_64-pc-windows-msvc`. Installez la toolchain MSVC
stable et les Visual Studio Build Tools avec la charge de travail **Desktop development with C++** ; le
compilateur C et l'éditeur de liens sont requis pour les bridges C et les extensions Rust natives. Installez le runtime
du langage source que vous migrez. Python peut être disponible sous le nom `python3` ou
`python` ; everything-rust détecte les deux. La cible Windows GNU n'est pas testée par la CI.

Lancez la suite d'intégration des bridges natifs depuis une invite Developer PowerShell :

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo test --locked --test native -- --nocapture
```

La suite compile et charge de vraies extensions Node et Python sous forme de DLL, ainsi que les bridges
natifs pris en charge pour chaque runtime présent dans le `PATH`. Les runtimes optionnels absents sont signalés comme ignorés ;
Cargo, la bibliothèque standard Rust active et l'éditeur de liens MSVC sont requis. Si Cargo indique
qu'il ne trouve pas `std`, réinstallez la cible active avec
`rustup target remove x86_64-pc-windows-msvc` suivi de
`rustup target add x86_64-pc-windows-msvc`. Si l'édition de liens échoue, lancez depuis un Developer
PowerShell de Visual Studio et vérifiez que `where.exe link` trouve l'éditeur de liens MSVC. Les tests utilisent des dossiers temporaires,
dont un chemin contenant des espaces et des caractères Unicode, et suppriment les artefacts générés après une
exécution réussie.

Pour les agents, `everything-rust setup` enregistre tout (voir Démarrage rapide). Pour ajouter plutôt le serveur MCP à la main
dans un seul projet (`.mcp.json`) :

```json
{ "mcpServers": { "everything-rust": { "command": "everything-rust", "args": ["mcp"] } } }
```

Chaque outil, ses paramètres et ce qu'il peut toucher : [docs/tools.md](/docs/reference/mcp-tools). Le serveur
propose aussi deux prompts, `port_unit` et `port_whole_app`, construits à partir du skill.
## Mettre à jour ou désinstaller

```sh
cargo install everything-rust                                    # mise à jour (Cargo)
npm install -g everything-rust@latest                            # mise à jour (npm)
everything-rust setup                                            # rafraîchir les réglages des agents
everything-rust setup --remove && cargo uninstall everything-rust  # désinstallation (Cargo)
everything-rust setup --remove && npm uninstall -g everything-rust # désinstallation (npm)
```

`setup --remove` ne retire que les entrées de everything-rust ; les copies enregistrées
(`<file>.everything-rust.bak`) restent jusqu'à ce que vous les supprimiez.
