use std::path::Path;

use tokio::io::AsyncWriteExt;

/// Creates `dir` if needed and restricts it to its owner. Generated PDFs are
/// written here, so other local users must not be able to list or replace them.
pub async fn create_private_dir(dir: &Path) -> std::io::Result<()> {
    tokio::fs::create_dir_all(dir).await?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        tokio::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700)).await?;
    }

    Ok(())
}

/// Writes a new file with owner-only permissions.
///
/// `create_new` refuses to follow or reuse an existing path, so a name that
/// collides with an attacker-planted file or symlink fails instead of
/// overwriting it.
pub async fn write_private_file(path: &Path, contents: &[u8]) -> std::io::Result<()> {
    let mut options = tokio::fs::OpenOptions::new();
    options.write(true).create_new(true);

    #[cfg(unix)]
    {
        options.mode(0o600);
    }

    let mut file = options.open(path).await?;
    file.write_all(contents).await?;
    file.flush().await?;

    Ok(())
}
