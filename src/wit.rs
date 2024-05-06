use zed_extension_api::{register_extension, Extension, LanguageServerId, Worktree};

struct WitExtension;

impl Extension for WitExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        // User should install the language server with
        // `cargo install --git https://github.com/Michael-F-Bryan/wit-lsp.git wit-cli`
        // They must have Cargo installed first
        let path = worktree
            .which("wit")
            .ok_or_else(|| "wit-cli must be installed and available on your $PATH".to_string())?;

        Ok(zed_extension_api::Command {
            command: path,
            args: vec!["serve".to_string()],
            env: Default::default(),
        })
    }
}

register_extension!(WitExtension);
