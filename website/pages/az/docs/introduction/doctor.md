---
title: Doctor
description: everything-rust doctor dil runtime-larını, agent quraşdırmasını və MCP serveri yoxlayır, hər problemi düzəldən əmrlə birlikdə.
---

# Doctor

```sh
everything-rust doctor
everything-rust doctor --json
```

`doctor` dil runtime-larını, agent quraşdırmasını və MCP serveri yoxlayır və hər problemi düzəldən əmri çap edir. Tələb olunan nəsə çatışmayanda 1 kodu ilə çıxır.

Addım runtime, agent quraşdırması və ya MCP server çatışmadığı və ya köhnəldiyi üçün uğursuz olanda onu işə salın. İş axını skill-i agentlərə də eyni şeyi etməyi deyir.

## Hər dilə nə lazımdır

`record`, `diff` və `bench` orijinalı öz runtime-ında işə salır: TS üçün Node 22.6+ (`.ts` mənbələrini birbaşa işə salır), Python və C (ctypes) üçün Python 3.9+, Go 1.21+, Ruby 3+, JDK 17+ (`javac` və `java`). Node-da `profile` üçün `NODE_OPTIONS`-da `--cpu-prof`-a icazə verən Node 24+ lazımdır. C üçün həmçinin C kompilyatoru lazımdır. wasm körpüləri üçün həmçinin `rustup target add wasm32-unknown-unknown` və uyğun `wasm-bindgen-cli` lazımdır; dəqiq versiyanı `build` göstərir.

## Töhfə verənlər üçün

`cargo run -- doctor` hansı runtime-larınızın olduğunu, onların versiyalarını və hər birini quraşdıran və ya yeniləyən əmri siyahıya alır. Bax [Töhfə vermək](/docs/project/contributing).
