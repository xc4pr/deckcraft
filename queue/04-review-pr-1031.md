# Queue item: review

- Kind: review (event: COMMENT)
- Target repo: storytold/deckcraft
- PR: #1031 "Round chart number format halves away from zero like Excel"
- Checked commit: 1008e1096db2b714ac2b8ac789a59bf9c8148b2d

## Body

I checked out 1008e10 and ran `cargo test -p deckcraft-model` (79 passed), `cargo clippy -p deckcraft-model --all-targets -- -D warnings` (clean) and `cargo fmt --check` (clean).

To see which values the change fixes, I ran `numfmt::format` on the PR head and on the merge base with the same inputs:

| Code | Value | Base | This PR | Excel |
|---|---|---|---|---|
| `0%` | 0.125 | 12% | 13% | 13% |
| `0.0%` | 0.0045 | 0.4% | 0.5% | 0.5% |
| `0.0` | 1.45 | 1.4 | 1.5 | 1.5 |
| `0.00` | 2.675 | 2.67 | 2.68 | 2.68 |
| `0.00` | 1.115 | 1.11 | 1.12 | 1.12 |
| `#,##0.00` | 1234567.125 | 1,234,567.12 | 1,234,567.13 | 1,234,567.13 |
| `0.0E+00` | 2.25 | 2.2E+00 | 2.3E+00 | 2.3E+00 |
| `0.00` | 1.005 | 1.00 | **1.00** | 1.01 |
| `0.00` | 1.015 | 1.01 | **1.01** | 1.02 |
| `0.00` | 0.285 | 0.28 | **0.28** | 0.29 |

So the PR fixes exact binary halves and some near-halves where `x * 10^n` happens to land on .5. The last three rows still round down, because `1.005 * 100` is `100.49999999999999`. These are the cases already raised in this thread. Rounding the 15-significant-digit decimal value (as suggested above) is what makes the last three rows match. The table's first seven rows would make good test cases for that version too.

Two more things I noticed:

- The branch no longer merges cleanly: `git merge-tree` against current main reports a conflict in `crates/model/src/numfmt.rs`. `fixed()` changed on main since this branch was cut (#1091, #1099, #1035), so the rounding will need to be redone on top of the new code.
- `format("0", -0.4)` gives `-0` both before and after. I believe Excel shows `-0` there as well, so I don't think this needs a change. I'm mentioning it only so a future test doesn't assume `0`.
