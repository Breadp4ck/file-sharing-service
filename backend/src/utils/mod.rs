mod error;

use error::{Error, Result};
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn save_file(path: &std::path::Path, data: &[u8]) -> Result<()> {
    File::create(path)
        .await
        .map_err(|err| Error::SaveFileError(err.to_string()))?
        .write_all(data)
        .await
        .map_err(|err| Error::SaveFileError(err.to_string()))
}
