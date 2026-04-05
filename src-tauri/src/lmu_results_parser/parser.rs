use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::lmu_results_parser::types::{RFactorXml, RaceResults};
use quick_xml::de::{from_reader, DeError};

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("XML deserialization error: {0}")]
    De(#[from] DeError),
}

/// Parses a Le Mans Ultimate results XML file into a RaceResults struct.
pub fn parse_results_xml<P: AsRef<Path>>(path: P) -> Result<RaceResults, ParseError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let root: RFactorXml = from_reader(reader)?;
    Ok(root.race_results)
}
use crate::lmu_file_system;
use crate::lmu_rest_api::types::Vehicle;
use crate::lmu_rest_api::LmuWatchApi;
use crate::lmu_results_parser::types::{
    Contact, FormattedReplay, FormattedResult, MatchedReplayResult, StreamItem, VcrFile,
};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use regex::Regex;
use std::collections::HashMap;

/// Finds and matches replays with their result files.
pub async fn get_races(
    _watch_api: &LmuWatchApi,
) -> Result<Vec<MatchedReplayResult>, Box<dyn std::error::Error>> {
    let lmu_path = lmu_file_system::get_lmu_path();
    let replays_folder = lmu_path.as_ref().unwrap().join("UserData/Replays");

    // Read .Vcr files from the replays folder
    let mut replays = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&replays_folder) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension.to_string_lossy().to_lowercase() == "vcr" {
                    if let Ok(metadata) = std::fs::metadata(&path) {
                        let modified_time = metadata.modified()?;
                        let modified_datetime: DateTime<Local> = DateTime::from(modified_time);

                        let file_name = path
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or("unknown")
                            .to_string();

                        replays.push(FormattedReplay {
                            replay: VcrFile {
                                id: 0, // Temporary ID, will be set after sorting
                                file_path: path.clone(),
                                file_name,
                                modified_date: modified_datetime,
                                size: metadata.len(),
                            },
                            date: modified_datetime,
                        });
                    }
                }
            }
        }
    }

    // Sort replays from newest to oldest
    replays.sort_by_key(|replay| replay.date);
    replays.reverse();

    // Assign IDs after sorting (newest = 0, oldest = n-1)
    for (index, replay) in replays.iter_mut().enumerate() {
        replay.replay.id = index;
    }

    let results_folder = lmu_path.unwrap().join("UserData/Log/Results");
    let files = std::fs::read_dir(&results_folder).map_err(|e| {
        format!(
            "Failed to read results folder at '{}': {}",
            results_folder.display(),
            e
        )
    })?;
    let mut formatted_results = files
        .flatten()
        .map(|f| f.path())
        .filter(|p| p.extension().and_then(|ext| ext.to_str()).unwrap_or("") == "xml")
        .map(|file| {
            let file_name = file.file_name().unwrap().to_str().unwrap();
            let date = file_name.split('-').next().unwrap().trim();
            let date = NaiveDateTime::parse_from_str(date, "%Y_%m_%d_%H_%M_%S").unwrap();
            FormattedResult {
                date: Local.from_local_datetime(&date).unwrap(),
                file_name: file,
            }
        })
        .collect::<Vec<_>>();

    formatted_results.sort_by_key(|result| result.date);
    formatted_results.reverse();

    let matched_replays = replays
        .iter()
        .filter_map(|replay| {
            let formatted_result = formatted_results
                .iter()
                .find(|result| result.date < replay.date);

            formatted_result.map(|result| MatchedReplayResult {
                replay: replay.replay.clone(),
                results_file_name: result.file_name.clone(),
            })
        })
        .collect::<Vec<_>>();

    Ok(matched_replays)
}

/// Parses race results from a matched replay.
pub fn parse_race_results(matched_replay: &MatchedReplayResult) -> Result<RaceResults, ParseError> {
    let results = parse_results_xml(matched_replay.results_file_name.clone())?;
    Ok(results)
}

/// Extracts contact incidents from race results.
pub fn get_race_contacts(
    results: &RaceResults,
    include_solo_incidents: bool,
) -> Result<Vec<Contact>, regex::Error> {
    let incidents = results.session.stream.items.iter().filter_map(|item| {
        if let StreamItem::Incident(incident) = item {
            Some(incident)
        } else {
            None
        }
    });

    let mut contacts: Vec<Contact> = Vec::new();
    let re = Regex::new(r"\((\d+(\.\d+)?)\)")?;
    for incident in incidents {
        let captures = re
            .captures_iter(incident.description.as_ref().unwrap())
            .map(|c| c[1].to_string())
            .collect::<Vec<_>>();
        let player_id = captures[0].parse::<u32>().unwrap_or(1);
        let distance = captures[1].parse::<f32>().unwrap_or(0.0);
        let second_player_id = captures.get(2).and_then(|c| c.parse::<u32>().ok());
        let et = incident.et.unwrap_or(0.0);

        if second_player_id.is_none() {
            if include_solo_incidents {
                contacts.push(Contact {
                    players: vec![player_id],
                    distance,
                    et,
                });
            }
        } else {
            let second_player_id = second_player_id.unwrap();
            let existing_contact = contacts.iter_mut().find(|c| {
                (c.players.contains(&second_player_id) || c.players.contains(&player_id))
                    && (c.et - et).abs() < 3.0
            });

            if let Some(contact) = existing_contact {
                if !contact.players.contains(&player_id) {
                    contact.players.push(player_id);
                }
                if !contact.players.contains(&second_player_id) {
                    contact.players.push(second_player_id);
                }
            } else {
                contacts.push(Contact {
                    players: vec![player_id, second_player_id],
                    distance,
                    et,
                });
            }
        }
    }

    Ok(contacts)
}

/// Estimates standings from race results when API is unavailable
/// Uses Sector and TrackLimits events to build driver_id -> driver mapping
pub fn estimate_standings_from_results(
    results: &RaceResults,
) -> Result<Vec<Vehicle>, Box<dyn std::error::Error>> {

    // Build driver_id -> driver_name mapping from Sector and TrackLimits events
    let mut driver_map: HashMap<String, String> = HashMap::new();

    // Extract from Sector events
    for item in &results.session.stream.items {
        if let StreamItem::Sector(event) = item {
            if let (Some(driver), Some(driver_id)) = (&event.driver, &event.driver_id) {
                driver_map.insert(driver_id.clone(), driver.clone());
            }
        }
    }

    // Extract from TrackLimits events
    for item in &results.session.stream.items {
        if let StreamItem::TrackLimits(event) = item {
            if let (Some(driver), Some(driver_id)) = (&event.driver, &event.driver_id) {
                driver_map.insert(driver_id.clone(), driver.clone());
            }
        }
    }


    // Create minimal Vehicle structs with essential data
    let mut vehicles = Vec::new();
    for (index, (driver_id, driver_name)) in driver_map.iter().enumerate() {
        // Find corresponding driver data if available
        let driver_data = results
            .session
            .drivers
            .iter()
            .find(|d| d.name == *driver_name);

        let vehicle = Vehicle {
            slot_id: driver_id.parse().unwrap_or(index as i32 + 1),
            driver_name: driver_name.clone(),
            vehicle_name: driver_data.map(|d| d.veh_name.clone()).unwrap_or_default(),
            laps_completed: driver_data.map(|d| d.laps as i32).unwrap_or(0),
            sector: crate::lmu_rest_api::types::Sector::Sector1, // Default
            finish_status: crate::lmu_rest_api::types::FinishStatus::FstatNone,
            lap_distance: 0.0,
            path_lateral: 0.0,
            track_edge: 0.0,
            best_sector_time1: 0.0,
            best_sector_time2: 0.0,
            best_lap_time: 0.0,
            last_sector_time1: 0.0,
            last_sector_time2: 0.0,
            last_lap_time: 0.0,
            current_sector_time1: 0.0,
            current_sector_time2: 0.0,
            pitstops: driver_data.map(|d| d.pitstops as i32).unwrap_or(0),
            penalties: 0,
            player: driver_data.map(|d| d.is_player == 1).unwrap_or(false),
            in_control: 1,
            pitting: false,
            position: driver_data
                .map(|d| d.position as i32)
                .unwrap_or(index as i32 + 1),
            car_class: driver_data.map(|d| d.car_class.clone()).unwrap_or_default(),
            time_behind_next: 0.0,
            laps_behind_next: 0,
            time_behind_leader: 0.0,
            laps_behind_leader: 0,
            lap_start_et: 0.0,
            car_position: crate::lmu_rest_api::types::CarPosition {
                position_type: 0,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            car_velocity: crate::lmu_rest_api::types::Velocity {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                velocity: 0.0,
            },
            car_acceleration: crate::lmu_rest_api::types::Velocity {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                velocity: 0.0,
            },
            headlights: false,
            pit_state: crate::lmu_rest_api::types::PitState::None,
            server_scored: driver_data.map(|d| d.server_scored == 1).unwrap_or(false),
            game_phase: crate::lmu_rest_api::types::VehicleGamePhase::Green,
            qualification: 0,
            time_into_lap: 0.0,
            estimated_lap_time: 0.0,
            pit_group: String::new(),
            flag: crate::lmu_rest_api::types::VehicleFlag::Green,
            under_yellow: false,
            count_lap_flag: crate::lmu_rest_api::types::CountLapFlag::CountLapAndTime,
            in_garage_stall: false,
            upgrade_pack: driver_data
                .map(|d| d.upgrade_code.clone())
                .unwrap_or_default(),
            pit_lap_distance: 0.0,
            best_lap_sector_time1: 0.0,
            best_lap_sector_time2: 0.0,
            steam_id: 0,
            vehicle_filename: driver_data.map(|d| d.veh_file.clone()).unwrap_or_default(),
            car_id: String::new(),
            car_number: driver_data
                .map(|d| d.car_number.clone())
                .unwrap_or_default(),
            full_team_name: driver_data.map(|d| d.team_name.clone()).unwrap_or_default(),
            has_focus: false,
            fuel_fraction: 1.0,
            attack_mode: crate::lmu_rest_api::types::AttackModeData {
                time_remaining: 0,
                total_count: 0,
                remaining_count: 0,
            },
            drs_active: false,
            focus: false,
        };

        vehicles.push(vehicle);
    }

    // Sort by position
    vehicles.sort_by_key(|v| v.position);

    Ok(vehicles)
}
