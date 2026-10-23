# Queue item: review

- Kind: review (event: COMMENT)
- Target repo: storytold/deckcraft
- PR: #1075 "Indent table rows in outline export so they stay on their slide"
- Checked commit: 502fb2ba73a3809b36d720982eaea5b5935242f4

## Body

I checked out 502fb2b and ran `cargo test -p deckcraft-format` (30 passed), `cargo clippy -p deckcraft-format --all-targets -- -D warnings` (clean) and `cargo fmt --check` (clean). Without the one-line `out.push('\t')`, `outline_table_rows_are_indented_and_do_not_become_slides` fails, so it covers the change. The branch merges cleanly into current main.

I also round-tripped a titled slide, as suggested above. The slide is a Title Only slide titled "Results", with a 2×2 table whose rows are `A1 | B1` and ` | B2` (the first cell of row 2 is empty).

- Export: `"Results\n\tA1\tB1\n\t\tB2\n"`
- Import: one slide. The title is "Results", the body is `A1\tB1` at level 0, then `B2` at level 1.

The fix works for a titled slide. A row is no longer split off into a new slide, and its cells keep their tab separators inside the body line.

One case still goes wrong. If a row starts with empty cells, those cells become extra leading tabs. The import reads them as indentation, so row 2 comes back as a level-1 bullet and drops its empty first column. Two ways to handle it:

- Write something in place of a leading empty cell, so the separators are not leading whitespace.
- Accept the limitation and document it in the test.

Either way, a test row with an empty first cell would pin down the behaviour.
