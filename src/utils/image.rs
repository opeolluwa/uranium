//! Upload guards for user-supplied image files.
//!
//! The codebase had no MIME or extension allowlist anywhere, so an upload
//! endpoint only ever checked that bytes arrived. These helpers are shared so
//! every image entry point applies the same rules.

use std::path::Path;

use crate::errors::service_error::ServiceError;

/// Extensions accepted for image uploads.
const ALLOWED_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp", "gif", "avif"];

/// Leading bytes that identify each supported image container.
const MAGIC_NUMBERS: &[(&[u8], &str)] = &[
    (&[0xFF, 0xD8, 0xFF], "jpeg"),
    (&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A], "png"),
    (b"GIF87a", "gif"),
    (b"GIF89a", "gif"),
    (&[0x1A, 0x45, 0xDF, 0xA3], "avif"),
];

/// Longest name we hand to the upstream uploader after sanitizing.
const MAX_FILE_NAME_LEN: usize = 120;

/// Largest image we accept, in bytes. Shared by every image entry point so the
/// limit cannot drift between them.
pub const MAX_IMAGE_SIZE_BYTES: u64 = 2 * 1024 * 1024;

/// Human-readable form of the limit, used in the rejection message.
const MAX_IMAGE_SIZE_LABEL: &str = "2 MB";

/// How many leading bytes identify a format. Long enough for every container we
/// accept: the longest is PNG's eight-byte signature.
const MAGIC_BYTES_LEN: usize = 12;

/// Rejects a file that is not a plausible image.
///
/// Checks the size, the extension, the declared content type and the magic
/// bytes. Only the magic-byte check is load-bearing — the content type is
/// client-controlled and the extension is self-declared — so the other three
/// exist to reject obviously wrong uploads with a clearer message.
///
/// `declared_path` and `contents_path` are separate because multipart parsers
/// spool the part into a temp file carrying no suffix, while the extension only
/// exists in the client's filename. Callers pass the sanitized name for the
/// extension and the temp file for the bytes.
pub fn ensure_upload_is_image(
    field: &'static str,
    declared_path: &Path,
    contents_path: &Path,
    content_type: Option<&str>,
) -> Result<(), ServiceError> {
    let size = std::fs::metadata(contents_path)
        .map_err(|e| ServiceError::OperationFailed(format!("could not read upload: {e}")))?
        .len();

    let magic = read_magic_bytes(contents_path)?;

    validate_image(field, declared_path, size, &magic, content_type)
}

/// The same checks as [`ensure_upload_is_image`], for callers that already hold
/// the bytes in memory rather than a spooled temp file.
pub fn ensure_image_bytes_are_valid(
    field: &'static str,
    declared_path: &Path,
    bytes: &[u8],
    content_type: Option<&str>,
) -> Result<(), ServiceError> {
    let magic = &bytes[..bytes.len().min(MAGIC_BYTES_LEN)];

    validate_image(
        field,
        declared_path,
        bytes.len() as u64,
        magic,
        content_type,
    )
}

/// Shared body of the image guards, cheapest check first: the size is a stat on
/// an open handle, the extension and content type are string work, and only the
/// magic-byte check touches the bytes.
///
/// A file of exactly `MAX_IMAGE_SIZE_BYTES` is allowed, so this rejects `>`
/// rather than `>=`.
fn validate_image(
    field: &'static str,
    declared_path: &Path,
    size: u64,
    magic: &[u8],
    content_type: Option<&str>,
) -> Result<(), ServiceError> {
    if size > MAX_IMAGE_SIZE_BYTES {
        return Err(ServiceError::ValidationError(image_error(
            field,
            format!("must be {MAX_IMAGE_SIZE_LABEL} or smaller"),
        )));
    }

    let extension = declared_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase)
        .unwrap_or_default();

    if !ALLOWED_EXTENSIONS.contains(&extension.as_str()) {
        return Err(ServiceError::ValidationError(image_error(
            field,
            format!("must be one of: {}", ALLOWED_EXTENSIONS.join(", ")),
        )));
    }

    if let Some(content_type) = content_type
        && !content_type.to_ascii_lowercase().starts_with("image/")
    {
        return Err(ServiceError::ValidationError(image_error(
            field,
            format!("content type '{content_type}' is not an image"),
        )));
    }

    let matches_magic = MAGIC_NUMBERS
        .iter()
        .any(|(magic_number, _)| magic.starts_with(magic_number))
        // WebP is a RIFF container: "RIFF" + 4 length bytes + "WEBP".
        || (magic.starts_with(b"RIFF") && magic.get(8..12) == Some(b"WEBP"));

    if !matches_magic {
        return Err(ServiceError::ValidationError(image_error(
            field,
            "contents do not match a supported image format".to_string(),
        )));
    }

    Ok(())
}

/// Strips any directory component from a client-supplied filename and caps its
/// length, so the name is only ever used for display and never for paths.
pub fn sanitize_file_name(file_name: Option<&str>) -> String {
    let name = file_name
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .unwrap_or("image")
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or("image")
        .trim()
        .to_string();

    if name.is_empty() || name.chars().all(|c| c == '.') {
        return "image".to_string();
    }

    name.chars().take(MAX_FILE_NAME_LEN).collect()
}

fn read_magic_bytes(path: &Path) -> Result<Vec<u8>, ServiceError> {
    use std::io::Read;

    let mut bytes = Vec::new();
    std::fs::File::open(path)
        .and_then(|file| file.take(MAGIC_BYTES_LEN as u64).read_to_end(&mut bytes))
        .map_err(|e| ServiceError::OperationFailed(format!("could not read upload: {e}")))?;

    Ok(bytes)
}

/// Builds a `ValidationErrors` carrying a single field message, reusing the
/// `validator` type the rest of the API reports through.
fn image_error(field: &'static str, message: String) -> validator::ValidationErrors {
    let mut errors = validator::ValidationErrors::new();
    errors.add(
        field,
        validator::ValidationError::new("invalid_image").with_message(message.into()),
    );
    errors
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;
    use tempfile::NamedTempFile;

    const PNG: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    const JPEG: [u8; 3] = [0xFF, 0xD8, 0xFF];
    const RIFF_WEBP: [u8; 12] = [
        b'R', b'I', b'F', b'F', 0x1A, 0, 0, 0, b'W', b'E', b'B', b'P',
    ];

    /// Writes `bytes` to a temp file with *no* suffix, mirroring what
    /// `TypedMultipart` actually hands the handler.
    fn temp_file(bytes: &[u8]) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(bytes).unwrap();
        file.flush().unwrap();
        file
    }

    /// Runs the guard the way the product handler does: the client's name
    /// carries the extension, the temp file carries the bytes.
    fn guard(
        declared_name: &str,
        file: &NamedTempFile,
        content_type: Option<&str>,
    ) -> Result<(), ServiceError> {
        ensure_upload_is_image(
            "picture",
            Path::new(declared_name),
            file.path(),
            content_type,
        )
    }

    /// Builds a PNG of exactly `len` bytes by zero-padding the signature. The
    /// guards only ever read the leading bytes, so the padding is enough to
    /// exercise the size limit without a real encoder.
    fn png_of_len(len: usize) -> Vec<u8> {
        let mut bytes = PNG.to_vec();
        bytes.resize(len, 0x00);
        bytes
    }

    #[test]
    fn accepts_png_jpeg_and_webp_by_content() {
        for (name, bytes) in [
            ("a.png", PNG.as_slice()),
            ("a.jpeg", JPEG.as_slice()),
            ("a.webp", RIFF_WEBP.as_slice()),
        ] {
            let file = temp_file(bytes);
            assert!(
                guard(name, &file, Some("image/png")).is_ok(),
                "{name} should be accepted"
            );
        }
    }

    #[test]
    fn accepts_upload_whose_temp_file_has_no_extension() {
        // The production shape: an extensionless `.tmpXXXX` spool whose format
        // is only knowable from the client's filename.
        let file = temp_file(&PNG);
        assert!(file.path().extension().is_none());
        assert!(guard("holiday photo.png", &file, Some("image/png")).is_ok());
    }

    #[test]
    fn accepts_image_at_exactly_the_size_limit() {
        // The limit is inclusive: 2 MB itself must not be rejected, so the check
        // has to be `>` and not `>=`.
        let bytes = png_of_len(MAX_IMAGE_SIZE_BYTES as usize);
        assert_eq!(bytes.len() as u64, MAX_IMAGE_SIZE_BYTES);

        let file = temp_file(&bytes);
        assert!(guard("a.png", &file, Some("image/png")).is_ok());
    }

    #[test]
    fn rejects_image_one_byte_over_the_size_limit() {
        let bytes = png_of_len(MAX_IMAGE_SIZE_BYTES as usize + 1);
        let file = temp_file(&bytes);

        let err = guard("a.png", &file, Some("image/png")).unwrap_err();
        assert_eq!(err.status_code(), axum::http::StatusCode::BAD_REQUEST);
        assert!(err.to_string().contains(MAX_IMAGE_SIZE_LABEL));
    }

    #[test]
    fn checks_size_before_extension_and_magic() {
        // Oversized *and* wrongly named: the size is the more useful complaint,
        // and the more expensive checks should not run.
        let bytes = png_of_len(MAX_IMAGE_SIZE_BYTES as usize + 1);
        let file = temp_file(&bytes);

        let err = guard("a.exe", &file, None).unwrap_err();
        assert!(err.to_string().contains(MAX_IMAGE_SIZE_LABEL));
        assert!(!err.to_string().contains("must be one of"));
    }

    #[test]
    fn in_memory_guard_matches_the_file_guard() {
        assert!(
            ensure_image_bytes_are_valid("file", Path::new("a.png"), &PNG, Some("image/png"))
                .is_ok()
        );
        assert!(
            ensure_image_bytes_are_valid(
                "file",
                Path::new("a.png"),
                b"MZ\x90\x00 not a png",
                Some("image/png")
            )
            .is_err()
        );
        assert!(ensure_image_bytes_are_valid("file", Path::new("a.txt"), &PNG, None).is_err());

        let oversized = png_of_len(MAX_IMAGE_SIZE_BYTES as usize + 1);
        let err =
            ensure_image_bytes_are_valid("file", Path::new("a.png"), &oversized, Some("image/png"))
                .unwrap_err();
        assert!(err.to_string().contains(MAX_IMAGE_SIZE_LABEL));
    }

    #[test]
    fn in_memory_guard_accepts_exactly_the_size_limit() {
        let bytes = png_of_len(MAX_IMAGE_SIZE_BYTES as usize);
        assert!(
            ensure_image_bytes_are_valid("file", Path::new("a.png"), &bytes, Some("image/png"))
                .is_ok()
        );
    }

    #[test]
    fn rejects_declared_name_without_extension() {
        let file = temp_file(&PNG);
        assert!(guard("holiday", &file, None).is_err());
    }

    #[test]
    fn rejects_disallowed_extension() {
        let file = temp_file(b"MZ\x90\x00");
        assert!(guard("a.exe", &file, None).is_err());
    }

    #[test]
    fn rejects_non_image_content_type() {
        let file = temp_file(&PNG);
        let err = guard("a.png", &file, Some("text/html")).unwrap_err();
        assert_eq!(err.status_code(), axum::http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn rejects_renamed_non_image() {
        // Declared a PNG, spooled a PE binary: only the magic-byte check,
        // which reads the temp file, can catch this.
        let file = temp_file(b"MZ\x90\x00 not really a png");
        assert!(guard("a.png", &file, Some("image/png")).is_err());
    }

    #[test]
    fn accepts_real_webp_riff_container() {
        let file = temp_file(&RIFF_WEBP);
        assert!(guard("a.webp", &file, Some("image/webp")).is_ok());
    }

    #[test]
    fn sanitize_strips_directory_components() {
        assert_eq!(sanitize_file_name(Some("../../etc/passwd")), "passwd");
        assert_eq!(sanitize_file_name(Some("C:\\tmp\\evil.png")), "evil.png");
        assert_eq!(sanitize_file_name(Some("plain.png")), "plain.png");
    }

    #[test]
    fn sanitize_falls_back_on_empty_or_dotted_names() {
        assert_eq!(sanitize_file_name(None), "image");
        assert_eq!(sanitize_file_name(Some("   ")), "image");
        assert_eq!(sanitize_file_name(Some("..")), "image");
    }

    #[test]
    fn sanitize_caps_length() {
        let long = format!("{}.png", "a".repeat(400));
        assert_eq!(sanitize_file_name(Some(&long)).len(), MAX_FILE_NAME_LEN);
    }
}
