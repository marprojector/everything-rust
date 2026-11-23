---
title: Perguntas frequentes
description: Reescreve o meu código automaticamente? O Rust é sempre mais rápido? Os meus floats vão coincidir? Respostas a perguntas comuns.
---

# Perguntas frequentes

Perguntas que as pessoas fazem sobre o everything-rust. Voltar ao [README](/).

## Perguntas frequentes

**Reescreve o meu código automaticamente?** Não. Encontra o que migrar, gera a bridge,
regista o que o original faz e compara o Rust com isso. Você ou o seu agente escrevem o Rust
em `port.rs`; `diff` e `gate` decidem quando está correto.

**A API da minha aplicação vai mudar?** Não. Quem a chama importa as mesmas funções (ou o módulo shadow, que
executa ambas e devolve o resultado do original enquanto ganha confiança).

**O Rust é sempre mais rápido?** Não, e o everything-rust di-lo. Uma pequena função chamada uma vez por item
é muitas vezes mais lenta em Rust, porque cada chamada atravessa uma bridge; o V8, a JVM e o Go executam essas funções
em nanossegundos. O `profile` encontra o ciclo à volta dela, o `--batch` migra o ciclo inteiro e o `bench`
dá um veredicto antes de publicar. O código já compilado por numba, Cython ou JAX também é executado como código
máquina: o `profile` e o `analyze` assinalam-no, pois migrá-lo traz pouco ganho de velocidade.

**O meu código é numérico; os floats vão coincidir exatamente?** Muitas vezes não: a ordem das operações, o fused
multiply-add e a biblioteca matemática diferem entre compiladores. Registe com `--float-tolerance 1e-9`
(`record` ou `app record`), e os floats coincidem dentro dessa fração do maior valor (absoluta abaixo
de 1). Inteiros, texto e estrutura continuam a ter de coincidir exatamente.

**Consegue migrar uma aplicação que só tenho como APK, JAR ou .exe?** Não. Migra código-fonte: analisa
as funções, regista o que cada uma faz e compara o Rust com o original em execução. Uma aplicação
compilada não oferece nada disso, e o `analyze` di-lo. Escrever uma nova aplicação em Rust
a partir do comportamento visível de um binário é outro projeto.

**Devo migrar C testado em batalha?** Muitas vezes não, e o `analyze` diz-lhe onde. C que acede a
registos de hardware, depende dos tamanhos de tipos ou da disposição de memória do seu compilador, ou utiliza assembly inline
(`volatile`, bitfields, structs compactadas, `__asm__`, inteiros convertidos em endereços) é marcado "manter em
C": uma gravação na sua máquina não prova nada sobre o alvo, e código estável de drivers ou firmware
ganha pouco com uma reescrita. A lógica portável à volta dele (parsing, protocolos, tratamento de dados)
é o que se migra bem.

**Consegue migrar a minha aplicação Electron?** A lógica, sim; as janelas continuam a ser páginas web. O `analyze
--whole-app` deteta o Electron e planeia o processo principal peça a peça, enquanto o renderer fica como
está. Mover a interface para Tauri (a mesma interface web sobre um backend Rust) é um passo separado depois de a lógica
estar em Rust.

**E C#, Kotlin ou Swift?** O C# está em pré-visualização (consulte [Linguagens](/docs/languages/overview)); Kotlin e
Swift ainda não são suportados. Cada linguagem precisa de um parser, um gravador, uma bridge e um driver de
reprodução, todos testados de ponta a ponta. Abra um issue (ou vote num existente) para a linguagem de que precisa.

**O Rust não vai ficar muito maior do que o original?** Não deveria. O `gate` compara as linhas da migração
com as do original e falha acima de 3x (mais 50 linhas para imports e assinaturas): isso normalmente significa escrever à mão o que uma crate já
faz, ou tratar casos um a um. O `analyze` lista as crates a utilizar.

**Migrar peça a peça não prende o Rust ao design da aplicação antiga?** Só enquanto ambos correm
lado a lado. As gravações verificam o comportamento nas fronteiras da aplicação (a saída de uma CLI, as respostas
de um servidor, as funções exportadas de um módulo), não os seus detalhes internos. Espelhe a estrutura antiga enquanto
as unidades ainda atravessam a bridge; depois de o `app lift` ter removido o original, reestruture o Rust
livremente, e o `app replay` continua a provar o comportamento. Nada chama de volta o original:
a porta de entrada executa-o como um processo separado (ou encaminha-lhe o HTTP), as unidades com bridge são chamadas
pelo original e nunca ao contrário, e os callbacks ficam na linguagem de origem.

**O meu código sai da minha máquina?** Não: o everything-rust é executado localmente e não envia o seu código
para lado nenhum. O Cargo descarrega crates como habitualmente, e o fornecedor do modelo do seu agente vê o que o seu agente
lhe envia, segundo a sua própria política.

**O que precisa de ter instalado?** Rust, mais o runtime da linguagem que migra (consulte
[Instalação](/docs/introduction/install)).

**Algo deixou de funcionar.** Abra um issue com o comando, a respetiva saída e a sua plataforma.
