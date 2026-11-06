# Upgrades

## 0.3.0 — signal-belief-nexus 0.3.1, translator 0.3.0, Signal 10.3.0, ethos-zero 19.2.0

This is the first upgrade of the text contract from 0.2.0, so it takes a minor
version. The ordinary contract is repinned to signal-belief-nexus 0.3.1 (`a9e3`),
with translator 0.3.0 (`eeaa`), Signal 10.3.0 (`7afb`), datom-codec 0.33.0
(`eabe`) and protos 0.33.0 (`3c7a`), and the flake generator to ethos-zero 19.2.0
(`5a5d`). The committed generated Rust is regenerated: the integer and `String`
newtypes additionally derive `PartialOrd` and `Ord`, and the integer ones also
`Copy`. The authored ethos file is unchanged.

The crate carries `tests/behavior.rs`: queries and responses restore through
validated signal, scalar wrappers archive to the bytes of plain scalars, and under
the `datom` feature queries, responses and a translator `Caller` inside a refusal
round trip through Datom text, a scalar-bearing `Budget` writes the text of plain
scalars. The flake runs the default selection and the `datom` selection.

A consumer selects this contract together with signal-belief-nexus 0.3.1 and
translator 0.3.0, so that one Signal and one translator meet in its graph.
