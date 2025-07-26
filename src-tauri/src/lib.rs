mod lmu_file_system;
mod lmu_rest_api;

use lmu_rest_api::LmuWatchApi;
mod google_auth;
mod lmu_results_parser;
use google_auth::GoogleAuth;
use lmu_results_parser::export::export_contacts_to_excel;
use lmu_results_parser::export_google_sheets::export_contacts_to_google_sheets;
use lmu_results_parser::parser::{estimate_standings_from_results, get_race_contacts, get_races, parse_race_results};
use lmu_results_parser::types::MatchedReplayResult;
use serde::Serialize;
use tauri::Manager;


#[tauri::command]
async fn get_matched_replays() -> Result<Vec<MatchedReplayResult>, String> {
    let watch_api = LmuWatchApi::localhost();
    match get_races(&watch_api).await {
        Ok(replays) => Ok(replays),
        Err(e) => Err(format!("Failed to get matched replays: {}", e)),
    }
}

#[tauri::command]
async fn play_replay(replay_idx: String) -> Result<(), String> {
    let watch_api = LmuWatchApi::localhost();
    match watch_api.play(&replay_idx).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to start replay: {}", e)),
    }
}

#[tauri::command]
async fn export_contacts_to_excel_command(replay_idx: String, path: String) -> Result<(), String> {
    let watch_api = LmuWatchApi::localhost();
    let matched_replays = get_races(&watch_api)
        .await
        .map_err(|e| format!("Failed to get races: {}", e))?;

    let replay_idx = replay_idx.parse::<usize>().unwrap();
    let replay = matched_replays
        .iter()
        .find(|replay| replay.replay.id == replay_idx)
        .ok_or(format!("No replay with id {} found", replay_idx))?;
    let results =
        parse_race_results(replay).map_err(|e| format!("Failed to parse race results: {}", e))?;
    let contacts = get_race_contacts(&results, false)
        .map_err(|e| format!("Failed to get race contacts: {}", e))?;
    let standings = match watch_api.get_standings().await {
        Ok(standings) => standings,
        Err(e) => {
            match estimate_standings_from_results(&results) {
                Ok(estimated_standings) => estimated_standings,
                Err(estimation_error) => {
                    return Err(format!("Failed to get standings from API: {} and failed to estimate: {}", e, estimation_error));
                }
            }
        },
    };
    export_contacts_to_excel(&contacts, &standings, &path)
        .map_err(|e| format!("Failed to export contacts: {}", e))?;
    Ok(())
}

#[tauri::command]
async fn export_contacts_to_google_sheets_command(
    replay_idx: String,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {

    let watch_api = LmuWatchApi::localhost();

    let matched_replays = get_races(&watch_api).await
        .map_err(|e| format!("Failed to get races: {}", e))?;

    let replay_idx = replay_idx.parse::<usize>()
        .map_err(|e| format!("Failed to parse replay_idx '{}': {}", replay_idx, e))?;

    let replay = matched_replays
        .iter()
        .find(|replay| replay.replay.id == replay_idx)
        .ok_or_else(|| format!("No replay with id {} found", replay_idx))?;

    let results = parse_race_results(replay)
        .map_err(|e| format!("Failed to parse race results: {}", e))?;

    let contacts = get_race_contacts(&results, false)
        .map_err(|e| format!("Failed to get race contacts: {}", e))?;

    let standings = match watch_api.get_standings().await {
        Ok(standings) => {
            println!("[EXPORT] Successfully retrieved {} standings from API", standings.len());
            standings
        }
        Err(e) => {
            println!("[EXPORT] API call failed: {}", e);
            println!("[EXPORT] Attempting to estimate standings from results data...");
            match estimate_standings_from_results(&results) {
                Ok(estimated_standings) => {
                    println!("[EXPORT] Successfully estimated {} standings from results", estimated_standings.len());
                    estimated_standings
                }
                Err(estimation_error) => {
                    let error_msg = format!("Failed to get standings from API: {} and failed to estimate: {}", e, estimation_error);
                    println!("[EXPORT] ERROR: {}", error_msg);
                    return Err(error_msg);
                }
            }
        }
    };
    println!(
        "[EXPORT] Successfully retrieved {} standings",
        standings.len()
    );

    // Use build-time environment variables for credentials
    let client_id = env!(
        "GOOGLE_CLIENT_ID",
        "GOOGLE_CLIENT_ID environment variable not set at build time"
    );
    let client_secret = env!(
        "GOOGLE_CLIENT_SECRET",
        "GOOGLE_CLIENT_SECRET environment variable not set at build time"
    );

    let redirect_url = "http://localhost:0/callback";

    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .expect("Failed to get app data directory");
    if !std::path::Path::new(&app_data_dir).exists() {
        std::fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {e}"))?;
    }
    let token_path = app_data_dir.join("google_tokens.json");

    let google_auth = GoogleAuth::new(
        client_id,
        client_secret,
        redirect_url,
        token_path.to_str().unwrap(),
    );

    let _access_token = match google_auth.get_access_token().await {
        Ok(token) => token,
        Err(_) => {
            google_auth
                .authenticate_via_browser(app_handle.clone(), 300)
                .await
                .map_err(|e| format!("Google OAuth2 authentication failed: {e}"))?
        },
    };

    let google_auth = GoogleAuth::new(
        client_id,
        client_secret,
        redirect_url,
        token_path.to_str().unwrap(),
    );

    let result = export_contacts_to_google_sheets(&contacts, &standings, &google_auth).await;

    if let Ok(url) = &result {
        let _ = tauri_plugin_opener::open_url(url, None::<&str>);
    }

    result
}

#[derive(Serialize)]
struct ContactsAndStandings {
    contacts: Vec<lmu_results_parser::types::Contact>,
    standings: Vec<lmu_rest_api::types::Vehicle>,
}

#[tauri::command]
async fn get_contacts_for_replay(replay_idx: String) -> Result<ContactsAndStandings, String> {
    let watch_api = LmuWatchApi::localhost();
    let matched_replays = get_races(&watch_api)
        .await
        .map_err(|e| format!("Failed to get races: {}", e))?;

    let replay_idx = replay_idx.parse::<usize>().unwrap();
    let replay = matched_replays
        .iter()
        .find(|replay| replay.replay.id == replay_idx)
        .ok_or(format!("No replay with id {} found", replay_idx))?;
    let results =
        parse_race_results(replay).map_err(|e| format!("Failed to parse race results: {}", e))?;
    let contacts = get_race_contacts(&results, false)
        .map_err(|e| format!("Failed to get race contacts: {}", e))?;
    let standings = match watch_api.get_standings().await {
        Ok(standings) => standings,
        Err(e) => {
            match estimate_standings_from_results(&results) {
                Ok(estimated_standings) => estimated_standings,
                Err(estimation_error) => {
                    return Err(format!("Failed to get standings from API: {} and failed to estimate: {}", e, estimation_error));
                }
            }
        },
    };
    Ok(ContactsAndStandings {
        contacts,
        standings,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_matched_replays,
            play_replay,
            export_contacts_to_excel_command,
            export_contacts_to_google_sheets_command,
            get_contacts_for_replay,
            play_contact
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn play_contact(_replay_idx: String, player_id: u32, et: f32) -> Result<(), String> {
    let watch_api = LmuWatchApi::localhost();
    let _ = watch_api
        .send_vcr_command(lmu_rest_api::types::VcrCommand::Stop)
        .await;
    let _ = watch_api.set_replay_time((et - 5.0).floor() as i64).await;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    let _ = watch_api.set_focus_slot(player_id as i32).await;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    let _ = watch_api
        .send_vcr_command(lmu_rest_api::types::VcrCommand::Play)
        .await;
    Ok(())
}

