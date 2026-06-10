# astro-core

Rust library and CLI for **Western numerology** and **sun sign** calculations ? the math layer behind [Digital Oracle](https://github.com/YehhiAleksandra/digital-oracle).

No LLM, no network. Pure deterministic logic you can embed in bots, Mini Apps, or WASM.

## Features

- Parse birth dates (`DD.MM.YYYY`)
- Life path number (master numbers 11 / 22 / 33)
- Personal year for any calendar year
- Western zodiac sun sign (Russian labels)

## CLI

```bash
cargo run -- numerology 17.05.1994 --year 2026
cargo run -- zodiac 17.05.1994 --json
```

## Library

```rust
use astro_core::{parse_date, profile, sun_sign};

let birth = parse_date("17.05.1994")?;
let nums = profile(birth, 2026);
let sign = sun_sign(birth.day, birth.month);
```

## Stack

- Rust 2021
- `clap` CLI, `serde` JSON output
- GitHub Actions CI (test + build)

## Related

- [zinaida-mini-app](https://github.com/YehhiAleksandra/zinaida-mini-app) ? Telegram Mini App UI
- [digital-oracle](https://github.com/YehhiAleksandra/digital-oracle) ? Telegram bot

## License

MIT ? see [LICENSE](LICENSE).
