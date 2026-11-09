# Upgrades

## 0.6.1 — translator unix; signal-belief-nexus 0.4.2

The translator dependency turns on its `unix` feature, which attaches libc and nix: the edge the build base's feature set asks of a repository that builds translator. signal-belief-nexus moves to 0.4.2 (011eea2), which carries the same edge. No change to the authored ethos, the generated Rust, the archive or the Datom text. The feature leaves this manifest when `unix` becomes a crate of its own.

## 0.6.0 — SettingAddress; signal-belief-nexus 0.4.1, translator 0.5.0, signal 11.0.0

A setting kept per key is named once and its key sits beside it. `SettingName` names `HarnessLayouts` and `PlacementDefaults` with no payload (0.5.1 had `LayoutOfHarness.Harness` and `PlacementOfKind.Kind`); new `SettingKey.[ Harness Kind ]` and `SettingAddress.{ SettingName Option<SettingKey> }`; `Readiness.Awaiting` is a vector of `SettingAddress`, and `Invalid` carries a `SettingAddress` where it carried a `SettingName`. The pins move: signal-belief-nexus 0.4.1 (a6d3bda), translator 0.5.0 (abc89b9) and signal 11.0.0 (6d4eb74), which carries no transport feature and no tokio. A consumer repins the three together.

## 0.5.1 — Set writes HarnessLayout and PlacementDefault bare

In `Set`, the variants `LayoutOfHarness.HarnessLayout` and `PlacementOfKind.PlacementDefault` of 0.5.0 are written bare: `HarnessLayout` and `PlacementDefault`, each carrying the type it is named for, the Rust unchanged but for the variant names `Set::HarnessLayout(HarnessLayout)` and `Set::PlacementDefault(PlacementDefault)`. `SettingName` keeps `LayoutOfHarness.Harness` and `PlacementOfKind.Kind`. A consumer matching `Set::LayoutOfHarness` and `Set::PlacementOfKind` matches the new names.

## 0.5.0 — variants renamed for ethos-zero 22.0.0; signal-belief-nexus 0.4.0, translator 0.4.2

ethos-zero 22.0.0 refuses a variant named for a defined type that carries another payload. Four variants are renamed, with the same payloads: in `SettingName`, `Layout.Harness` becomes `LayoutOfHarness.Harness` and `Placement.Kind` becomes `PlacementOfKind.Kind`; in `Set`, `Layout.HarnessLayout` becomes `LayoutOfHarness.HarnessLayout` and `Placement.PlacementDefault` becomes `PlacementOfKind.PlacementDefault`. A consumer matching `SettingName::Layout`, `SettingName::Placement`, `Set::Layout` or `Set::Placement` matches the new names; the Datom text names the new variants. The pins move: signal-belief-nexus 0.4.0 (c388b11), translator 0.4.2 (27da70f), signal 10.4.1 (d77169d), datom-codec 09a0526, protos c93b598; the generator is ethos-zero 22.0.0 (1976e45). The `datom` feature carries signal/datom. The repository builds on build-base 9ee222c.

## 0.4.1 — the contract owns the admission limits, and one component rule

New, and no existing method changed: the trait `Admitting` with `admitted`, which refuses a bootstrap
whose byte limit or timeout is not positive with the new `BootstrapRefusal::InvalidAdmission`;
`Validating::validated` calls it. The launcher and the Nexus delegate this check to the contract
instead of each holding the rule. `Validating` itself is unchanged, so an external implementor of it
still compiles. The trait `Component` with `is_one_component` is the one rule for a file or socket
name: exactly one normal path component equal to the raw text, no NUL; the socket spelling check, the
file name check and the launcher's five deployment names all use it.

API notes. `BootstrapRefusal` is a local Rust enum, with no archive or Datom form and no place in
the authored ethos, so the new variant `InvalidAdmission` changes neither the wire nor the contract
digest; it breaks only an exhaustive external `match` on that enum. The selected callers were
checked: the belief body wraps and formats the refusal and has no exhaustive match. No change to the
authored ethos, the generated Rust, the archive or the Datom text.

## 0.4.0 — a typed Place in the Nexus bootstrap

`NexusBootstrap` carries `Place`, the published translator type with its place name, runtime
and state, and `MemoryFileName`, in place of `MemoryPath`. The memory path is one value:
the Place state joined with the file name. The three sockets stay explicit and are checked
against the Place runtime. The crate adds `Validating`, `Locating` and `Enveloped`:
`validated` refuses, before any effect, an unsafe place name, a relative runtime or state,
a memory file name that is not one normal component or that is its own `sema` legacy name,
a socket that is not directly in the runtime or not distinct, an owner that is not the
effective user and a paired translator of another user. `Enveloped` is the 8-byte frame
that precedes a Nexus bootstrap on its pipe: the digest of this contract, which changes
with the ethos, so a bootstrap sent by another contract is refused before it is read.

The Query and Response layouts are unchanged; the contract digest is not, so a 0.3.0 and a
0.4.0 peer are refused at the greeting. The bootstrap archive layout changes; a representative
0.3.0 frame does not restore as the new bootstrap, shown in `tests/bootstrap.rs`, and no claim
is made about every historical byte sequence. Selects signal-belief-nexus 0.3.3 and
translator 0.3.2 (`5719`).

## 0.3.0 — signal-belief-nexus 0.3.2, translator 0.3.1, Signal 10.3.0, ethos-zero 19.2.0

This is the first upgrade of the text contract from 0.2.0, so it takes a minor
version. The ordinary contract is repinned to signal-belief-nexus 0.3.2 (`cc87`),
with translator 0.3.1 (`1b0e`), Signal 10.3.0 (`7afb`), datom-codec 0.33.0
(`eabe`) and protos 0.33.0 (`3c7a`), and the flake generator to ethos-zero 19.2.0
(`5a5d`). The committed generated Rust is regenerated: the integer and `String`
newtypes additionally derive `PartialOrd` and `Ord`, and the integer ones also
`Copy`. The authored ethos file is unchanged.

The crate carries `tests/behavior.rs`: queries and responses restore through
validated signal, scalar wrappers archive to the bytes of plain scalars, and under
the `datom` feature queries, responses and a translator `Caller` inside a refusal
round trip through Datom text, a scalar-bearing `Budget` writes the text of plain
scalars. The flake runs the default selection and the `datom` selection.

A consumer selects this contract together with signal-belief-nexus 0.3.2 and
translator 0.3.1, so that one Signal and one translator meet in its graph.
