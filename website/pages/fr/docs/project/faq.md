---
title: FAQ
description: Réécrit-il mon code automatiquement ? Rust est-il toujours plus rapide ? Mes flottants vont-ils correspondre ? Réponses aux questions fréquentes.
---

# FAQ

Les questions qu'on pose sur everything-rust. Retour au [README](/).

## FAQ

**Réécrit-il mon code automatiquement ?** Non. Il trouve ce qu'il faut déplacer, génère le bridge,
enregistre ce que fait l'original et vérifie le Rust face à cet enregistrement. Vous ou votre agent écrivez le Rust
dans `port.rs` ; `diff` et `gate` décident quand il est correct.

**L'API de mon application va-t-elle changer ?** Non. Les appelants importent les mêmes fonctions (ou le module shadow, qui
exécute les deux et renvoie le résultat de l'original pendant que vous gagnez en confiance).

**Rust est-il toujours plus rapide ?** Non, et everything-rust le dit. Une petite fonction appelée une fois par élément
est souvent plus lente en Rust, car chaque appel traverse un bridge ; V8, la JVM et Go exécutent ce genre de fonction
en quelques nanosecondes. `profile` trouve la boucle qui l'entoure, `--batch` déplace toute la boucle, et `bench`
rend un verdict avant la livraison. Le code déjà compilé par numba, Cython ou JAX s'exécute lui aussi en code
machine : `profile` et `analyze` le signalent, car le porter apporte peu de vitesse.

**Mon code est numérique ; les flottants vont-ils correspondre exactement ?** Souvent non : l'ordre des opérations, le fused
multiply-add et la bibliothèque mathématique diffèrent d'un compilateur à l'autre. Enregistrez avec `--float-tolerance 1e-9`
(`record` ou `app record`), et les flottants correspondent à cette fraction près de la plus grande valeur (en absolu sous
1). Les entiers, le texte et la structure doivent toujours correspondre exactement.

**Peut-il porter une application que je n'ai qu'en APK, JAR ou .exe ?** Non. Il porte du code source : il
analyse les fonctions, enregistre ce que fait chacune, et vérifie le Rust face à l'original en
cours d'exécution. Une application compilée ne lui offre rien de tout cela, et `analyze` le dit. Écrire une nouvelle application Rust
à partir du comportement visible d'un binaire est un autre projet.

**Faut-il porter du C éprouvé ?** Souvent non, et `analyze` vous dit où. Le C qui touche des registres
matériels, dépend de la taille des types ou de la disposition mémoire de son compilateur, ou utilise de l'assembleur en ligne
(`volatile`, champs de bits, structs packées, `__asm__`, entiers convertis en adresses) est marqué « garder en
C » : un enregistrement sur votre machine ne prouve rien sur la cible, et du code de pilote ou de firmware
stable gagne peu à une réécriture. C'est la logique portable qui l'entoure (analyse syntaxique, protocoles, traitement de données)
qui se déplace bien.

**Peut-il porter mon application Electron ?** La logique, oui ; les fenêtres restent des pages web. `analyze
--whole-app` repère Electron et planifie le processus principal morceau par morceau, tandis que le renderer reste tel
quel. Passer l'interface à Tauri (la même interface web sur un backend Rust) est une étape séparée, une fois la logique
en Rust.

**Et C#, Kotlin ou Swift ?** C# est en préversion (voir [Langages](/docs/languages/overview)) ; Kotlin et
Swift ne sont pas encore pris en charge. Chaque langage a besoin d'un analyseur, d'un enregistreur, d'un bridge et d'un pilote
de replay, tous testés de bout en bout. Ouvrez une issue (ou votez pour une existante) pour le langage dont vous avez besoin.

**Le Rust ne sera-t-il pas bien plus gros que l'original ?** Il ne devrait pas. `gate` compare les lignes du portage
à celles de l'original et échoue au-delà de 3x (plus 50 lignes pour les imports et les signatures) : cela signifie en général qu'on réécrit à la main ce qu'une crate fait
déjà, ou qu'on traite les cas un par un. `analyze` liste les crates à utiliser.

**Porter morceau par morceau ne rend-il pas le Rust prisonnier de la conception de l'ancienne application ?** Seulement tant que les deux tournent
côte à côte. Les enregistrements vérifient le comportement aux bords de l'application (la sortie d'une CLI, les réponses
d'un serveur, les fonctions exportées d'un module), pas ses entrailles. Reproduisez l'ancienne structure tant que des
unités traversent encore le bridge ; une fois que `app lift` a retiré l'original, restructurez librement
le Rust, et `app replay` prouve toujours le comportement. Rien ne rappelle l'original non plus :
la porte d'entrée l'exécute comme un processus séparé (ou lui transmet le HTTP), les unités passées par un bridge sont appelées
par l'original et jamais l'inverse, et les callbacks restent dans le langage source.

**Mon code quitte-t-il ma machine ?** Non : everything-rust s'exécute localement et n'envoie votre code
nulle part. Cargo télécharge les crates comme d'habitude, et le fournisseur du modèle de votre agent voit ce que votre agent
lui envoie, selon sa propre politique.

**Que faut-il installer ?** Rust, plus le runtime du langage que vous portez (voir
[Installation](/docs/introduction/install)).

**Quelque chose a cassé.** Ouvrez une issue avec la commande, sa sortie et votre plateforme.
