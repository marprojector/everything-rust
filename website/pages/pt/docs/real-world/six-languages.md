---
title: Seis linguagens
description: O everything-rust executado do princípio ao fim em código real em Go, Ruby, Python, Java e C, com a velocidade por chamada e em lote, e os bugs que as execuções encontraram.
---

# Código real em seis linguagens

O everything-rust foi executado do princípio ao fim (analyze, bridge, record, migração, diff, gate, bench) em código real em
cada linguagem que suporta. O código tinha de estar já nesta máquina: código descarregado da
internet não foi executado. Por isso, os alvos são módulos da biblioteca padrão distribuídos com as
toolchains instaladas e, para Java e C, onde não havia código-fonte de bibliotecas adequado no disco, o próprio JDK
e a glibc servem de oráculo por trás de uma fachada fina.

As migrações, os casos registados e as entradas extra estão em [`demo/real-world`](https://github.com/marprojector/everything-rust/blob/main/demo/real-world/README.md).

| Linguagem | Código | Funções | Casos registados | Resultado |
| --- | --- | ---: | ---: | --- |
| Go | `path` da biblioteca padrão do Go 1.24 | 7 (+1 mantida) | 2,642 (542 das próprias tabelas de teste do Go) | 2,642/2,642, o gate passa |
| Ruby | `shellwords.rb` do Ruby 3.3 | 3 | 1,617 (446 lançam exceção) | 1,617/1,617, o gate passa |
| Python | `colorsys.py` do CPython 3.13 | 6 | 2,400 | 2,400/2,400, o gate passa |
| Java | JDK 21 por trás de uma fachada (`Integer`, `Long`, `Math`, `Double.toString`, `URLEncoder`, `Base64`, `toUpperCase`) | 11 | 4,492 (226 lançam exceção) | 4,492/4,492, o gate passa |
| C | glibc por trás de uma fachada (`strtol`, `atoi`, `strtod`, `strspn`, `strcmp`, ctype) | 9 | 1,860 (6 crashes) | 1,859/1,859 depois de retirar um caso de comportamento indefinido, o gate passa |
| C | aklomp/base64 (incluído no runtime do mypyc) | 0 de 7 | - | apenas plano: todas as funções recebem buffers de saída ou uma struct de estado |

## Velocidade

As primeiras bridges para Python, Go, Ruby e Java enviavam cada argumento e resultado como texto
JSON, o que custava 10-50x mais do que o trabalho nestas funções. Foram reconstruídas para converter
os valores diretamente. O `bench` mede agora apenas as chamadas (argumentos descodificados uma vez, no processo) e diz
se vale a pena entregar.

| Linguagem | Por chamada antes | Por chamada agora | Rust vs o original (`bench`, média geométrica) | Veredicto |
| --- | ---: | ---: | --- | --- |
| Ruby | 6.9-9.5 µs (Fiddle + JSON) | 0.36-1.15 µs (extensão magnus) | **4,4x mais rápido** (3.6-7x por função) | entregar |
| Python | 11.7 µs (JSON) | cerca de 0.3 µs (conversão direta PyO3) | **1,35x mais rápido** (1.5-1,9x na maioria das chamadas) | entregar |
| C | - | sem travessia: ligado diretamente | 0,87x: ctype 1.1-1,4x mais rápido, `strtod`, `strspn`, `strcmp` otimizados da glibc 0.5-0,7x | otimizar ou manter as mais lentas |
| Java | cerca de 1.7 µs (JNI + JSON) | 30-800 ns (JNI tipado e protocolo binário) | 0,18x | migrar unidades maiores |
| Go | 2.6 µs (cgo + JSON) | 0.11-0.42 µs (cgo tipado e protocolo binário) | 0,04x | migrar unidades maiores |

Listas, mapas, structs e resultados múltiplos em Go e Java passam por um protocolo binário gerado por
tipo: o `WordFreq` do Go (um mapa) passou de 5.1 para 1.0 µs, o `Midpoint` (uma struct) de 7.2 para 1.0 µs;
o `frequencies` do Java (um `Map`) de 8.4 para 1.0 µs, e o `words` (uma `List<String>`) de 6.0 µs para
0.57 µs, 1,39x mais rápido do que o próprio Java.

TypeScript (núcleo do x18n, nove unidades, 27 funções nos mesmos casos): o protocolo binário reduziu o
custo da bridge em um terço, em média (`rangeContains` 3.6 -> 1.1 µs, `applyTextEdits` 10.4 -> 5.7
µs, `toWireRange` 5.0 -> 2.6 µs), e o `buildTree` passou de empate para 2,0x mais rápido do que o TS. As
outras funções são folhas que o V8 executa em 15-600 ns: continuam mais lentas em Rust, tal como no Go e no Java.

O que decide:

- **Originais interpretados (Python, Ruby)**: a chamada para o Rust custa cerca de 100-300 ns e o
  código Rust é muito mais rápido do que o interpretador, por isso até as funções pequenas ganham.
- **Originais compilados (Go, Java depois do JIT)**: a função folha original demora 4-90 ns; uma única
  chamada cgo custa só por si cerca de 80 ns, e uma chamada JNI com strings mais. Nenhuma bridge consegue tornar mais rápida uma função
  de 5 ns. Estas linguagens só ganham quando uma chamada faz muito trabalho: migre uma função
  juntamente com quem a chama. O `bench` di-lo.
- **C**: não há travessia; o código Rust é comparado diretamente com o código C, e o resultado
  é uma propriedade da migração.

### Lotes: a pesquisa do x18n

As migrações de folhas acima são medidas por chamada. Na pesquisa da barra lateral do x18n (50,000 chaves, escrita letra
a letra), o `everything-rust profile` mostrou o `fuzzyScore` a ocupar cerca de um quarto da execução, chamado uma vez por chave por
`matched`, e sugeriu `--batch fuzzyScore:text`. O `FuzzyScoreBatch` gerado mantém as
chaves em Rust; um `FuzzyScoreIndex` em `port.rs` converte-as para minúsculas em UTF-16 uma única vez e mantém uma máscara de 64 bits
das unidades de código de cada chave, de modo que a maioria das chaves que não podem coincidir é ignorada sem ser lida. Os resultados
são idênticos aos do pipeline TS para cada consulta:

| | ms por consulta | vs TS |
| --- | ---: | ---: |
| Pipeline TS | 10 | 1x |
| TS com o mesmo estreitamento | 4.1-5.4 | 2x |
| Lote, índice predefinido (migração por item, 4 núcleos) | 5.1 | 1,7x |
| Lote com `FuzzyScoreIndex` (máscaras), napi, 4 núcleos | 1.2 | 7x |
| Lote com `FuzzyScoreIndex` a fazer a própria ordenação, napi, 4 núcleos | 0.45-0.56 | 18-22x |
| O mesmo, wasm, 1 thread | 0.95 | 11,5x |

O salto de 7x para 20x: dos 1.2 ms, 0.4 ms eram do próprio `Array.from(order, i =>
keys[i])` do benchmark (um ciclo simples custa 0.02 ms). O resto veio de deixar o índice ordenar por si próprio:
volta a pontuar apenas as correspondências da última consulta quando a nova consulta a estende (uma chave que não
coincide com `s` não pode coincidir com `se`), lê as consultas de uma letra a partir de uma tabela de primeiras posições, pontua
as chaves ASCII como bytes e ordena as pontuações inteiras com um counting sort. O Callgrind mostrou
chamadas de biblioteca (`memchr`, `memcmp`) a custar mais do que ciclos simples em chaves de 30 bytes. Parte do ganho
vem do algoritmo, por isso o benchmark executa também o TS com o mesmo estreitamento: o Rust é 7-12x mais rápido
do que isso (wasm 5x).

O mesmo padrão nas outras linguagens, face aos originais (4 núcleos, resultados e
erros idênticos):

| Linguagem | Carga de trabalho | Original | Migração por chamada vs original | Lote vs original |
| --- | --- | ---: | ---: | ---: |
| Python | fixture `slugify`, 20,000 strings | 98 ms | - | 17x |
| Ruby | fixture `slugify`, 20,000 strings | 110 ms | - | 12,7x |
| Ruby | `Shellwords.shellescape`, 50,000 valores mistos | 88 ms | - | 11,8x |
| Go | fixture `Slugify`, 50,000 strings | 49 ms | 0,93x | 3.3-4,1x |
| Java | fixture `slugify`, 50,000 strings | 43 ms | 0,68x | 3,0x |

O lote em Go começou por ser mais lento do que uma única thread: a migração faz uma alocação por item e o alocador da glibc
serializava as threads dentro do processo Go. Os crates de lote utilizam agora o mimalloc. O lote do shellescape
encontrou também um bug na migração que o registo por chamada não tinha apanhado: os símbolos Ruby (`shellescape(:sym)`) regressavam
como o seu marcador JSON. Corrigido na migração.

O Go e o Java passaram depois de 2,5x e 1,7x para 4x e 3x. Dentro do lote, metade do tempo não era
o trabalho: os resultados eram unidos e libertados numa única thread, o Go copiava-os uma segunda vez, e o Java
fazia uma chamada JNI por string através de UTF-8 modificado. Os resultados string regressam agora em bloco, escritos
em todos os núcleos no buffer do próprio Go ou num único `char[]` do Java. A outra metade eram as migrações: numa única
thread, o `slugify` em Rust era pouco mais rápido do que o do Go e mais lento do que o JIT do Java. Converter ASCII para minúsculas
sem consultas a tabelas e sem uma string intermédia resolveu isso; a migração para Java mantém a
regra do sigma final convertendo para minúsculas apenas o fragmento delimitado por espaços que contém um `Σ`, como um todo.
Por chamada, a migração para Go passou de 0,73x para 0,93x, e a de Java de 0,46x para 0,68x.

## O que as execuções encontraram

### Bugs no everything-rust, corrigidos

- **Floats alterados em um ULP** (execução Java). O parser de floats predefinido do `serde_json` nem sempre
  arredonda corretamente. O everything-rust alterava as entradas registadas ao lê-las, e os crates gerados
  voltavam a alterá-las. 43 casos de `Double.toString` falhavam para uma migração correta. Corrigido com a
  feature `float_roundtrip` do `serde_json` no everything-rust e em cada crate que gera (napi-rs,
  wasm e o dispatcher JSON).
- **Os crashes em C interrompiam o registo** (execução C). Entradas fora do contrato de uma função C (`isalpha(INT_MAX)`)
  provocam segfault na glibc. O driver C executa agora cada caso num processo filho (fork) e regista
  `crash: SIGSEGV`; o `diff` aceita qualquer comportamento da migração nesse caso, já que o do C é indefinido.
- **`module_function :name` não detetado** (execução Ruby). O `shellwords` declara as suas funções de módulo
  com argumentos símbolo depois dos métodos; o exportador só conhecia a forma `module_function`
  simples. Ambas as formas são agora encontradas.
- **Código sem tipos recebia entradas sem sentido** (execução Python). O `colorsys` não tem type hints, por isso 1,709 de
  1,800 casos gerados eram `TypeError`s. Os parâmetros sem anotação utilizados como números (`r - g`,
  `max(r, g, b)`, `x < 1`) são agora tipados como `number`, em Python e Ruby: restam 94 erros, todos reais
  (`ZeroDivisionError`).
- **Inteiros passados como floats nunca eram registados** (execução Python). Quem chama em Python passa `1` com tanta frequência como
  `1.0`, e o `colorsys` deixa passar inteiros (`hsv_to_rgb(0, 0, 7)` é `(7, 7, 7)`). O registo passa agora
  números inteiros como ints em cada quarto caso Python; isso expôs 134 divergências numa
  migração tipada como `f64`.
- **Mensagens dos tempos do TypeScript**: "must stay in JS", "kept in TS", "characterization tests
  against the TS code" indicam agora a linguagem de origem.
- **Classes do núcleo reabertas** (`class String` em `shellwords.rb`) são agora um risco de monkeypatching.

### Comportamento que uma migração tem de reproduzir, encontrado pelo `diff`

Cada linha foi uma divergência numa primeira migração, detetada pelos casos registados. As linhas marcadas com *fixture*
vieram dos pequenos projetos de teste em `tests/fixtures/`, escritos para cada linguagem; as restantes das
execuções reais acima.

- Go `path.Clean`: o `lazybuf` do Go lê o byte no índice de escrita depois de recuar, não o
  último byte mantido (9 casos, da própria tabela de teste do Go).
- *fixture* `strings.ToLower` do Go e `downcase` do Ruby convertem cada rune isoladamente: `ΣΊΣΥΦΟΣ` torna-se `σίσυφοσ`,
  enquanto o `to_lowercase` do Rust aplica a regra do sigma final. O Go mapeia também `İ` para `i`, enquanto o Rust
  dá `i̇`.
- *fixture* O Go corta strings por bytes; um corte a meio de uma rune chega ao JSON como um U+FFFD por byte.
- *fixture* O `slugify` do Ruby lançava `NoMethodError` para um `max` negativo (`slug[0, -1]`
  é `nil`): um bug no original que a migração tem de manter, ou então quem chama tem de ser corrigido primeiro.
- *fixture* O `Array#sum` do Ruby utiliza a soma de Kahan-Babuska e começa no inteiro 0, por isso `[-0.0].sum` é `0.0`.
- O `Float#to_s` do Ruby utiliza a forma exponencial a partir de 1e15 (`1e15.to_s` é `1.0e+15`, `1e14.to_s` é
  `100000000000000.0`); `nil.inspect` dentro de arrays é `nil`.
- *fixture* O `\s` do Java é apenas ASCII; o `split_whitespace` do Rust também divide em U+00A0.
- Resultados int-ou-float do Python: o `colorsys` mantém ints através de `+`, `-` e `*`, por isso a migração fiel
  calcula com um pequeno tipo int-ou-float em vez de `f64` (134 casos assim que os ints passaram a ser registados).
- O `strtod("-")` da glibc não converte nada e devolve `+0.0`, não `-0.0`.

Tratado na primeira migração e confirmado pelos casos registados:

- O `Integer.parseInt` do Java aceita qualquer dígito decimal Unicode (`١٢`, `２`) através de `Character.digit`.
- O `Double.toString` do Java (JDK 19+) escreve o decimal mais curto, mas nunca com menos de dois dígitos:
  `4.9E-324`, enquanto a forma mais curta do Rust é `5e-324`.
- O `max(nan, 1, 2)` do Python é `nan` (o primeiro valor ganha, a menos que um posterior seja maior na comparação);
  `-0.0 % 1.0` é `0.0`.
- O `toupper(-128)` da glibc é `128` (as suas tabelas aceitam valores `char` com sinal); o `strtol` com base 0
  lê `077` como octal e `0x` sem dígito hexadecimal como `0`.

### Limites que estas execuções mostraram

- **APIs C que passam buffers.** As funções públicas do aklomp/base64 recebem `char *out, size_t *outlen`
  ou um `struct base64_state *`. A bridge C transporta apenas escalares e `const char *`, por isso mantém
  todas em C, com o motivo. Suportar o padrão de buffer de saída é o próximo passo para o C.
- **Macros C.** 43 dos 51 ficheiros do base64 definem funções através de macros
  (`BASE64_ENC_FUNCTION(arch)`); o parser assinala erros aí e o plano marca essas unidades como
  parciais. Os intrínsecos SIMD ainda não são assinalados como risco.
- **Comportamento indefinido que não provoca crash.** O `ispunct(1000)` lê para lá da tabela da glibc e
  devolve 1. Nenhuma migração deve copiar isso; o único caso foi removido manualmente de `cases.jsonl`. Ainda
  não existe uma forma integrada de marcar um caso como indefinido.
- **A tipagem dinâmica custa código.** A migração do `shellwords` reimplementa o `to_s` e o `inspect` do Ruby
  para números, nil, arrays e hashes, porque o `shellescape` aceita qualquer coisa. A migração do `colorsys`
  precisa da torre numérica do Python. Uma migração tipada é mais curta, mas altera o comportamento para alguns
  chamadores; o `diff` mostra exatamente quais.
- **As fachadas.** As execuções Java e C testam o comportamento real do JDK e da glibc, mas através de fachadas
  escritas para a execução, não através da API própria de uma biblioteca.

## Como corre cada linguagem

| | Record | Diff | Bridge | Shadow |
| --- | --- | --- | --- | --- |
| Go | driver gerado, compilado como módulo próprio com um `replace` para o original | o mesmo driver sobre o wrapper cgo | dispatcher JSON + pacote cgo com a API original | pacote com a API original, executa ambos |
| Ruby | o driver gerado carrega os ficheiros originais | o mesmo driver carrega o wrapper Fiddle | dispatcher JSON + wrapper Fiddle que define os mesmos módulos | ficheiro a fazer require depois do original: faz `prepend` de ambos |
| Java | driver de reflexão compilado com o código-fonte original | o mesmo driver compilado com as classes wrapper JNI | dispatcher JSON sobre JNI + classes wrapper com os métodos estáticos originais | classes `<Class>Shadow` |
| Python | driver no processo importa o original | o mesmo driver importa o pacote PyO3 | PyO3 | módulo com a API original |
| C | `cc -shared` do original, chamado através de ctypes, um fork por caso | ctypes sobre o `cdylib` Rust | funções `extern "C"` com os nomes originais e um header | `.c` de shadow com renomeações `-D`; a feature `shadow` do crate exporta apenas nomes `everything_rust_` |
