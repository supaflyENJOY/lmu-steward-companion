//! LMU Watch API client implementation

use crate::lmu_rest_api::{
    error::{LmuApiError, LmuApiResult},
    types::{
        ReplayFolder, ReplayInfo, ReplayRename, ReplayVisibility, SessionInfo, VcrCommand, Vehicle,
        Waypoint,
    },
};
use reqwest::Client;
use tauri::http::HeaderMap;

/// LMU Watch API client
#[derive(Debug, Clone)]
pub struct LmuWatchApi {
    client: Client,
    base_url: String,
}

#[allow(dead_code)]
impl LmuWatchApi {
    /// Create a new LMU Watch API client
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::builder()
                .user_agent("LMU Watch API Client")
                .default_headers({
                    let mut headers = HeaderMap::new();
                    headers.insert("Content-Type", "application/json".parse().unwrap());
                    headers.insert("accept", "application/json".parse().unwrap()); 
                    headers.insert("Accept-Encoding", "gzip, deflate, br, zstd".parse().unwrap());
                    headers.insert("Accept-Language", "en-US,en;q=0.9,uk;q=0.8".parse().unwrap());
                    headers.insert("Cache-Control", "no-cache".parse().unwrap());
                    headers.insert("Connection", "keep-alive".parse().unwrap());
                    headers.insert("Content-Length", "0".parse().unwrap());
                    headers.insert("Host", "localhost:6397".parse().unwrap());
                    headers.insert("Origin", "http://localhost:6397".parse().unwrap());
                    headers.insert("Pragma", "no-cache".parse().unwrap());
                    headers.insert("Referer", "http://localhost:6397/swagger/index.html".parse().unwrap());
                    headers.insert("Sec-Fetch-Dest", "empty".parse().unwrap());
                    headers.insert("Sec-Fetch-Mode", "cors".parse().unwrap());
                    headers.insert("Sec-Fetch-Site", "same-origin".parse().unwrap());
                    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36".parse().unwrap()); 
                    headers.insert("sec-ch-ua", "\"Google Chrome\";v=\"137\", \"Chromium\";v=\"137\", \"Not/A)Brand\";v=\"24\"".parse().unwrap());
                    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
                    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
                    headers
                })
                .build()
                .unwrap(),
            base_url: base_url.into(),
        }
    }

    /// Create a new LMU Watch API client with custom reqwest client
    pub fn with_client(client: Client, base_url: impl Into<String>) -> Self {
        Self {
            client,
            base_url: base_url.into(),
        }
    }

    /// Get the index of the active camera
    pub async fn get_active_camera(&self) -> LmuApiResult<i32> {
        let url = format!("{}/rest/watch/activeCamera", self.base_url);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let camera_index: i32 = response.json().await?;
            Ok(camera_index)
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Get the slot ID of the car that has focus (-1 if no car has focus)
    pub async fn get_focus(&self) -> LmuApiResult<i32> {
        let url = format!("{}/rest/watch/focus", self.base_url);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let focus_slot: i32 = response.json().await?;
            Ok(focus_slot)
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Change the camera focus to a specific type of camera in a specific group
    pub async fn set_focus_camera(
        &self,
        camera_type: &str,
        track_side_group: &str,
        should_advance: bool,
    ) -> LmuApiResult<()> {
        let url = format!(
            "{}/rest/watch/focus/{}/{}/{}",
            self.base_url, camera_type, track_side_group, should_advance
        );
        let response = self.client.put(&url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Get focus camera options
    pub async fn get_focus_camera_options(
        &self,
        camera_type: &str,
        track_side_group: &str,
        should_advance: bool,
    ) -> LmuApiResult<serde_json::Value> {
        let url = format!(
            "{}/rest/watch/focus/{}/{}/{}",
            self.base_url, camera_type, track_side_group, should_advance
        );
        let response = self
            .client
            .request(reqwest::Method::from_bytes(b"OPTIONS").unwrap(), &url)
            .send()
            .await?;

        if response.status().is_success() {
            let options: serde_json::Value = response.json().await?;
            Ok(options)
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Focus the camera on a specific slot (car)
    pub async fn set_focus_slot(&self, slot_id: i32) -> LmuApiResult<()> {
        let url = format!("{}/rest/watch/focus/{}", self.base_url, slot_id);
        let response = self.client.put(&url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Get focus slot options
    pub async fn get_focus_slot_options(&self, slot_id: i32) -> LmuApiResult<serde_json::Value> {
        let url = format!("{}/rest/watch/focus/{}", self.base_url, slot_id);
        let response = self
            .client
            .request(reqwest::Method::from_bytes(b"OPTIONS").unwrap(), &url)
            .send()
            .await?;

        if response.status().is_success() {
            let options: serde_json::Value = response.json().await?;
            Ok(options)
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Move the camera focus backward
    pub async fn focus_backward(&self) -> LmuApiResult<()> {
        let url = format!("{}/rest/watch/focusBackward", self.base_url);
        let response = self.client.put(&url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Move the camera focus forward
    pub async fn focus_forward(&self) -> LmuApiResult<()> {
        let url = format!("{}/rest/watch/focusForward", self.base_url);
        let response = self.client.put(&url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Get play information by ID
    pub async fn play(&self, id: &str) -> LmuApiResult<serde_json::Value> {
        let url = format!("{}/rest/watch/play/{}", self.base_url, id);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let play_info: serde_json::Value = response.json().await?;
            Ok(play_info)
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Check if replay is active
    pub async fn is_replay_active(&self) -> LmuApiResult<bool> {
        let url = format!("{}/rest/watch/replay/isActive", self.base_url);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let is_active: bool = response.json().await?;
            Ok(is_active)
        } else {
            Ok(false)
        }
    }

    /// Toggle replay active state
    pub async fn toggle_replay_active(&self) -> LmuApiResult<()> {
        let url = format!("{}/rest/watch/replay/toggleActive", self.base_url);
        let response = self.client.put(&url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Delete a replay by ID
    pub async fn delete_replay(&self, id: &str) -> LmuApiResult<()> {
        let url = format!("{}/rest/watch/replay/{}", self.base_url, id);
        let response = self.client.delete(&url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Issue a specific replay command (like play, pause, slowmotion, etc.)
    pub async fn send_replay_command(&self, command: &str) -> LmuApiResult<()> {
        let url = format!("{}/rest/watch/replayCommand/{}", self.base_url, command);
        let response = self.client.put(&url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Issue a VCR command using the type-safe enum
    pub async fn send_vcr_command(&self, command: VcrCommand) -> LmuApiResult<()> {
        self.send_replay_command(command.as_str()).await
    }

    /// Jump to a specific event time in the replay
    pub async fn set_replay_time(&self, event_time: i64) -> LmuApiResult<()> {
        let url = format!("{}/rest/watch/replaytime/{}", self.base_url, event_time);

        println!("Setting replay time to {}", url);

        let response = self.client.put(&url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Get list of available replays
    pub async fn get_replays(&self) -> LmuApiResult<Vec<ReplayInfo>> {
        let url = format!("{}/rest/watch/replays", self.base_url);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let replays: Vec<ReplayInfo> = response.json().await?;
            Ok(replays)
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Get session information
    pub async fn get_session_info(&self) -> LmuApiResult<SessionInfo> {
        let url = format!("{}/rest/watch/sessionInfo", self.base_url);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let session_info: SessionInfo = response.json().await?;
            Ok(session_info)
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Get current standings
    pub async fn get_standings(&self) -> LmuApiResult<Vec<Vehicle>> {
        let url = format!("{}/rest/watch/standings", self.base_url);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let standings: Vec<Vehicle> = response.json().await?;
            Ok(standings)
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Get replay metadata from a specific replay file
    pub async fn get_replay_metadata_from_file(&self, id: &str) -> LmuApiResult<serde_json::Value> {
        let url = format!(
            "{}/rest/watch/replay/getMetadataFromFile/{}",
            self.base_url, id
        );
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let metadata: serde_json::Value = response.json().await?;
            Ok(metadata)
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Get replay folder information
    pub async fn get_replay_folder(&self) -> LmuApiResult<String> {
        let url = format!("{}/rest/watch/replay/getReplayFolder/", self.base_url);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let folder: String = response.json().await?;
            Ok(folder)
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Rename a replay
    pub async fn rename_replay(&self, id: &str, new_name: &str) -> LmuApiResult<()> {
        let url = format!("{}/rest/watch/replay/rename/{}", self.base_url, id);
        let body = ReplayRename {
            new_name: new_name.to_string(),
        };
        let response = self.client.put(&url).json(&body).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Set current JSON metadata for replays
    pub async fn set_current_metadata(&self, metadata: serde_json::Value) -> LmuApiResult<()> {
        let url = format!("{}/rest/watch/replay/setCurrentMetadata/", self.base_url);
        let response = self.client.put(&url).json(&metadata).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Set the directory where replay files are stored
    pub async fn set_replay_folder(&self, folder: &str) -> LmuApiResult<()> {
        let url = format!("{}/rest/watch/replay/setReplayFolder/", self.base_url);
        let body = ReplayFolder {
            folder: folder.to_string(),
        };
        let response = self.client.put(&url).json(&body).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Set the visibility of the replay UI
    pub async fn set_replay_ui_visible(&self, visible: bool) -> LmuApiResult<()> {
        let url = format!("{}/rest/watch/replay/setReplayUIVisible/", self.base_url);
        let body = ReplayVisibility { visible };
        let response = self.client.post(&url).json(&body).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Get replay information
    pub async fn get_replay_info(&self) -> LmuApiResult<serde_json::Value> {
        let url = format!("{}/rest/watch/replayInfo", self.base_url);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let replay_info: serde_json::Value = response.json().await?;
            Ok(replay_info)
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Get standings history
    pub async fn get_standings_history(&self) -> LmuApiResult<serde_json::Value> {
        let url = format!("{}/rest/watch/standings/history", self.base_url);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let history: serde_json::Value = response.json().await?;
            Ok(history)
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Get track map waypoints for drawing a track map
    pub async fn get_trackmap(&self) -> LmuApiResult<Vec<Waypoint>> {
        let url = format!("{}/rest/watch/trackmap", self.base_url);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let trackmap: Vec<Waypoint> = response.json().await?;
            Ok(trackmap)
        } else {
            Err(LmuApiError::Api {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            })
        }
    }
}

/// Convenience functions for common operations
impl LmuWatchApi {
    /// Create a client configured for localhost on the default LMU port
    pub fn localhost() -> Self {
        Self::new("http://localhost:6397")
    }

    /// Create a client configured for a custom host and port
    #[allow(dead_code)]
    pub fn custom_host(host: &str, port: u16) -> Self {
        Self::new(format!("http://{}:{}", host, port))
    }

    /// Check if the API is reachable by getting session info
    #[allow(dead_code)]
    pub async fn health_check(&self) -> LmuApiResult<bool> {
        match self.get_session_info().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_creation() {
        let api = LmuWatchApi::localhost();
        assert_eq!(api.base_url, "http://localhost:6397");
    }

    #[tokio::test]
    async fn test_custom_host() {
        let api = LmuWatchApi::custom_host("192.168.1.100", 8080);
        assert_eq!(api.base_url, "http://192.168.1.100:8080");
    }
}
