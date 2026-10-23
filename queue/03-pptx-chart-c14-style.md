# Queue item: issue + PR

- Kind: issue+pr
- Target repo: storytold/deckcraft
- Base branch: main
- Head branch: xc4pr:fix/pptx-chart-c14-style
- Head commit: db923979bf4cef81eaa14c3b7aea27ee7ccc0c3d
- Tested on: upstream main a58bdbc7 (deckcraft 0.10.0)

## Issue

### Title

PPTX: an edited chart is saved with chart style 48 instead of its own style

### Body

PowerPoint writes a chart's style twice:

```xml
<mc:AlternateContent>
  <mc:Choice Requires="c14"><c14:style val="102"/></mc:Choice>
  <mc:Fallback><c:style val="2"/></mc:Fallback>
</mc:AlternateContent>
```

`c14:style` is 100 + the style number (101 to 148). `resolve_mc` takes the c14 choice, and `crates/pptx/src/read/chart.rs` stores its value as is, so a chart in style 2 reads as style 102.

**Steps**

1. Open a .pptx whose chart part has the markup above.
2. Edit the chart (for example change a value) and save.

**Expected:** the saved chart keeps `<c:style val="2"/>`.

**Actual:** it is written as `<c:style val="48"/>` (`clamp(1, 48)` in `crates/pptx/src/write/chart.rs`), the last of the dark styles. An unchanged chart is not affected, because its original part is reused.

**Version:** main at a58bdbc7 (0.10.0), Linux.

## PR

### Title

PPTX: read a chart's Office 2010 style number as the style it names

### Body

Fixes #ISSUE

**Summary**
An edited chart from a PowerPoint deck was saved with chart style 48 instead of its own style.

**Root cause**
PowerPoint writes `c14:style` (100 + the style number) with `c:style` as its fallback. `resolve_mc` keeps the c14 choice, and the reader stored its value unchanged (102 for style 2). Once the chart is edited, the writer clamps that to 48.

**Fix**
The reader subtracts 100 from a style number above 100. Plain `c:style` values (1 to 48) read as before; the writer is unchanged. The unchanged-chart check in `chart_xml` reads through the same function, so unchanged charts still reuse their original part.

**Tests**
- New `read::chart::tests::office_2010_style_number_is_read_as_the_style_it_names`: PowerPoint's AlternateContent markup reads as style 2, and a chart written from the model contains `<c:style val="2"/>`.
- `cargo test -p deckcraft-pptx --lib office_2010_style` without the fix: fails with `left: 102, right: 2`. With the fix: passes; `cargo test -p deckcraft-pptx --lib chart`: 19 passed.
- `cargo xtask ci`: all 8 steps passed (fmt, pdf-deps, clippy, test, doctest, assets, layers, wasm), Rust 1.99.1
