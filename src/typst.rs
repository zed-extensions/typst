use std::fs;
use zed_extension_api::{self as zed, settings::LspSettings, Result};

struct TypstExtension {
    cached_binary_path: Option<String>,
}

#[derive(Clone)]
struct TinymistBinary {
    path: String,
    args: Option<Vec<String>>,
    environment: Option<Vec<(String, String)>>,
}

impl TypstExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<TinymistBinary> {
        let binary_settings = LspSettings::for_worktree("tinymist", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|settings| settings.arguments.clone());
        if let Some(path) = worktree.which("tinymist") {
            let env = worktree.shell_env();
            return Ok(TinymistBinary {
                path,
                args: binary_args,
                environment: Some(env),
            });
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(TinymistBinary {
                    path: path.clone(),
                    args: binary_args,
                    environment: None,
                });
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "Myriad-Dreamin/tinymist",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let mut asset_name = format!(
            "tinymist-{os}-{arch}",
            arch = match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X86 => "x86",
                zed::Architecture::X8664 => "x64",
            },
            os = match platform {
                zed::Os::Mac => "darwin",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "win32",
            },
        );

        if platform == zed::Os::Windows {
            asset_name = format!("{}.exe", asset_name);
        }

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("tinymist-{}", release.version);
        fs::create_dir_all(&version_dir).map_err(|e| format!("failed to create directory: {e}"))?;

        let binary_path = format!("{version_dir}/tinymist");

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &binary_path,
                zed::DownloadedFileType::Uncompressed,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(TinymistBinary {
            path: binary_path,
            args: binary_args,
            environment: None,
        })
    }
}

fn normalize_tinymist_settings(mut settings: zed::serde_json::Value) -> zed::serde_json::Value {
    let Some(settings_object) = settings.as_object_mut() else {
        return settings;
    };

    if let Some(tinymist_settings) = settings_object
        .get("tinymist")
        .and_then(|settings| settings.as_object())
        .cloned()
    {
        for (key, value) in tinymist_settings {
            settings_object.entry(key).or_insert(value);
        }
    }

    if !settings_object.contains_key("tinymist") {
        settings_object.insert(
            "tinymist".to_string(),
            zed::serde_json::Value::Object(settings_object.clone()),
        );
    }

    settings
}

impl zed::Extension for TypstExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let tinymist_binary = self.language_server_binary(language_server_id, worktree)?;

        Ok(zed::Command {
            command: tinymist_binary.path,
            args: tinymist_binary
                .args
                .unwrap_or_else(|| vec!["lsp".to_string()]),
            env: tinymist_binary.environment.unwrap_or_default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let lsp_settings = LspSettings::for_worktree(server_id.as_ref(), worktree).ok();
        let initialization_options = lsp_settings
            .as_ref()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone());
        let settings = lsp_settings.and_then(|lsp_settings| lsp_settings.settings.clone());
        let options = initialization_options
            .or(settings)
            .map(normalize_tinymist_settings)
            .unwrap_or_default();
        Ok(Some(options))
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(normalize_tinymist_settings(settings)))
    }
}

zed::register_extension!(TypstExtension);

#[cfg(test)]
mod tests {
    use super::*;
    use zed::serde_json::json;

    #[test]
    fn normalizes_flat_tinymist_settings_to_sectioned_settings() {
        let settings = normalize_tinymist_settings(json!({
            "exportPdf": "onSave",
            "outputPath": "$root/$name",
            "projectResolution": "lockDatabase",
            "typstExtraArgs": ["main.typ"]
        }));

        assert_eq!(settings["exportPdf"], "onSave");
        assert_eq!(settings["tinymist"]["exportPdf"], "onSave");
        assert_eq!(settings["tinymist"]["typstExtraArgs"], json!(["main.typ"]));
    }

    #[test]
    fn normalizes_sectioned_tinymist_settings_to_flat_settings() {
        let settings = normalize_tinymist_settings(json!({
            "tinymist": {
                "exportPdf": "onSave",
                "typstExtraArgs": ["main.typ"]
            }
        }));

        assert_eq!(settings["exportPdf"], "onSave");
        assert_eq!(settings["typstExtraArgs"], json!(["main.typ"]));
        assert_eq!(settings["tinymist"]["exportPdf"], "onSave");
    }
}
