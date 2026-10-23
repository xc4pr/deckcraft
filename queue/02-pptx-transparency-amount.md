# Queue item: issue + PR

- Kind: issue+pr
- Target repo: storytold/deckcraft
- Base branch: main
- Head branch: xc4pr:fix/pptx-transparency-amount
- Head commit: c70db3e9fca4910ed25e558dac15f75d600c4fda
- Tested on: upstream main a58bdbc7 (deckcraft 0.10.0)

## Issue

### Title

PPTX: a Transparency emphasis effect loses its amount on save and reopen

### Body

A Transparency emphasis effect (preset 9) comes back from a .pptx at the default 50 %, whatever amount it had.

**Steps**

1. Give a shape a Transparency emphasis with amount 0.75 (`Animation { class: Emphasis, effect: "transparency", amount: Some(0.75), .. }`).
2. Save as .pptx and open it again.

**Expected:** the effect's amount is 0.75.

**Actual:** the amount is `None`, so the player uses 0.5, and the next save writes 0.5.

The writer stores the amount as the opacity the effect sets (`p:set` of `style.opacity`, `p:to/p:strVal val = 1 - amount`, `crates/pptx/src/write/timing.rs`), but `crates/pptx/src/read/timing.rs` only reads an amount from `animRot` and `animScale`. PowerPoint stores its own Transparency effect the same way, so those decks lose their amount too.

**Version:** main at a58bdbc7 (0.10.0), Linux.

## PR

### Title

PPTX: a Transparency emphasis keeps its amount through save and reopen

### Body

Fixes #ISSUE

**Summary**
A Transparency emphasis effect came back from a .pptx at the 50 % default.

**Root cause**
The writer stores the amount as the opacity the effect sets (`p:set` of `style.opacity` with `p:to/p:strVal` = 1 - amount). The reader only set `amount` from `animRot` and `animScale`, so it never read that value.

**Fix**
For a `transparency` effect, the reader takes the `p:set` whose attribute is `style.opacity` and sets `amount = 1 - val`, clamped to 0..1. A missing or unparsable value keeps the old behaviour (no amount, 50 % in the player). Other effects are untouched.

**Tests**
- New `roundtrip::transparency_keeps_its_amount` in `crates/pptx/tests/it/roundtrip.rs` (two saves).
- `cargo test -p deckcraft-pptx --test it transparency_keeps` without the fix: fails with `left: None, right: Some(0.75)`. With the fix: passes.
- `cargo xtask ci`: all 8 steps passed (fmt, pdf-deps, clippy, test, doctest, assets, layers, wasm), Rust 1.99.1
