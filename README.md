## Western Marches

This is a attempt to learn the `bracket-lib` rogue-like library and also work a little bit on my Rust skills.

### Run

Probably something like this:

```
> git checkout western-rltk
> cd western-rltk
> cargo run --release
```

### TODO

* No character builder
* No town or stores
* ~~No usable items or inventory~~
* No spells or effects
* ~~No stairs or additional levels~~
* No save or load game
* No win condition

### Known Bugs
* The Image file used for the actor and item layers renders many ~~greens~~ dark colors as transparent
* Tooltip on locaton generates multiple messages causing flickering

### Credit
* This is built on the [brackets-lib](https://github.com/thebracket/bracket-lib) and inspired by the book _Hands-on Rust_.
* Most of the tiles are from [@JoeCreates](https://joecreates.co.uk/) and found on OpenGameArt.org
