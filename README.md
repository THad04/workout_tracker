# Workout Tracker

A local-first workout tracker PWA where training programs are written as code, logging a session takes seconds, and history syncs across devices. Under the hood it's a Rust core compiled to WebAssembly, with progression logic as pluggable Wasm components.

**Status:** early development. Core data model in progress (Phase 1).

## Why

Most workout apps are slow to log, rigid about programs, and hide your data behind someone else's server. This project aims for:

- **Fast logging.** Templates prefill last session's numbers, so most sets take a tap or two.
- **Programs as code.** Define a split and its progression rules in a small DSL.
- **Real analytics.** e1RM trends, volume per muscle group, PRs, and stall detection.
- **Local-first.** Works fully offline at the gym; syncs across devices when online.
- **Extensible.** Progression rules are sandboxed Wasm components behind a WIT interface.

Built for personal use, with support for two users, each with their own data.

## Architecture

```
┌─────────────── PWA (TypeScript + Svelte/Solid) ───────────────┐
│  Log screen · History · Charts · Program editor               │
└──────────────────────────┬────────────────────────────────────┘
                           │ wasm-bindgen
┌──────────────────── Rust core (Wasm) ─────────────────────────┐
│  model      Workout, Set, Weight, Exercise library            │
│  analytics  volume, e1RM, PRs, trends                         │
│  dsl        parse + run program files                         │
│  plugins    host for progression-rule components (WIT)        │
│  store      CRDT documents (Automerge or Loro)                │
└──────────────────────────┬────────────────────────────────────┘
                           │ CRDT sync messages
                ┌──────────┴──────────┐
                │ Small sync relay    │  stores/relays docs per user
                └─────────────────────┘
```

## Project layout

### Current

A single crate. The data model lives in `src/domain/`:

```
src/
  main.rs          entry point (thin)
  lib.rs           library root
  domain/
    mod.rs         module declarations and re-exports
    ids.rs         ExerciseId, SetId, WorkoutId, ExerciseKindId
    weight.rs      Weight
    set.rs         Set
    exercise.rs    Exercise, ExerciseKind
    workout.rs     Workout
```

### Planned

`domain/` becomes the `core` crate once the Wasm work starts:

```
crates/
  core/            data model: Workout, Exercise, Set, Weight, IDs
  analytics/       volume, e1RM, PRs, trends, stall detection
  dsl/             program language parser + interpreter
  plugin-host/     loads progression-rule components
  sync/            CRDT document storage + sync protocol
  wasm/            wasm-bindgen bindings for the web app
plugins/           progression rules as Wasm components (WIT)
web/               PWA frontend
server/            minimal sync relay
```

## Programs as code

Training programs are plain text files. Progression rules referenced by a program resolve to Wasm components:

```
program "Upper/Lower" {
  day "Upper A" {
    bench:       2 sets, 6-10 reps, progression: double
    pullup:      2 sets, 8-12 reps, progression: double
    incline_db:  2 sets, 8-12 reps, progression: double
  }
}
```

`progression: double` maps to a component that implements a WIT interface roughly like:

```wit
interface progression {
  suggest-next: func(history: list<session>) -> target;
}
```

So different users can run different rules, and new rules can be added without touching the core.

## Data model notes

- `Weight` is stored as an integer number of tenths of a pound (`Weight(2250)` = 225.0 lb) to avoid float precision issues and make values comparable and hashable.
- IDs are newtypes over UUIDv7, so records created on different devices never collide during sync and still sort roughly by creation time.
- Exercise kinds (e.g. "Bench Press") are stored separately from logged exercises and referenced by ID.

## Roadmap

### Phase 1: Core + logging

- [x] Core structs: `Workout`, `Exercise`, `Set`, `ExerciseKind`
- [x] `Weight` as integer tenths of a pound
- [x] Newtype IDs for exercises, sets, workouts, and exercise kinds
- [x] UUIDv7 IDs
- [x] `Weight` constructors (`from_lbs`, `from_tenths`) and derives
- [x] First unit tests
- [ ] Split model into modules under `src/domain/`
- [ ] Move to a library crate (`lib.rs`)
- [ ] serde derives
- [ ] Exercise library (ID → name lookup)
- [ ] Workout templates that prefill from the last session
- [ ] Volume, e1RM, and PR detection (tested)
- [ ] Wasm build with wasm-bindgen / wasm-pack
- [ ] Minimal PWA: log screen, history, local storage, offline support

### Phase 2: Analytics

- [ ] e1RM trends per lift
- [ ] Weekly volume per muscle group
- [ ] PR timeline
- [ ] Stall detection (e.g. no e1RM gain in 3 weeks)
- [ ] Charts in the UI

### Phase 3: Programs as code

- [ ] DSL grammar and parser (nom or pest) with good error messages
- [ ] In-app program editor
- [ ] Generate each day's session from the program
- [ ] Built-in progression rules (plain Rust)

### Phase 4: Sync + multiple users

- [ ] Move storage to CRDT documents
- [ ] Minimal sync relay
- [ ] Simple auth (magic link or passkey)
- [ ] Multiple devices per user
- [ ] Second user account

### Phase 5: Wasm components

- [ ] WIT interface for progression rules
- [ ] Move built-in rules into components
- [ ] Load components in the browser (jco)
- [ ] Share/import program files between users

## Out of scope (for now)

Native mobile apps, social features, nutrition tracking, wearable integration, public sign-ups.

## Open decisions

- CRDT library: Automerge vs. Loro
- UI framework: Svelte vs. Solid (or plain TypeScript)
- Sync relay hosting: e.g. Fastly Compute
- Sharing between users: fully separate accounts vs. shareable programs

## Getting started

```sh
# run tests
cargo test

# build for the web (planned)
wasm-pack build crates/wasm --target web
```

## Tech stack

Rust · WebAssembly · wasm-bindgen · Wasm Component Model (WIT, jco) · CRDTs · TypeScript · PWA
