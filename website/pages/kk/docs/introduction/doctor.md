---
title: Doctor
description: everything-rust doctor тіл runtime-дарын, агент баптауын және MCP серверді тексеріп, әр мәселені түзететін команданы көрсетеді.
---

# Doctor

```sh
everything-rust doctor
everything-rust doctor --json
```

`doctor` тіл runtime-дарын, агент баптауын және MCP серверді тексеріп, әр мәселені түзететін команданы шығарады. Қажетті бірдеңе жетіспесе, ол 1 кодымен аяқталады.

Runtime, агент баптауы немесе MCP сервер жоқ не ескірген болғандықтан бір қадам құласа, осыны іске қосыңыз. Workflow skill агенттерге де дәл солай істеуді айтады.

## Әр тілге не керек

`record`, `diff` және `bench` түпнұсқаны өз runtime-ында іске қосады: TS үшін Node 22.6+ (ол `.ts` бастапқы файлдарын тікелей іске қосады), Python мен C үшін Python 3.9+ (ctypes), Go 1.21+, Ruby 3+, JDK 17+ (`javac` және `java`). Node-та `profile` үшін Node 24+ керек, ол `NODE_OPTIONS` ішінде `--cpu-prof` рұқсат етеді. C үшін C компиляторы да керек. wasm bridge-тері үшін тағы `rustup target add wasm32-unknown-unknown` және сәйкес `wasm-bindgen-cli` керек; нақты нұсқасын `build` атайды.

## Үлес қосушыларға

`cargo run -- doctor` қандай runtime-дарыңыз бар екенін, олардың нұсқаларын және әрқайсысын орнататын не жаңартатын команданы тізімдейді. [Үлес қосу](/docs/project/contributing) бетін қараңыз.
