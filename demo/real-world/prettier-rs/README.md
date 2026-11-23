# prettier-rs

[Prettier](https://github.com/prettier/prettier) 3.6.2 (commit `7a8b05f`) behind a Rust front
door: JavaScript, TypeScript, JSON, GraphQL, YAML, Markdown, CSS, SCSS and Less files whose syntax the port covers are formatted in Rust,
byte-identical to Prettier; every other file goes to Prettier unchanged. The case study is in
[`docs/real-world.md`](../../../docs/real-world.md#prettier).

`everything-rust package app` lays it out as an npm command, `prettier-rs`, that runs this front
door and depends on Prettier 3.6.2 for everything it hands back; it is not on npm
([how](../../../docs/packaging.md#a-whole-cli-front-door)).

Prettier is MIT licensed, Copyright © James Long and contributors; its license is in
[`LICENSE-prettier`](LICENSE-prettier). The Rust in this folder ports Prettier's code: each module
names the Prettier file it ports, and `prettier-js-rs/src/lib.rs` and `doc-printer-rs/src/port.rs`
carry the license notice. No Prettier source, fixture or recorded case is copied here: the
scripts below regenerate them from a Prettier checkout. `doc-printer-rs/src/emoji-regex.txt` is
the pattern of [emoji-regex](https://github.com/mathiasbynens/emoji-regex) (MIT) and
`doc-printer-rs/src/wide.rs` the table of
[get-east-asian-width](https://github.com/sindresorhus/get-east-asian-width) (MIT), the two
packages Prettier's `getStringWidth` uses. `prettier-js-rs/src/graphql.rs` also ports
[graphql-js](https://github.com/graphql/graphql-js) 16.11's lexer and parser (MIT,
[`LICENSE-graphql-js`](LICENSE-graphql-js)), and `prettier-js-rs/src/yaml/` ports
[yaml](https://github.com/eemeli/yaml) 1.10.2's CST parser and document checks (ISC,
[`LICENSE-yaml`](LICENSE-yaml)) and
[yaml-unist-parser](https://github.com/prettier/yaml-unist-parser) 2.0.5 (MIT,
[`LICENSE-yaml-unist-parser`](LICENSE-yaml-unist-parser)), the parsers Prettier uses for those
languages. `prettier-js-rs/src/markdown/` ports remark-parse 8.0.3
([`LICENSE-remark-parse`](LICENSE-remark-parse)), remark-footnotes 2.0.0
([`LICENSE-remark-footnotes`](LICENSE-remark-footnotes)), remark-math 3.0.1
([`LICENSE-remark-math`](LICENSE-remark-math)) and parse-entities 2.0.0
([`LICENSE-parse-entities`](LICENSE-parse-entities)), all MIT, and its `entities.rs` holds the
data of character-entities ([`LICENSE-character-entities`](LICENSE-character-entities)).
`prettier-js-rs/src/css/` ports postcss 8.5.6's tokenizer and parser
([`LICENSE-postcss`](LICENSE-postcss)) with postcss-scss 4.0.9
([`LICENSE-postcss-scss`](LICENSE-postcss-scss)) and postcss-less 6.0.0
([`LICENSE-postcss-less`](LICENSE-postcss-less)), postcss-values-parser 2.0.1
([`LICENSE-postcss-values-parser`](LICENSE-postcss-values-parser)), postcss-selector-parser 2.2.3
([`LICENSE-postcss-selector-parser`](LICENSE-postcss-selector-parser)) and
postcss-media-query-parser 0.2.3
([`LICENSE-postcss-media-query-parser`](LICENSE-postcss-media-query-parser)), all MIT.

| Folder | What it is |
| --- | --- |
| `app` | the front door: `cargo build --release`, then `prettier --stdin-filepath x.js < x.js`; its glue (`src/units/js_format.rs`) resolves Prettier's config, `.editorconfig` and ignore files for the file (`js_format/config.rs`), tries the Rust port, and passes the case to Prettier on anything it does not cover |
| `doc-printer-rs` | `printDocToString` and `getStringWidth` from `src/document/` |
| `prettier-js-rs` | the JavaScript/TypeScript/JSON printer on oxc's ESTree, with comment attachment (`json.rs` holds the JSON parser checks and `printer-estree-json.js`; `graphql.rs` holds graphql-js's lexer and parser, Prettier's generic comment attachment and `printer-graphql.js`; `yaml/` holds yaml's CST parser and document checks, yaml-unist-parser's transform and comment attachment, and `printer-yaml.js`; `markdown/` holds remark-parse with Prettier's plugins and the Markdown printer; `css/` holds postcss with its SCSS and Less parsers, the value, selector and media query parsers, and `printer-postcss.js`); `examples/check.rs` runs every recorded CLI case through it in about a second, `examples/bench.rs` times it |
| `js-format-rs` | Prettier's `format()` (`src/index.js`), bridged and backed by `prettier-js-rs` |
| `scripts` | `gen-inputs.mjs` (the CLI corpus), `gen-config-inputs.mjs` (63 cases with config, `.editorconfig` and ignore files around the input), `gen-json-inputs.mjs` (the JSON corpus), `gen-graphql-inputs.mjs` (the GraphQL corpus), `gen-yaml-inputs.mjs` (the YAML corpus), `gen-markdown-inputs.mjs` (the Markdown corpus), `gen-css-inputs.mjs` (the CSS, SCSS and Less corpus), `fuzz-inputs.mjs` (mutated files, each run through Prettier by `fuzz-worker.mjs` in a child process with a 10 s limit, read by `examples/fuzz.rs`), `harvest-docs.mjs` and `synthetic-docs.mjs` (the doc printer's inputs), `format-inputs.mjs` (the `format()` calls for the cases the port covers), `facts.mjs` and `node-types.mjs` (corpus facts), `gen-id-chars.mjs` (`prettier-js-rs/src/id_chars.rs`, the identifier characters of the clone's `@babel/parser` and `typescript`: `node scripts/gen-id-chars.mjs <prettier clone> > prettier-js-rs/src/id_chars.rs`) |
| `equiv` | checks of the port against Prettier beyond the recorded corpus (differential fuzzing, mutation testing, Doc-level comparison, a bounded-exhaustive doc printer check), their results, and the 355 cases they found (`cases.jsonl`, recorded like `inputs.jsonl`) |
| `bench` | `cli.mjs` (one process per file, Prettier's CLI against the front door) and `format-node.mjs` (Prettier's `format()` in process) |
| `DECISIONS.md`, `MAP.md` | the choices made before porting, and what is ported |

To rebuild it: clone Prettier at `3.6.2`, `corepack yarn install` and `corepack yarn build
--no-minify`, put this folder at `rust/` of the clone, and point each `everything-rust.json`'s
`root` at the clone. Then, from a scratch folder holding a copy of `dist/prettier` as
`node_modules/prettier` (so Prettier's own config does not apply to the cases):

```sh
node scripts/gen-inputs.mjs <prettier clone> <folder with the real repositories> inputs.jsonl
node scripts/gen-config-inputs.mjs config-inputs.jsonl && cat config-inputs.jsonl >> inputs.jsonl
node scripts/gen-json-inputs.mjs <prettier clone> <folder with the real repositories> json-inputs.jsonl && cat json-inputs.jsonl >> inputs.jsonl
node scripts/gen-graphql-inputs.mjs <prettier clone> graphql-inputs.jsonl && cat graphql-inputs.jsonl >> inputs.jsonl
node scripts/gen-yaml-inputs.mjs <prettier clone> <folder with the real repositories> yaml-inputs.jsonl && cat yaml-inputs.jsonl >> inputs.jsonl
node scripts/gen-markdown-inputs.mjs <prettier clone> <folder with the real repositories> md-inputs.jsonl && cat md-inputs.jsonl >> inputs.jsonl
node scripts/gen-css-inputs.mjs <prettier clone> <folder with the real repositories> css-inputs.jsonl && cat css-inputs.jsonl >> inputs.jsonl
everything-rust app record . --name prettier --inputs inputs.jsonl -- node node_modules/prettier/bin/prettier.cjs
everything-rust app front-door .
everything-rust record <clone>/rust/doc-printer-rs --count 0 --inputs doc-inputs.jsonl
everything-rust gate <clone>/rust/doc-printer-rs
everything-rust record <clone>/rust/js-format-rs --count 0 --inputs format-inputs.jsonl
everything-rust gate <clone>/rust/js-format-rs
everything-rust app wire . <clone>/rust/js-format-rs --command prettier
everything-rust app replay . --build rust/app/target/release/prettier
```

The two unit crates were made with `everything-rust bridge`; each `everything-rust.json` names the
unit and the exports kept in JavaScript. `doc-inputs.jsonl` is what `scripts/harvest-docs.mjs`
and `scripts/synthetic-docs.mjs` write. The real repositories are ms `3.0.0-canary.1`, Fuse.js `v7.1.0` and json-server
`v1.0.0-beta.3`. `format-inputs.jsonl` comes from `scripts/format-inputs.mjs` and the list of
case names `prettier-js-rs/examples/check.rs` writes when `CLAIMED=<file>` is set.

The benchmark, from an empty folder so no config applies:

```sh
node bench/cli.mjs <prettier>/bin/prettier.cjs app/target/release/prettier inputs.jsonl names.txt
node bench/format-node.mjs <prettier> inputs.jsonl names.txt 20
cargo run --release --example bench --manifest-path prettier-js-rs/Cargo.toml -- inputs.jsonl names.txt 20
```

`names.txt` lists the case names to time (the ones `check.rs` writes with `CLAIMED`).
