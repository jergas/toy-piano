# Toy Piano

Simple Toy Piano app to use with a MIDI keyboard.

Just plug your MIDI controller, and then run the app. It will try to autoselect your controller, or you can select it from the drop down menu.

## SoundFont Required

You'll need a SoundFont. We've been using [SalamanderGrandPiano](https://freepats.zenvoid.org/Piano/SalamanderGrandPiano/SalamanderGrandPiano-SF2-V3+20200602.tar.xz), but you may find others at [FreePats](https://freepats.zenvoid.org/about.html), or by [searching for them](https://www.google.com/search?q=open%20source%20soundfont).

Place the `.sf2` file in the `assets/` folder next to the executable.

## Building from Source

```bash
cargo build --release
```

## License

Open source. See LICENSE for details.
