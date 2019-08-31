# ShadowPAL
This is a simple app for tracking NPCs and accelerating dice rolls in Shadowrun 5E. The focus is largely toward accelerating combat, but since it's intended to be somewhat generic it can be used for accelerating most rolls.

Part of the purpose of writing this is to help me get a better handle on the rules of the game, so there's bound to be mistakes/misinterpretations.

## Goals
* Make it easy to track NPC stats, damage tracks, and initiative.
* Enable creating, saving, and using NPC templates.
* Simplify dice rolling and provide stored rolls.

## Non-Goals:
* Implement the rules of Shadowrun 5E.
* NPC location management/maps/etc...

## Implementation
_ShadowPAL_ is written in Rust and uses Gtk-rs as its widget library. Building and deploying on Linux should be trivial, but Windows is a much trickier affair.

## Roadmap
- [x] The most common, basic dice rolling mechanics (standard rolls, push the limit, and second chance).
- [x] NPC stats tracking
- [x] Data serialization
- [x] NPC damage tracking
- [x] NPC initiative rolls/tracking
- [ ] NPC editing
- [ ] NPC templates
- [ ] Roll templates