use zed_extension_api::{
    lsp::{Completion, CompletionKind},
    register_extension, CodeLabel, CodeLabelSpan, Extension, LanguageServerId, Worktree,
};

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

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        // https://github.com/Michael-F-Bryan/wit-lsp/blob/main/crates/wit-language-server/src/ops/completion.rs#L110
        let highlight_name = match completion.kind? {
            CompletionKind::Struct | CompletionKind::Interface | CompletionKind::Module => {
                Some("type".to_string())
            }
            CompletionKind::Keyword => Some("keyword".to_string()),
            _ => None,
        };

        Some(CodeLabel {
            spans: vec![CodeLabelSpan::literal(&completion.label, highlight_name)],
            filter_range: (0..completion.label.len()).into(),
            code: completion.label,
        })
    }
}

register_extension!(WitExtension);
