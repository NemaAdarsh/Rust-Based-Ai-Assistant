use tower_lsp::{Client, LanguageServer, LspService, Server};
use tokio::io::{stdin, stdout};

struct CodeAssistant;




#[tower_lsp::async_trait]
impl LanguageServer for CodeAssistant {
    async fn initialize(&self, _: tower_lsp::lsp_types::InitializeParams) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }
}










pub async fn run_lsp_server() {
    let stdin = stdin();
    let stdout = stdout();
    let (service, socket) = LspService::new(|_client| CodeAssistant);
    Server::new(stdin, stdout, socket).serve(service).await;
}
