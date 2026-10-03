use std::fs;
use zed_extension_api::{self as zed, serde_json::Value, settings::LspSettings, Result, Worktree};

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
        );

        let release = match release {
            Ok(r) => r,
            Err(e) => {
                if let Ok(path) = fs::read_to_string("latest-binary") {
                    if fs::metadata(&path).is_ok_and(|stat| stat.is_file()) {
                        return Ok(TinymistBinary {
                            path,
                            args: binary_args,
                            environment: None,
                        });
                    }
                }
                return Err(e);
            }
        };

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

            let _ = fs::write("latest-binary", &binary_path);
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(TinymistBinary {
            path: binary_path,
            args: binary_args,
            environment: None,
        })
    }
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
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| {
                lsp_settings
                    .initialization_options
                    .map(|options| substitute_paths(options, worktree))
            })
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| {
                lsp_settings
                    .settings
                    .map(|settings| substitute_paths(settings, worktree))
            })
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

/// Recursively inject worktree root "variable" into string values when found.
/// Other [task variables](https://zed.dev/docs/tasks#variables) and
/// `${ZED_WORKTREE_ROOT:default}` syntax are unsupported.
fn substitute_paths(value: Value, worktree: &Worktree) -> Value {
    match value {
        Value::String(s) => Value::String(
            s.replace("$ZED_WORKTREE_ROOT", &worktree.root_path())
                .replace("${ZED_WORKTREE_ROOT}", &worktree.root_path()),
        ),
        Value::Array(values) => values
            .into_iter()
            .map(|v| substitute_paths(v, worktree))
            .collect(),
        Value::Object(map) => map
            .into_iter()
            .map(|(k, v)| (k, substitute_paths(v, worktree)))
            .collect(),
        other => other,
    }
}

zed::register_extension!(TypstExtension);
