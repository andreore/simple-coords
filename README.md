# Simple Coords

A Horizon mod that shows your coordinates on one line in the top-left corner:

```
X: 123, Y: 65, Z: -45
```

Just where you are — clean and out of the way. `/coords` hides or shows the panel.

## Capabilities

`game-read` (the player's position) and `hud` (the panel).

## Build

```sh
cargo install --git https://github.com/horizon-rust/horizon cargo-horizon-mod
cargo horizon-mod build      # writes target/horizon-mod/simple_coords-0.1.0.hmod
cargo horizon-mod check target/horizon-mod/simple_coords-0.1.0.hmod
```

Put the `.hmod` in Horizon's `horizon-mods/` folder (Options → Mods → Open Mod
Folder) and turn it on in Options → Mods.

## Licence

MIT.
