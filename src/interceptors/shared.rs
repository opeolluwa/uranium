use tonic::{metadata::MetadataMap, Status};

pub fn parse_user_id(metadata: &MetadataMap) -> Result<String, Status> {
    // let metadata = request.metadata();

    let Some(user_id) = metadata.get("user_id") else {
        return Err(tonic::Status::unauthenticated(
            "Missing or badly formatted authorization header",
        ));
    };

    let user_id = user_id
        .to_str()
        .map_err(|_| tonic::Status::not_found("Missing or badly formatted authorization header"))?;

    Ok(user_id.to_string())
}
