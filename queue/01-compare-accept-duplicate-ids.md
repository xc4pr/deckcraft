# Queue item: issue + PR

- Kind: issue+pr
- Target repo: storytold/deckcraft
- Base branch: main
- Head branch: xc4pr:fix/compare-accept-duplicate-ids
- Head commit: 9fe48ee852fcb26a8f9aa7558065e42ae7919496
- Tested on: upstream main a58bdbc7 (deckcraft 0.10.0)

## Issue

### Title

Review > Compare: accepting a revised slide can give two shapes on it the same id

### Body

When the open deck and the revised copy both gained shapes after the copy was made, accepting a revised slide (`review.accept`) can leave two shapes on that slide with the same id.

**Steps**

1. New presentation, add a second slide (`slide.new`).
2. Save a copy (the "revised" deck). In the copy, insert two shapes on slide 1.
3. In the open deck, insert one shape on slide 2.
4. `review.compare` with the copy, then `review.accept {index: 0}` (slide 1).

**Expected:** slide 1 as revised, every shape id unique.

**Actual:** `Presentation::validate()` reports `slide 257: duplicate shape id 264`.

Both decks hand out the same new ids. The revised slide's first new shape clashes with the open deck's new shape on slide 2, so `adopt()` renumbers it with `alloc_id()`. The open deck's `next_id` is only raised to the slide's highest id after renumbering, so the fresh id is the id of the second new shape on the same slide.

**Version:** main at a58bdbc7 (0.10.0), Linux.

## PR

### Title

Review: accepting a revised slide never gives two shapes the same id

### Body

Fixes #ISSUE

**Summary**
Accepting a revised slide in Review > Compare could leave two shapes on that slide with the same id.

**Root cause**
`compare::adopt` renumbers a revised shape whose id another slide of the open deck already uses, with `doc.alloc_id()`. `next_id` was raised to the slide's highest shape id only after renumbering, so a fresh id could equal an id that a later shape on the same slide keeps. This happens when both decks gained shapes after they split, because they then hand out the same new ids.

**Fix**
Raise `next_id` to the slide's highest shape id (groups included, via `max_shape_id`) before renumbering. The final `next_id` is the same as before; fresh ids now always come after every id the slide keeps.

**Tests**
- New `compare::tests::accepting_a_slide_never_duplicates_a_shape_id`, through the public commands only.
- `cargo test -p deckcraft-engine --lib accepting_a_slide_never_duplicates` without the fix: fails with `["slide 257: duplicate shape id 264"]`. With the fix: passes; `cargo test -p deckcraft-engine --lib compare`: 4 passed.
- `cargo xtask ci`: all 8 steps passed (fmt, pdf-deps, clippy, test, doctest, assets, layers, wasm), Rust 1.99.1
