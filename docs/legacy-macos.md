# Running Toy Piano on legacy macOS (El Capitan and friends)

**Status: untested.** Nothing in this document has been executed against a real
El Capitan machine. It is a plan, written down so we can try it in one sitting
and stop early if it goes badly.

The machine this concerns: macOS 10.11.6 El Capitan, 3.06 GHz Intel Core i3,
4 GB RAM.

## 1. Why it is hard

Two independent floors stand between us and a working build:

| Floor | Version | Consequence |
| --- | --- | --- |
| Rust toolchain | macOS **10.12+** since Rust 1.63 (2022) | 10.11.6 is *below* the compiler's own minimum |
| `wgpu` 0.19 (via `iced` 0.12) | macOS 10.13–10.15 in practice | the GUI stack is further ahead still |

Neither is documented as a hard `rust-version` or `MACOSX_DEPLOYMENT_TARGET`
in the dependency tree, so we have to discover the real floor empirically.

## 2. Do the cheap check first: can the machine be upgraded?

This is worth ten minutes before any toolchain archaeology, because it may make
the whole problem disappear.

El Capitan is not necessarily the ceiling. The machine can likely reach
**High Sierra (10.13)**, which is already above Rust's 10.12 floor and only one
step from what `wgpu` wants.

Check the exact model first — `Apple menu → About This Mac`, or:

```bash
system_profiler SPHardwareDataType | grep -E 'Model Name|Model Identifier|Processor'
```

- **Metal support is the gate for Mojave (10.14).** Ivy Bridge integrated
  graphics (HD Graphics 3000/4000) have no Metal support, so **Mojave is almost
  certainly out**. High Sierra is the realistic target.
- Whatever the identifier, try the App Store / `softwareupdate` route to High
  Sierra. Back up first.

If the machine lands on 10.13, re-test with a stock modern toolchain before
reading the rest of this document.

## 3. Establishing the actual floor

On any Mac that can build the project, ask the binary what it actually
requires:

```bash
cargo build --release
otool -l target/release/toy-piano | grep -A4 LC_BUILD_VERSION
# or, on older toolchains:
otool -l target/release/toy-piano | grep -A5 LC_VERSION_MIN_MACOSX
```

`minos` is the answer. Then try to push it lower:

```bash
MACOSX_DEPLOYMENT_TARGET=10.11 cargo build --release
```

Expect either success, or a link error naming a symbol introduced after 10.11.
**That error is the real floor**, and it belongs to a dependency, not to our
code — note which one.

## 4. The old-toolchain attempt

Only if step 2 fails:

```bash
rustup toolchain install 1.62.0     # last Rust with a macOS 10.11-capable floor
rustup override set 1.62.0
```

Then the dependencies must also age, and this is where it gets expensive.
`iced 0.12` will not build on a 2022 toolchain; we would be walking backwards
through `iced` (0.12 → 0.11 → 0.10 → …) and `wgpu` (0.19 → 0.14 → …), each step
possibly requiring source edits of its own.

**Budget this honestly.** If step 4 needs more than one sitting, the right call
is to say so plainly rather than sink more days.

## 5. The pragmatic fallback

If the GUI stack cannot be aged back, split the target:

- **A GUI-less build for the old machine.** `cpal` + `midir` + `rustysynth` are
  plain Rust with no graphics dependency, so they have no business needing a
  2019 macOS. A small `--headless` mode (MIDI in, audio out, no window) would
  plausibly run all the way back to 10.11 and still be *useful* — it is a piano.
  Cost: a few hundred lines behind a feature flag.
- **The GUI build targets whatever the GUI can actually reach** (10.13+),
  documented plainly in the README.

That gets Raven a working piano on old hardware without holding the whole
project hostage to `wgpu`.

## 6. What to record

Whichever way this lands, the answer belongs in `roadmap.md` under Phase 4
("Legacy macOS Support") and the floor we discovered belongs in this file, so
nobody re-derives it.