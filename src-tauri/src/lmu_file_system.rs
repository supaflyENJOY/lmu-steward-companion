use std::path::PathBuf;
use steamlocate::SteamDir;

pub fn get_lmu_path() -> Option<PathBuf> {
    let steamdir = SteamDir::locate().ok()?;
    for library in steamdir.libraries().ok()?.flatten() {
        for app in library.apps().flatten() {
            if app.app_id == 2_399_420 {
                let app_path = library
                    .path()
                    .join("steamapps\\common")
                    .join(app.install_dir);
                return Some(app_path);
            }
        }
    }
    None
}
