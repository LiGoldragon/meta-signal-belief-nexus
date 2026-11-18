//! The meta contract as a wire and as Datom text, through its public API.

use meta_signal_belief_nexus::{
    Budget, ByteLimit, ConfigurationRefusal, ConfigurationRevision, ConfigureRequest,
    ConfiguredState, DepthLimit, Input, Keeper, NodeLimit, Output, Readiness, Set, SettingAddress,
    SettingName, SocketPath, Timeout,
};
use rkyv::rancor::Error;
use signal::{ByteViewable, Restorable, Signal, Signalizable};

fn budget() -> Budget {
    Budget {
        node_limit: NodeLimit(64),
        depth_limit: DepthLimit(8),
    }
}

fn configure(set: Set) -> Input {
    Input::Configure(ConfigureRequest {
        configuration_revision_option: Some(ConfigurationRevision(3)),
        set,
    })
}

fn configured(set: Set) -> Output {
    Output::Configured(ConfiguredState {
        set,
        keeper: Keeper::Translator,
        configuration_revision: ConfigurationRevision(4),
        readiness: Readiness::Awaiting(vec![
            SettingAddress {
                setting_name: SettingName::TextBudget,
                setting_key_option: None,
            },
            SettingAddress {
                setting_name: SettingName::NexusSocket,
                setting_key_option: None,
            },
        ]),
    })
}

fn caller() -> translator::Caller {
    translator::Caller {
        process_id: translator::ProcessId(42),
        user_id: translator::UserId(1000),
    }
}

fn refused() -> Output {
    Output::ConfigureRefused(ConfigurationRefusal::NotAuthorized(caller()))
}

#[test]
fn queries_and_responses_restore_through_validated_signal() {
    for set in [
        Set::TextBudget(budget()),
        Set::RepositoryPolling(Timeout(5)),
        Set::OutputLimit(ByteLimit(4096)),
        Set::NexusSocket(SocketPath("/run/belief/nexus.sock".into())),
    ] {
        let query = configure(set.clone());
        let archived = query.signalize().expect("archive query");
        assert_eq!(
            Signal::<Input>::from(archived.bytes().to_vec())
                .restore()
                .unwrap(),
            query
        );
        let response = configured(set);
        let archived = response.signalize().expect("archive response");
        assert_eq!(
            Signal::<Output>::from(archived.bytes().to_vec())
                .restore()
                .unwrap(),
            response
        );
    }
    let archived = refused().signalize().expect("archive refusal");
    assert_eq!(
        Signal::<Output>::from(archived.bytes().to_vec())
            .restore()
            .unwrap(),
        refused()
    );
}

#[test]
fn scalar_wrappers_archive_to_the_bytes_of_plain_scalars() {
    #[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
    struct PlainBudget {
        node_limit: i64,
        depth_limit: i64,
    }
    let new = rkyv::to_bytes::<Error>(&budget()).expect("archive budget");
    let old = rkyv::to_bytes::<Error>(&PlainBudget {
        node_limit: 64,
        depth_limit: 8,
    })
    .expect("archive plain");
    assert_eq!(new.as_slice(), old.as_slice());
}

#[cfg(feature = "datom")]
mod text {
    use super::*;
    use datom_codec::{Actualizing, Budget as ReadBudget, Composing, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    fn written<T: Datomizable>(value: &T) -> String {
        value.datomize(Vec::new()).protosize().textualize()
    }

    fn read<T: Composing>(text: &str) -> T {
        let mut budget = ReadBudget {
            remaining: 4096,
            reader: ReaderBudget::try_from(4096usize).unwrap(),
            depth: 0,
            maximum_depth: 64,
        };
        Potential::<T>::from(text)
            .actualize(&mut budget)
            .expect("read the datom text")
    }

    #[test]
    fn queries_and_responses_round_trip_through_text() {
        for set in [
            Set::TextBudget(budget()),
            Set::RepositoryPolling(Timeout(5)),
            Set::NexusSocket(SocketPath("/run/belief/nexus.sock".into())),
        ] {
            let query = configure(set.clone());
            let text = written(&query);
            assert_eq!(read::<Input>(&text), query);
            assert_eq!(written(&read::<Input>(&text)), text);
            let response = configured(set);
            assert_eq!(read::<Output>(&written(&response)), response);
        }
    }

    #[test]
    fn the_configures_metaflow_writes_are_read_as_they_were() {
        assert_eq!(
            read::<Input>("Configure.{ None RepositoryPolling.211 }"),
            Input::Configure(ConfigureRequest {
                configuration_revision_option: None,
                set: Set::RepositoryPolling(Timeout(211)),
            })
        );
        let peer = translator::PeerBinding {
            peer_id: translator::PeerId("flow".into()),
            peer_identity: translator::PeerIdentity {
                user_id: translator::UserId(1000),
                process_id: translator::ProcessId(42),
            },
        };
        assert_eq!(
            read::<Input>("Configure.{ None OrdinaryPeers.[ { flow { 1000 42 } } ] }"),
            Input::Configure(ConfigureRequest {
                configuration_revision_option: None,
                set: Set::OrdinaryPeers(signal_belief_nexus::PeerList(vec![peer])),
            })
        );
    }

    #[test]
    fn translator_identity_in_a_refusal_round_trips_through_text() {
        assert_eq!(read::<Output>(&written(&refused())), refused());
    }

    #[test]
    fn scalar_wrappers_write_the_text_of_plain_scalars() {
        #[derive(datom_codec::Datomizable, datom_codec::Composing)]
        struct PlainBudget {
            node_limit: i64,
            depth_limit: i64,
        }
        let plain = PlainBudget {
            node_limit: 64,
            depth_limit: 8,
        };
        assert_eq!(written(&budget()), written(&plain));
        assert_eq!(read::<Budget>(&written(&plain)), budget());
    }
}
