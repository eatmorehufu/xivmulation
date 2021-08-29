# xivmulation
XIV rotation simulator.

## Feature Goals

### Phase 0
- Basic event engine
- Damage formulas with stat input
- Single class (PLD)
- Takes moves in one at a time and prints it out to a log (on screen, not saved)
- Counts amount of damage done

### Future Goals
- Priority-based rotation definition
- Support all classes
- Stat/CD tracking and rotation validation
- Simulator creates rotation during the simulation (especially important for proc-based rotations)
- Import character with stats from Lodestone (optional)

## Prerequisites
Trunk:

```cargo install trunk wasm-bindgen-cli```

wasm32-unknown-unknown

```rustup target add wasm32-unknown-unknown```

## Starting the server

```trunk serve```