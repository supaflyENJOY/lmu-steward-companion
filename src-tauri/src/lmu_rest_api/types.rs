//! Data types for LMU REST API operations

use serde::{Deserialize, Serialize};

/// VCR commands for controlling replay functionality
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum VcrCommand {
    #[serde(rename = "VCRCOMMAND_BEGIN")]
    Begin,
    #[serde(rename = "VCRCOMMAND_END")]
    End,
    #[serde(rename = "VCRCOMMAND_REVERSESCANFAST")]
    ReverseScanFast,
    #[serde(rename = "VCRCOMMAND_REVERSESCAN")]
    ReverseScan,
    #[serde(rename = "VCRCOMMAND_PLAYBACKWARDS")]
    PlayBackwards,
    #[serde(rename = "VCRCOMMAND_SLOWBACKWARDS")]
    SlowBackwards,
    #[serde(rename = "VCRCOMMAND_STOP")]
    Stop,
    #[serde(rename = "VCRCOMMAND_SLOW")]
    Slow,
    #[serde(rename = "VCRCOMMAND_PLAY")]
    Play,
    #[serde(rename = "VCRCOMMAND_FORWARDSCAN")]
    ForwardScan,
    #[serde(rename = "VCRCOMMAND_FORWARDSCANFAST")]
    ForwardScanFast,
    #[serde(rename = "VCRCOMMAND_DROPBOOKMARK")]
    DropBookmark,
    #[serde(rename = "VCRCOMMAND_REMOVEBOOKMARK")]
    RemoveBookmark,
    #[serde(rename = "VCRCOMMAND_REMOVEALLBOOKMARKS")]
    RemoveAllBookmarks,
    #[serde(rename = "VCRCOMMAND_NEXTBOOKMARK")]
    NextBookmark,
    #[serde(rename = "VCRCOMMAND_PREVBOOKMARK")]
    PrevBookmark,
    #[serde(rename = "VCRCOMMAND_SAVEBOOKMARKS")]
    SaveBookmarks,
    #[serde(rename = "VCRCOMMAND_JUMPTODESTINATION")]
    JumpToDestination,
}

impl VcrCommand {
    /// Convert the enum variant to its string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            VcrCommand::Begin => "VCRCOMMAND_BEGIN",
            VcrCommand::End => "VCRCOMMAND_END",
            VcrCommand::ReverseScanFast => "VCRCOMMAND_REVERSESCANFAST",
            VcrCommand::ReverseScan => "VCRCOMMAND_REVERSESCAN",
            VcrCommand::PlayBackwards => "VCRCOMMAND_PLAYBACKWARDS",
            VcrCommand::SlowBackwards => "VCRCOMMAND_SLOWBACKWARDS",
            VcrCommand::Stop => "VCRCOMMAND_STOP",
            VcrCommand::Slow => "VCRCOMMAND_SLOW",
            VcrCommand::Play => "VCRCOMMAND_PLAY",
            VcrCommand::ForwardScan => "VCRCOMMAND_FORWARDSCAN",
            VcrCommand::ForwardScanFast => "VCRCOMMAND_FORWARDSCANFAST",
            VcrCommand::DropBookmark => "VCRCOMMAND_DROPBOOKMARK",
            VcrCommand::RemoveBookmark => "VCRCOMMAND_REMOVEBOOKMARK",
            VcrCommand::RemoveAllBookmarks => "VCRCOMMAND_REMOVEALLBOOKMARKS",
            VcrCommand::NextBookmark => "VCRCOMMAND_NEXTBOOKMARK",
            VcrCommand::PrevBookmark => "VCRCOMMAND_PREVBOOKMARK",
            VcrCommand::SaveBookmarks => "VCRCOMMAND_SAVEBOOKMARKS",
            VcrCommand::JumpToDestination => "VCRCOMMAND_JUMPTODESTINATION",
        }
    }
}

/// Replay command for controlling replay functionality
#[derive(Debug, Serialize, Deserialize)]
pub struct ReplayCommand {
    pub command: String,
}

/// Event time for replay navigation
#[derive(Debug, Serialize, Deserialize)]
pub struct EventTime {
    pub time: u64,
}

/// Metadata for replay operations
#[derive(Debug, Serialize, Deserialize)]
pub struct ReplayMetadata {
    pub metadata: serde_json::Value,
}

/// Replay folder configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct ReplayFolder {
    pub folder: String,
}

/// Replay visibility settings
#[derive(Debug, Serialize, Deserialize)]
pub struct ReplayVisibility {
    pub visible: bool,
}

/// Replay rename request
#[derive(Debug, Serialize, Deserialize)]
pub struct ReplayRename {
    pub new_name: String,
}

/// Focus camera configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct FocusCamera {
    pub camera_type: String,
    pub track_side_group: String,
    pub should_advance: bool,
}
/// Velocity vector (x, y, z) with additional velocity magnitude
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub velocity: f64,
}

/// Attack mode data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackModeData {
    #[serde(rename = "timeRemaining")]
    pub time_remaining: i32,
    #[serde(rename = "totalCount")]
    pub total_count: i32,
    #[serde(rename = "remainingCount")]
    pub remaining_count: i32,
}

/// Car position data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarPosition {
    #[serde(rename = "type")]
    pub position_type: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Waypoint for track map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoint {
    pub m_x: f32,
    pub m_y: f32,
    pub m_z: f32,
    pub m_type: i32,
}

/// Session info as returned by /rest/watch/sessionInfo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub player_file_name: String,
    pub ambient_temp: f64,
    pub sector_flag: Vec<String>,
    pub start_event_time: f32,
    pub session: SessionType,
    pub yellow_flag_state: YellowFlagState,
    pub raining: f64,
    pub server_name: String,
    pub track_name: String,
    pub server_port: i32,
    pub maximum_laps: i32,
    pub start_light_frame: String,
    pub current_event_time: f64,
    pub num_red_lights: String,
    pub min_path_wetness: f64,
    pub dark_cloud: f64,
    pub average_path_wetness: f64,
    pub in_realtime: bool,
    pub max_path_wetness: f64,
    pub wind_speed: Velocity,
    pub max_players: i32,
    pub player_name: String,
    pub race_completion: serde_json::Value, // java.util.Map, unknown structure
    pub end_event_time: f64,
    pub number_of_vehicles: i32,
    pub password_protected: bool,
    pub game_phase: String,
    pub lap_distance: f64,
    pub game_mode: GameMode,
    pub track_temp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SessionType {
    Testday,
    Practice1,
    Practice2,
    Practice3,
    Practice4,
    Qualify1,
    Qualify2,
    Qualify3,
    Qualify4,
    Warmup,
    Race1,
    Race2,
    Race3,
    Race4,
    Invalid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum YellowFlagState {
    Invalid,
    None,
    Pending,
    PitsClosed,
    PitLeadUp,
    PitsOpen,
    LastLap,
    Resume,
    RaceHalt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GameMode {
    Unknown,
    Server,
    Client,
    ServerAndClient,
}

/// Vehicle as returned by /rest/watch/standings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    #[serde(rename = "slotID")]
    pub slot_id: i32,
    #[serde(rename = "driverName")]
    pub driver_name: String,
    #[serde(rename = "vehicleName")]
    pub vehicle_name: String,
    #[serde(rename = "lapsCompleted")]
    pub laps_completed: i32,
    pub sector: Sector,
    #[serde(rename = "finishStatus")]
    pub finish_status: FinishStatus,
    #[serde(rename = "lapDistance")]
    pub lap_distance: f64,
    #[serde(rename = "pathLateral")]
    pub path_lateral: f64,
    #[serde(rename = "trackEdge")]
    pub track_edge: f64,
    #[serde(rename = "bestSectorTime1")]
    pub best_sector_time1: f64,
    #[serde(rename = "bestSectorTime2")]
    pub best_sector_time2: f64,
    #[serde(rename = "bestLapTime")]
    pub best_lap_time: f64,
    #[serde(rename = "lastSectorTime1")]
    pub last_sector_time1: f64,
    #[serde(rename = "lastSectorTime2")]
    pub last_sector_time2: f64,
    #[serde(rename = "lastLapTime")]
    pub last_lap_time: f64,
    #[serde(rename = "currentSectorTime1")]
    pub current_sector_time1: f64,
    #[serde(rename = "currentSectorTime2")]
    pub current_sector_time2: f64,
    pub pitstops: i32,
    pub penalties: i32,
    pub player: bool,
    #[serde(rename = "inControl")]
    pub in_control: i32,
    pub pitting: bool,
    pub position: i32,
    #[serde(rename = "carClass")]
    pub car_class: String,
    #[serde(rename = "timeBehindNext")]
    pub time_behind_next: f64,
    #[serde(rename = "lapsBehindNext")]
    pub laps_behind_next: i32,
    #[serde(rename = "timeBehindLeader")]
    pub time_behind_leader: f64,
    #[serde(rename = "lapsBehindLeader")]
    pub laps_behind_leader: i32,
    #[serde(rename = "lapStartET")]
    pub lap_start_et: f64,
    #[serde(rename = "carPosition")]
    pub car_position: CarPosition,
    #[serde(rename = "carVelocity")]
    pub car_velocity: Velocity,
    #[serde(rename = "carAcceleration")]
    pub car_acceleration: Velocity,
    pub headlights: bool,
    #[serde(rename = "pitState")]
    pub pit_state: PitState,
    #[serde(rename = "serverScored")]
    pub server_scored: bool,
    #[serde(rename = "gamePhase")]
    pub game_phase: VehicleGamePhase,
    pub qualification: i32,
    #[serde(rename = "timeIntoLap")]
    pub time_into_lap: f64,
    #[serde(rename = "estimatedLapTime")]
    pub estimated_lap_time: f64,
    #[serde(rename = "pitGroup")]
    pub pit_group: String,
    pub flag: VehicleFlag,
    #[serde(rename = "underYellow")]
    pub under_yellow: bool,
    #[serde(rename = "countLapFlag")]
    pub count_lap_flag: CountLapFlag,
    #[serde(rename = "inGarageStall")]
    pub in_garage_stall: bool,
    #[serde(rename = "upgradePack")]
    pub upgrade_pack: String,
    #[serde(rename = "pitLapDistance")]
    pub pit_lap_distance: f64,
    #[serde(rename = "bestLapSectorTime1")]
    pub best_lap_sector_time1: f64,
    #[serde(rename = "bestLapSectorTime2")]
    pub best_lap_sector_time2: f64,
    #[serde(rename = "steamID")]
    pub steam_id: i64,
    #[serde(rename = "vehicleFilename")]
    pub vehicle_filename: String,
    #[serde(rename = "carId")]
    pub car_id: String,
    #[serde(rename = "carNumber")]
    pub car_number: String,
    #[serde(rename = "fullTeamName")]
    pub full_team_name: String,
    #[serde(rename = "hasFocus")]
    pub has_focus: bool,
    #[serde(rename = "fuelFraction")]
    pub fuel_fraction: f64,
    #[serde(rename = "attackMode")]
    pub attack_mode: AttackModeData,
    #[serde(rename = "drsActive")]
    pub drs_active: bool,
    pub focus: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CountLapFlag {
    CountNeither,
    CountLapOnly,
    CountLapAndTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Sector {
    Sector1,
    Sector2,
    Sector3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PitState {
    None,
    Request,
    Entering,
    Setup,
    Stopped,
    Exiting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VehicleFlag {
    Green,
    Yellow,
    Red,
    Checkered,
    White,
    Black,
    Blue,
    Hazard,
    Unsportsmanlike,
    Meatball,
    PitClosed,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FinishStatus {
    FstatNone,
    FstatFinished,
    FstatDnf,
    FstatDq,
    FstatMaximum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VehicleGamePhase {
    Before,
    Reconnaissance,
    Walkthrough,
    Formation,
    Countdown,
    Green,
    SafetyCar,
    Red,
    Checkered,
    Invalid,
}
/// Metadata for a replay entry in get_replays
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayMetadataInfo {
    #[serde(rename = "sceneDesc")]
    pub scene_desc: String,
    pub session: ReplaySessionType,
}

/// Session type for replay metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ReplaySessionType {
    Practice,
    Race,
    Qualify,
}

/// Info for a replay entry in get_replays
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayInfo {
    pub id: i32,
    pub metadata: ReplayMetadataInfo,
    #[serde(rename = "replayDirectory")]
    pub replay_directory: String,
    #[serde(rename = "replayName")]
    pub replay_name: String,
    pub size: u64,
    pub timestamp: u64,
}
