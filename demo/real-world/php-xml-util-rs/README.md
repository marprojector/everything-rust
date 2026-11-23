# xml-util-rs

Rust port of seven functions of PEAR's [XML_Util](https://pear.php.net/package/XML_Util) 1.4.5
(New BSD License, Stephan Schmidt), reached through `original/xml_util.php`, a facade of
top-level functions written for this run. Bridged with the JSON dispatcher over PHP's `ext-ffi`.

- `replace_entities`, `reverse_entities` (`XML_Util::replaceEntities`, `reverseEntities`, modes
  0-3 and others, default encoding ISO-8859-1)
- `attributes_to_string` (`XML_Util::attributesToString` with boolean `$sort`)
- `split_qualified_name`, `create_comment`, `create_cdata_section`, `create_end_element`

## Setup

The project the cases were recorded from: `original/composer.json` at its root,
`original/xml_util.php` as `src/xml_util.php`, and `XML/Util.php` from a PEAR install (Homebrew's
PHP has it at `$(brew --prefix)/share/pear/XML/Util.php`) as `src/XML/Util.php`. The facade requires
it from there. The bridged unit is `src/`. Point `root` in `everything-rust.json` at that project,
then:

```sh
everything-rust record . --inputs inputs.jsonl   # or keep the recorded cases.jsonl
everything-rust build .
everything-rust gate .
everything-rust bench .
```

`inputs.jsonl` holds the strings and attribute arrays of XML_Util's own tests in every entity
mode, plus numeric entities, mixed numeric and string keys, and floats PHP prints in exponent form.
