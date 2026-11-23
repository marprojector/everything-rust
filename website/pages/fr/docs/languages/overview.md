---
title: Langages
description: "TypeScript/JavaScript, Python, C, Go, Ruby et Java, avec PHP et C# en préversion. L'unité, le bridge et l'enregistreur de chacun."
---

# Langages

Six langages passent par toute la boucle : Node.js (TypeScript/JavaScript), Python, C, Go, Ruby et Java. PHP et C# sont en préversion : `analyze` et les applications entières aujourd'hui, les bridges unitaires ensuite.

| Langage | Unité | Bridge | Enregistré et rejoué par |
| --- | --- | --- | --- |
| TypeScript/JavaScript | dossier ou fichier | napi-rs ou wasm-bindgen avec un protocole binaire généré ; classes et async compris | Node, qui exécute les sources `.ts` |
| Python | package ou module | PyO3 (abi3), conversion directe, exceptions typées | Python, qui importe l'original |
| C | fichier `.c` | fonctions `extern "C"` avec les noms d'origine et un en-tête | l'original compilé en bibliothèque partagée, appelé via ctypes, un fork par cas |
| Go | package | appels cgo typés et un protocole binaire généré, dans un package avec l'API d'origine | un pilote Go généré, compilé comme module à part |
| Ruby | fichier ou dossier de fonctions de module | une extension magnus sous les modules d'origine | un pilote Ruby généré |
| Java | package de méthodes `public static` | JNI typé et un protocole binaire généré, dans des classes wrapper avec les méthodes d'origine | un pilote par réflexion généré |
| PHP (préversion) | dossier de fonctions et de méthodes `public static` | à venir : PHP FFI via une ABI C | applications entières aujourd'hui : `app record`, `app replay` et la porte d'entrée, pour les CLI et les serveurs (`php -S`) |
| C# (préversion) | dossier de méthodes `public static` | à venir : P/Invoke via une ABI C | applications entières aujourd'hui : `app record`, `app replay` et la porte d'entrée, pour les CLI et les serveurs ASP.NET (`dotnet <app>.dll`) |

Tous les langages sauf C ont aussi les classes de lot (`--batch`) : les éléments traversent une seule fois et restent en
Rust, et chaque appel les traite tous, sur tous les cœurs.

Le langage est détecté depuis le dépôt (`go.mod`, `pom.xml`, un `.csproj` ou `.sln`,
`pyproject.toml`, `Gemfile`, `composer.json`, `package.json`, sinon les fichiers sources les plus fréquents) ;
`--lang` le remplace. Les types viennent du code source : types TS, annotations Python, signatures C, Go, Java et C#,
tags YARD Ruby, types PHP et phpdoc ; les paramètres non typés utilisés comme des nombres sont déduits
comme des nombres.

PHP est en préversion : `analyze` planifie une application PHP (packages Composer et autoload PSR-4, risques
comme la comparaison souple et le dépassement d'entier vers les flottants, packages Composer mis en correspondance avec des crates),
et une CLI ou un serveur PHP peut être enregistré, rejoué et placé derrière une porte d'entrée Rust. Porter ses
unités en Rust via le FFI de PHP est la prochaine étape ([issue #2](https://github.com/marprojector/everything-rust/issues/2)).

C# est en préversion de la même manière : `analyze` lit les projets `.csproj` et leurs packages NuGet
(mis en correspondance avec des crates), résout les directives `using` en dossiers, et signale ce que C# fait différemment
(`decimal`, `dynamic`, les requêtes différées de LINQ, `Task`, `lock`, la réflexion). Une CLI C# ou un serveur ASP.NET
peut être enregistré, rejoué et placé derrière une porte d'entrée Rust ; le portage des unités via P/Invoke
est la prochaine étape.
## État par langage

| Langage | État | Dernière exécution réelle |
| --- | --- | --- |
| [TypeScript/JavaScript](/docs/languages/typescript) | Vérifié | json-server, Fuse.js, vercel/ms, `packages/core` de x18n |
| [Python](/docs/languages/python) | Vérifié | `colorsys.py` de CPython 3.13 : 2,400/2,400 |
| [Go](/docs/languages/go) | Vérifié | `path` de Go 1.24 : 2,642/2,642 |
| [Ruby](/docs/languages/ruby) | Vérifié | `shellwords.rb` de Ruby 3.3 : 1,617/1,617 |
| [Java](/docs/languages/java) | Vérifié | JDK 21 derrière une façade : 4,492/4,492 |
| [C](/docs/languages/c) | Vérifié | glibc derrière une façade : 1,859/1,859 après retrait d'un cas de comportement indéfini |
| [PHP](/docs/languages/php) | Préversion | `analyze` et applications entières uniquement |
| [C#](/docs/languages/csharp) | Préversion | `analyze` et applications entières uniquement |

Le tableau complet, avec les commandes, se trouve sur la page [État](/docs/project/status).
