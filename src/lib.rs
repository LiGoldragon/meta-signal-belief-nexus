#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Keeper {
    Nexus,
    Translator,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ConfigurationRevision(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SettingName {
    RepositoryPolling,
    SubscriberCapacity,
    OrdinaryPeers,
    MetaPeers,
    Layout(signal_belief_nexus::Harness),
    Placement(signal_belief_nexus::Kind),
    GenerationRetention,
    NexusFrameCapacity,
    NexusTransportTimeout,
    NexusSocket,
    MetaNexusSocket,
    CallTimeout,
    OutputLimit,
    TextBudget,
    TranslatorFrameCapacity,
    TranslatorTransportTimeout,
    AllowedNexusPeers,
    GitExecutable,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Readiness {
    Awaiting(std::vec::Vec<SettingName>),
    Ready,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Timeout(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ByteLimit(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NodeLimit(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DepthLimit(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Budget {
    pub node_limit: NodeLimit,
    pub depth_limit: DepthLimit,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SocketPath(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct OrdinarySocket(pub SocketPath);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MetaSocket(pub SocketPath);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct OrdinaryTranslatorSocket(pub SocketPath);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MetaTranslatorSocket(pub SocketPath);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct OrdinaryNexusSocket(pub SocketPath);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MetaNexusSocketPath(pub SocketPath);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct EmissionTranslatorSocket(pub SocketPath);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MemoryFileName(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct OwnedRoot(pub signal_belief_nexus::RelativePath);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct GenerationsDirectory(pub signal_belief_nexus::RelativePath);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CurrentLink(pub signal_belief_nexus::RelativePath);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SettingsFile(pub signal_belief_nexus::RelativePath);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InstructionsSetting(pub String);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InstructionsFile(pub signal_belief_nexus::RelativePath);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SkillsDirectory(pub signal_belief_nexus::RelativePath);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PositiveInteger(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RetentionPolicy {
    KeepAll,
    KeepPrevious(PositiveInteger),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Layout {
    pub owned_root: OwnedRoot,
    pub generations_directory: GenerationsDirectory,
    pub current_link: CurrentLink,
    pub settings_file: SettingsFile,
    pub instructions_setting: InstructionsSetting,
    pub instructions_file: InstructionsFile,
    pub skills_directory: SkillsDirectory,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HarnessLayout {
    pub harness: signal_belief_nexus::Harness,
    pub layout: Layout,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PlacementDefault {
    pub kind: signal_belief_nexus::Kind,
    pub placement: signal_belief_nexus::Placement,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Set {
    RepositoryPolling(Timeout),
    SubscriberCapacity(i64),
    OrdinaryPeers(signal_belief_nexus::PeerList),
    MetaPeers(signal_belief_nexus::PeerList),
    Layout(HarnessLayout),
    Placement(PlacementDefault),
    GenerationRetention(RetentionPolicy),
    NexusFrameCapacity(ByteLimit),
    NexusTransportTimeout(Timeout),
    NexusSocket(SocketPath),
    MetaNexusSocket(SocketPath),
    CallTimeout(Timeout),
    OutputLimit(ByteLimit),
    TextBudget(Budget),
    TranslatorFrameCapacity(ByteLimit),
    TranslatorTransportTimeout(Timeout),
    AllowedNexusPeers(signal_belief_nexus::PeerList),
    GitExecutable(signal_belief_nexus::ExecutablePath),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ConfigureRequest {
    pub configuration_revision_option: std::option::Option<ConfigurationRevision>,
    pub set: Set,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ConfiguredState {
    pub set: Set,
    pub keeper: Keeper,
    pub configuration_revision: ConfigurationRevision,
    pub readiness: Readiness,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TranslatorConfiguration {
    pub configuration_revision: ConfigurationRevision,
    pub set_vector: std::vec::Vec<Set>,
    pub readiness: Readiness,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Invalid {
    pub keeper: Keeper,
    pub setting_name: SettingName,
    pub string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct StaleRevision {
    pub configuration_revision_option: std::option::Option<ConfigurationRevision>,
    pub configuration_revision: ConfigurationRevision,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ConfigurationRefusal {
    Invalid(Invalid),
    StaleRevision(StaleRevision),
    NotAuthorized(translator::Caller),
    Unavailable,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MetaCall {
    pub caller: translator::Caller,
    pub query: Query,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum MetaTransportInput {
    Call(MetaCall),
    RestoreTranslatorConfiguration,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum MetaTransportOutput {
    Answered(Response),
    Restored(TranslatorConfiguration),
    RestoreRefused(ConfigurationRefusal),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Admission {
    pub byte_limit: ByteLimit,
    pub timeout: Timeout,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PairedProcess {
    pub user_id: translator::UserId,
    pub group_id: translator::GroupId,
    pub process_id: translator::ProcessId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PairedTranslator {
    pub user_id: translator::UserId,
    pub group_id: translator::GroupId,
    pub process_id: translator::ProcessId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BootstrapOwner(pub signal_belief_nexus::Owner);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NexusBootstrap {
    pub ordinary_socket: OrdinarySocket,
    pub meta_socket: MetaSocket,
    pub emission_translator_socket: EmissionTranslatorSocket,
    pub place: translator::Place,
    pub memory_file_name: MemoryFileName,
    pub admission: Admission,
    pub paired_translator: PairedTranslator,
    pub bootstrap_owner: BootstrapOwner,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TranslatorBootstrap {
    pub ordinary_translator_socket: OrdinaryTranslatorSocket,
    pub meta_translator_socket: MetaTranslatorSocket,
    pub ordinary_nexus_socket: OrdinaryNexusSocket,
    pub meta_nexus_socket_path: MetaNexusSocketPath,
    pub admission: Admission,
    pub paired_process: PairedProcess,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Bootstrap {
    Nexus(NexusBootstrap),
    Translator(TranslatorBootstrap),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Configure(ConfigureRequest),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    Configured(ConfiguredState),
    ConfigureRefused(ConfigurationRefusal),
}
const ETHOS: &str = include_str!("../signal.ethos");

impl signal::Contracted for Query {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}

/// Why a Nexus bootstrap is refused before it has any effect.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BootstrapRefusal {
    UnsafePlaceName(String),
    RelativePlace(String),
    UnnormalizedPlace(String),
    UnsafeFileName(String),
    LegacyExtension(String),
    SocketOutsideRuntime(String),
    SocketsNotDistinct,
    OwnerNotEffectiveUser,
    PairedUserNotEffectiveUser,
    InvalidAdmission,
    EnvelopeMalformed,
    EnvelopeMismatch,
}

/// A bootstrap that names a prepared Place, checked before any effect.
pub trait Validating {
    fn validated(&self, effective_user: i64) -> Result<(), BootstrapRefusal>;
    /// The admission limits alone: both positive. `validated` includes it.
    fn validated_admission(&self) -> Result<(), BootstrapRefusal>;
}

/// The one memory path a bootstrap names: the Place state joined with the file name.
pub trait Locating {
    fn memory_path(&self) -> std::path::PathBuf;
}

/// The frame that precedes a bootstrap on its pipe, naming this contract.
pub trait Enveloped {
    fn envelope() -> Vec<u8>;
    fn admit(bytes: &[u8]) -> Result<(), BootstrapRefusal>;
}

/// One raw path component: exactly one normal component equal to the text, no NUL. The one rule
/// for every file and socket name a bootstrap or a deployment carries.
pub trait Component {
    fn is_one_component(&self) -> bool;
}

impl Component for str {
    fn is_one_component(&self) -> bool {
        let mut parts = std::path::Path::new(self).components();
        matches!(parts.next(), Some(std::path::Component::Normal(part)) if part == self)
            && parts.next().is_none()
            && !self.contains('\0')
    }
}

fn within(runtime: &str, socket: &SocketPath) -> Result<(), BootstrapRefusal> {
    let remainder = if runtime == "/" {
        socket.0.strip_prefix('/')
    } else {
        socket.0.strip_prefix(runtime).and_then(|rest| rest.strip_prefix('/'))
    };
    let named = remainder.is_some_and(Component::is_one_component);
    if named {
        Ok(())
    } else {
        Err(BootstrapRefusal::SocketOutsideRuntime(socket.0.clone()))
    }
}

fn normalized_absolute(value: &str) -> bool {
    value.starts_with('/')
        && !value.contains('\0')
        && (value == "/"
            || value[1..]
                .split('/')
                .all(|segment| !segment.is_empty() && segment != "." && segment != ".."))
}

impl Validating for NexusBootstrap {
    fn validated_admission(&self) -> Result<(), BootstrapRefusal> {
        if self.admission.byte_limit.0 <= 0 || self.admission.timeout.0 <= 0 {
            return Err(BootstrapRefusal::InvalidAdmission);
        }
        Ok(())
    }

    fn validated(&self, effective_user: i64) -> Result<(), BootstrapRefusal> {
        self.validated_admission()?;
        translator::placement::validate_place_name(&self.place.place_name.0)
            .map_err(|_| BootstrapRefusal::UnsafePlaceName(self.place.place_name.0.clone()))?;
        for path in [&self.place.runtime.0, &self.place.state.0] {
            if !std::path::Path::new(path).is_absolute() {
                return Err(BootstrapRefusal::RelativePlace(path.clone()));
            }
            if !normalized_absolute(path) {
                return Err(BootstrapRefusal::UnnormalizedPlace(path.clone()));
            }
        }
        let runtime = self.place.runtime.0.as_str();
        if !self.memory_file_name.0.is_one_component() {
            return Err(BootstrapRefusal::UnsafeFileName(self.memory_file_name.0.clone()));
        }
        if std::path::Path::new(&self.memory_file_name.0).with_extension("sema").as_os_str()
            == std::ffi::OsStr::new(&self.memory_file_name.0)
        {
            return Err(BootstrapRefusal::LegacyExtension(self.memory_file_name.0.clone()));
        }
        within(runtime, &self.ordinary_socket.0)?;
        within(runtime, &self.meta_socket.0)?;
        within(runtime, &self.emission_translator_socket.0)?;
        if self.ordinary_socket.0 == self.meta_socket.0
            || self.ordinary_socket.0 == self.emission_translator_socket.0
            || self.meta_socket.0 == self.emission_translator_socket.0
        {
            return Err(BootstrapRefusal::SocketsNotDistinct);
        }
        match &self.bootstrap_owner.0 {
            signal_belief_nexus::Owner::Login(user) if user.0 == effective_user => {}
            _ => return Err(BootstrapRefusal::OwnerNotEffectiveUser),
        }
        if self.paired_translator.user_id.0 != effective_user {
            return Err(BootstrapRefusal::PairedUserNotEffectiveUser);
        }
        Ok(())
    }
}

impl Locating for NexusBootstrap {
    fn memory_path(&self) -> std::path::PathBuf {
        std::path::Path::new(&self.place.state.0).join(&self.memory_file_name.0)
    }
}

impl Enveloped for NexusBootstrap {
    fn envelope() -> Vec<u8> {
        <Query as signal::Contracted>::contract_digest().0.to_le_bytes().to_vec()
    }

    fn admit(bytes: &[u8]) -> Result<(), BootstrapRefusal> {
        let bytes: [u8; 8] = bytes.try_into().map_err(|_| BootstrapRefusal::EnvelopeMalformed)?;
        if bytes == <Self as Enveloped>::envelope().as_slice() {
            Ok(())
        } else {
            Err(BootstrapRefusal::EnvelopeMismatch)
        }
    }
}
