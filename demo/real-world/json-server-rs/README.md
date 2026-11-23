# json-server-rs

[json-server](https://github.com/typicode/json-server) (MIT, commit `89a34a4`, 1.0.0-beta.15)
ported to a pure Rust binary with everything-rust's whole-app flow. The case study is in
[`docs/real-world.md`](../../../docs/real-world.md#json-server).

| Folder | What it is |
| --- | --- |
| `app` | the lifted binary: `cargo build --release`, then `json-server db.json [-p 3000] [-s dir]` |
| `app-cases` | 514 HTTP cases recorded from the Node server (`everything-rust app record --http`), with the database and `public/` they start from in `http/files` |
| `app-rs` | `src/app.ts` and `src/bin.ts`: the tinyhttp app as its router runs it, CORS, the body parser, the index page, the database file, the command line |
| `service-rs` | `src/service.ts` |
| `parse-where-rs`, `matches-where-rs`, `paginate-rs`, `where-operators-rs` | the units of the same names |
| `json-server-js` | the JavaScript behavior json-server depends on: V8's `JSON.parse` messages and `JSON.stringify`, comparisons, `Number()`, Node's query-string parsers, dot-prop, sort-on's ICU collation, inflection, sirv and mrmime; `oracle.mjs` records Node's answers to 9,000 inputs into `tests/node-oracle.json`, which `cargo test` checks |
| `adapter` | `service-calls.ts` and `app-requests.ts`, copied to `src/rust/` of the clone so the Service and the app can be bridged and recorded like functions |
| `*-inputs.jsonl` | the extra inputs given to `everything-rust record --inputs` and `app record --inputs` |
| `fixtures/db.json` | the database the HTTP cases start from |
| `bench/bench.py` | the benchmark: Node's `lib/bin.js` against the binary with `ab`, on this database and on a generated one with 10,000 posts |

The unit crates were lifted (bindings removed), so `everything-rust diff` no longer runs on them;
each one's `tests/recorded.rs` replays its `cases.jsonl` on the port instead (`cargo test`). To
replay the HTTP cases against the binary, put this folder at `rust/` of a json-server clone and
run:

```sh
everything-rust app replay . --build rust/app/target/release/json-server
```
