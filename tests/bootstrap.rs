//! The Nexus bootstrap names a Place and a file, and is checked before it has any effect.

use meta_signal_belief_nexus::{
    Admission, BootstrapOwner, BootstrapRefusal, ByteLimit, EmissionTranslatorSocket, Enveloped,
    Locating, MemoryFileName, MetaSocket, NexusBootstrap, OrdinarySocket, PairedTranslator,
    SocketPath, Timeout, Validating,
};
use signal::{ByteViewable, Restorable, Signal, Signalizable};

const USER: i64 = 1000;

fn place() -> translator::Place {
    translator::Place {
        place_name: translator::PlaceName("belief".into()),
        runtime: translator::Runtime("/run/user/1000/nexus/belief".into()),
        state: translator::State("/home/u/.local/state/nexus/belief".into()),
    }
}

fn socket(name: &str) -> SocketPath {
    SocketPath(format!("/run/user/1000/nexus/belief/{name}"))
}

fn bootstrap() -> NexusBootstrap {
    NexusBootstrap {
        ordinary_socket: OrdinarySocket(socket("ordinary.sock")),
        meta_socket: MetaSocket(socket("meta.sock")),
        emission_translator_socket: EmissionTranslatorSocket(socket("translator.sock")),
        place: place(),
        memory_file_name: MemoryFileName("memory".into()),
        admission: Admission {
            byte_limit: ByteLimit(1_000_000),
            timeout: Timeout(1_000),
        },
        paired_translator: PairedTranslator {
            user_id: translator::UserId(USER),
            group_id: translator::GroupId(100),
            process_id: translator::ProcessId(42),
        },
        bootstrap_owner: BootstrapOwner(signal_belief_nexus::Owner::Login(translator::UserId(
            USER,
        ))),
    }
}

#[test]
fn a_coherent_bootstrap_is_accepted_and_names_one_memory_path() {
    assert_eq!(bootstrap().validated(USER), Ok(()));
    assert_eq!(
        bootstrap().memory_path(),
        std::path::Path::new("/home/u/.local/state/nexus/belief/memory")
    );
}

#[test]
fn a_bootstrap_restores_through_validated_signal() {
    let archived = bootstrap().signalize().expect("archive bootstrap");
    let restored = Signal::<NexusBootstrap>::from(archived.bytes().to_vec())
        .restore()
        .expect("restore");
    assert_eq!(restored, bootstrap());
}

#[test]
fn file_names_that_are_not_one_normal_component_are_refused() {
    for name in ["", ".", "..", "../memory", "a/b", "/abs", "mem\0ory"] {
        let mut value = bootstrap();
        value.memory_file_name = MemoryFileName(name.into());
        assert_eq!(
            value.validated(USER),
            Err(BootstrapRefusal::UnsafeFileName(name.into())),
            "{name:?}"
        );
    }
}

#[test]
fn a_file_name_that_is_its_own_legacy_name_is_refused_and_others_are_not() {
    for name in ["memory.sema", "belief.sema"] {
        let mut value = bootstrap();
        value.memory_file_name = MemoryFileName(name.into());
        assert_eq!(
            value.validated(USER),
            Err(BootstrapRefusal::LegacyExtension(name.into())),
            "{name:?}"
        );
    }
    for name in [
        "memory",
        "belief.memory",
        "sema",
        "a.sema.memory",
        "memory.SEMA",
    ] {
        let mut value = bootstrap();
        value.memory_file_name = MemoryFileName(name.into());
        assert_eq!(value.validated(USER), Ok(()), "{name:?}");
    }
}

#[test]
fn place_names_and_roots_that_are_unsafe_are_refused() {
    for name in ["", ".", "..", "a/b", "/abs"] {
        let mut value = bootstrap();
        value.place.place_name = translator::PlaceName(name.into());
        assert_eq!(
            value.validated(USER),
            Err(BootstrapRefusal::UnsafePlaceName(name.into())),
            "{name:?}"
        );
    }
    let mut value = bootstrap();
    value.place.state = translator::State("relative/state".into());
    assert_eq!(
        value.validated(USER),
        Err(BootstrapRefusal::RelativePlace("relative/state".into()))
    );
    let mut value = bootstrap();
    value.place.runtime = translator::Runtime("relative/runtime".into());
    assert_eq!(
        value.validated(USER),
        Err(BootstrapRefusal::RelativePlace("relative/runtime".into()))
    );
}

#[test]
fn sockets_must_sit_directly_in_the_runtime_and_be_distinct() {
    for path in [
        "/elsewhere/ordinary.sock",
        "/run/user/1000/nexus/belief/sub/ordinary.sock",
        "/run/user/1000/nexus/belief/..",
        "/run/user/1000/nexus/belief/",
        "ordinary.sock",
    ] {
        let mut value = bootstrap();
        value.ordinary_socket = OrdinarySocket(SocketPath(path.into()));
        assert!(
            matches!(
                value.validated(USER),
                Err(BootstrapRefusal::SocketOutsideRuntime(_))
            ),
            "{path:?}"
        );
    }
    let mut value = bootstrap();
    value.meta_socket = MetaSocket(socket("ordinary.sock"));
    assert_eq!(
        value.validated(USER),
        Err(BootstrapRefusal::SocketsNotDistinct)
    );
}

#[test]
fn the_owner_and_the_paired_translator_must_be_the_effective_user() {
    assert_eq!(
        bootstrap().validated(USER + 1),
        Err(BootstrapRefusal::OwnerNotEffectiveUser)
    );
    let mut value = bootstrap();
    value.paired_translator.user_id = translator::UserId(USER + 1);
    assert_eq!(
        value.validated(USER),
        Err(BootstrapRefusal::PairedUserNotEffectiveUser)
    );
    let mut value = bootstrap();
    value.bootstrap_owner = BootstrapOwner(signal_belief_nexus::Owner::Peer(translator::PeerId(
        "peer".into(),
    )));
    assert_eq!(
        value.validated(USER),
        Err(BootstrapRefusal::OwnerNotEffectiveUser)
    );
}

#[test]
fn the_envelope_names_this_contract_and_refuses_others() {
    assert_eq!(NexusBootstrap::admit(&NexusBootstrap::envelope()), Ok(()));
    assert_eq!(
        NexusBootstrap::admit(&[]),
        Err(BootstrapRefusal::EnvelopeMalformed)
    );
    assert_eq!(
        NexusBootstrap::admit(&[0; 7]),
        Err(BootstrapRefusal::EnvelopeMalformed)
    );
    let mut other = NexusBootstrap::envelope();
    other[0] ^= 1;
    assert_eq!(
        NexusBootstrap::admit(&other),
        Err(BootstrapRefusal::EnvelopeMismatch)
    );
}

/// The 0.3.0 shape, kept here as a mirror: a representative old frame, not every historical byte.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
struct PreviousBootstrap {
    ordinary_socket: String,
    meta_socket: String,
    emission_translator_socket: String,
    memory_path: String,
    byte_limit: i64,
    timeout: i64,
}

#[test]
fn a_representative_previous_frame_does_not_restore_as_the_current_bootstrap() {
    let previous = PreviousBootstrap {
        ordinary_socket: "/run/o.sock".into(),
        meta_socket: "/run/m.sock".into(),
        emission_translator_socket: "/run/t.sock".into(),
        memory_path: "/state/memory".into(),
        byte_limit: 1_000_000,
        timeout: 1_000,
    };
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&previous)
        .expect("archive previous")
        .to_vec();
    assert!(Signal::<NexusBootstrap>::from(bytes).restore().is_err());
}

#[cfg(feature = "datom")]
mod text {
    use super::*;
    use datom_codec::{Actualizing, Budget, Composing, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    #[test]
    fn a_bootstrap_with_its_place_round_trips_through_text() {
        let text = bootstrap().datomize(Vec::new()).protosize().textualize();
        let mut budget = Budget {
            remaining: 8192,
            reader: ReaderBudget::try_from(8192usize).unwrap(),
            depth: 0,
            maximum_depth: 64,
        };
        let read = Potential::<NexusBootstrap>::from(text)
            .actualize(&mut budget)
            .expect("read");
        assert_eq!(read, bootstrap());
    }
}
