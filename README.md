Check out the solana on-chain logic!
https://github.com/spacemandev-git/dominari-ecs

## Quick Start

```
bash build_wasm.sh
# or for change detection, install cargo watch 
# then: cargo watch -- bash build_wasm.sh

# new terminal
cd wasmtest
npm ci
npm start
```

## MVP
### Data model

- Define components mappings to on-chain state
- Create systems to take chain update events and update bevy data model
- Handling keypairs
  - generates custodial keypairs
- Handle connection info
- Fetch all entities by instanceId (sdk?)

### Rendering

- Render a tilemap
  - Tile sprite determined by feature type
- Render units
- Select a tile & display info
- Select a unit & display info
- Play a card
- View cards
- Score
- Info about other players??
- Tooltips

### Submit user actions on-chain

---

Player flow

Screen 1
Welcome to Dominari

- Connect or generate keypair

Screen 2

- Generate map -> becomes admin
- Join map by instanceId

Screen 3

- Modal: is game started & how many players are in lobby
- normal game view

Normal Game View

- View of the map

  - tiles
  - features
  - units
    - recovering vs active gfx 

- View of cards
- Player metadata: score, in-game currency

Click on Tile:

- highlight the tile
- ui window w/
  - feature info (use/upgrade)
  - unit (move/attack)
  - if unit on tile, hightlight tiles it can move/attack
  - unit slots until available 

If tile w/ unit selected and hover over tile w/ enemy unit:

- UI w/ projected outcome of attack or smt

Click on Card:

- highlight card in deck
- highlight placeable tiles
- ui window w/ info

Game Over Screen
- Who won
