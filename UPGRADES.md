# Upgrades

## 0.4.1 — the validator owns the admission limits

`Validating` gains `validated_admission`, which refuses a bootstrap whose byte limit or
timeout is not positive with `BootstrapRefusal::InvalidAdmission`, and `validated` includes it.
The launcher and the Nexus delegate this check to the contract instead of each holding the rule.
`Component::is_one_component` is the one rule for a file or socket name: exactly one normal path
component equal to the raw text, no NUL, which `validated` applies to the file name and the socket
spellings and the launcher applies to every deployment name.
No change to the authored ethos, the generated Rust, the archive, the contract digest or the
Datom text.

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
