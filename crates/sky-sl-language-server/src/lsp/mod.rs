use camino::Utf8PathBuf;
use tower_lsp::jsonrpc::*;
use tower_lsp::lsp_types::Url;

pub fn url_to_path(url: &Url) -> Result<Utf8PathBuf> {
    let path = url.to_file_path().map_err(|_| Error {
        code: ErrorCode::InvalidParams,
        message: "Cannot convert Url to Path".to_string(),
        data: None,
    })?;

    Utf8PathBuf::from_path_buf(path).map_err(|_| Error {
        code: ErrorCode::InvalidParams,
        message: "Cannot convert Url to Path".to_string(),
        data: None,
    })
}
