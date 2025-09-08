#[derive(Debug, Serialize, Deserialize)]
pub struct RFactorXml {
    #[serde(rename = "RaceResults")]
    pub race_results: RaceResults,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RaceResults {
    #[serde(rename = "Setting")]
    pub setting: String,
    #[serde(rename = "PlayerFile")]
    pub player_file: String,
    #[serde(rename = "DateTime")]
    pub date_time: String,
    #[serde(rename = "TimeString")]
    pub time_string: String,
    #[serde(rename = "TrackVenue")]
    pub track_venue: String,
    #[serde(rename = "TrackCourse")]
    pub track_course: String,
    #[serde(rename = "TrackEvent")]
    pub track_event: String,
    #[serde(rename = "TrackData")]
    pub track_data: String,
    #[serde(rename = "TrackLength")]
    pub track_length: f32,
    #[serde(rename = "GameVersion")]
    pub game_version: String,
    #[serde(rename = "Dedicated")]
    pub dedicated: u32,
    #[serde(rename = "ConnectionType")]
    pub connection_type: ConnectionType,
    #[serde(rename = "RaceLaps")]
    pub race_laps: u32,
    #[serde(rename = "RaceTime")]
    pub race_time: u32,
    #[serde(rename = "MechFailRate")]
    pub mech_fail_rate: u32,
    #[serde(rename = "DamageMult")]
    pub damage_mult: u32,
    #[serde(rename = "FuelMult")]
    pub fuel_mult: u32,
    #[serde(rename = "TireMult")]
    pub tire_mult: u32,
    #[serde(rename = "VehiclesAllowed")]
    pub vehicles_allowed: String,
    #[serde(rename = "ParcFerme")]
    pub parc_ferme: u32,
    #[serde(rename = "FixedSetups")]
    pub fixed_setups: u32,
    #[serde(rename = "FreeSettings")]
    pub free_settings: u32,
    #[serde(rename = "FixedUpgrades")]
    pub fixed_upgrades: u32,
    #[serde(rename = "LimitedTires")]
    pub limited_tires: Option<u32>,
    #[serde(rename = "TireWarmers")]
    pub tire_warmers: Option<u32>,
    #[serde(alias = "Race", alias = "Practice1", alias = "Qualify")]
    pub session: Session,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionType {
    #[serde(rename = "upload", default)]
    pub upload: Option<u32>,
    #[serde(rename = "download", default)]
    pub download: Option<u32>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    #[serde(rename = "DateTime")]
    pub date_time: String,
    #[serde(rename = "TimeString")]
    pub time_string: String,
    #[serde(rename = "Laps")]
    pub laps: i32,
    #[serde(rename = "Minutes")]
    pub minutes: u32,
    #[serde(rename = "Stream")]
    pub stream: Stream,
    #[serde(rename = "FormationAndStart")]
    pub formation_and_start: Option<u32>,
    #[serde(rename = "MostLapsCompleted")]
    pub most_laps_completed: u32,
    #[serde(rename = "Driver", default)]
    pub drivers: Vec<Driver>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stream {
    #[serde(rename = "$value", default)]
    pub items: Vec<StreamItem>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum StreamItem {
    Incident(Event),
    Penalty(Event),
    Score(Event),
    Sector(Event),
    TrackLimits(Event),
    Sent(Event),
    Chat(Event),
    ChatMessage(Event),
    Command(Event),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Event {
    #[serde(rename = "@et")]
    pub et: Option<f32>,
    #[serde(rename = "$text")]
    pub description: Option<String>,
    #[serde(rename = "@Driver")]
    pub driver: Option<String>,
    #[serde(rename = "@ID")]
    pub driver_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Driver {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Connected")]
    pub connected: u32,
    #[serde(rename = "VehFile")]
    pub veh_file: String,
    #[serde(rename = "UpgradeCode")]
    pub upgrade_code: String,
    #[serde(rename = "VehName")]
    pub veh_name: String,
    #[serde(rename = "Category")]
    pub category: String,
    #[serde(rename = "CarType")]
    pub car_type: String,
    #[serde(rename = "CarClass")]
    pub car_class: String,
    #[serde(rename = "CarNumber")]
    pub car_number: String,
    #[serde(rename = "TeamName")]
    pub team_name: String,
    #[serde(rename = "isPlayer")]
    pub is_player: u32,
    #[serde(rename = "ServerScored")]
    pub server_scored: u32,
    #[serde(rename = "GridPos")]
    pub grid_pos: Option<u32>,
    #[serde(rename = "Position")]
    pub position: u32,
    #[serde(rename = "ClassGridPos")]
    pub class_grid_pos: Option<u32>,
    #[serde(rename = "ClassPosition")]
    pub class_position: u32,
    #[serde(rename = "LapRankIncludingDiscos")]
    pub lap_rank_including_discos: u32,
    #[serde(rename = "Laps")]
    pub laps: u32,
    #[serde(rename = "Pitstops")]
    pub pitstops: u32,
    #[serde(rename = "FinishStatus")]
    pub finish_status: String,
    #[serde(rename = "DNFReason")]
    pub dnf_reason: Option<String>,
}

use chrono::{DateTime, Local};
use std::path::PathBuf;

/// Represents a .Vcr file from the replays folder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VcrFile {
    pub id: usize,
    pub file_path: PathBuf,
    pub file_name: String,
    #[serde(
        serialize_with = "serialize_datetime",
        deserialize_with = "deserialize_datetime"
    )]
    pub modified_date: DateTime<Local>,
    pub size: u64,
}

/// Matched replay and its result file
#[derive(Debug, Serialize, Deserialize)]
pub struct MatchedReplayResult {
    pub replay: VcrFile,
    #[serde(
        serialize_with = "serialize_pathbuf",
        deserialize_with = "deserialize_pathbuf"
    )]
    pub results_file_name: PathBuf,
}

// Helper functions for PathBuf serialization
use serde::{Deserializer, Serializer};

fn serialize_pathbuf<S>(path: &PathBuf, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&path.to_string_lossy())
}

fn deserialize_pathbuf<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(PathBuf::from(s))
}

/// Formatted replay with date information
#[derive(Debug)]
pub struct FormattedReplay {
    pub replay: VcrFile,
    pub date: DateTime<Local>,
}

/// Result file with formatted date
#[derive(Debug)]
pub struct FormattedResult {
    pub date: DateTime<Local>,
    pub file_name: PathBuf,
}

/// Contact incident between players
#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub players: Vec<u32>,
    pub distance: f32,
    pub et: f32,
}

// Helper functions for DateTime serialization
fn serialize_datetime<S>(dt: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&dt.to_rfc3339())
}

fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(&s)
        .map(|dt| dt.with_timezone(&Local))
        .map_err(serde::de::Error::custom)
}
