// check password for correctness
            let verify_password = verify(password, &hashed_password);
            let pass = match verify_password {
                Ok(is_correct_password) => {
                    if !is_correct_password {
                        ApiErrorResponse::BadRequest { error: vec![] }
                    }
                }
                Err(err) => Err(ApiErrorResponse::ServerError { error: vec![] }),
            };

            //:encrypt the user data
            let jwt_payload = JwtClaims {
                id: id.to_string(),
                email,
                fullname,
                exp: 2000000000, //may 2023
            };
            let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
                String::from("Ux6qlTEMdT0gSLq9GHp812R9XP3KSGSWcyrPpAypsTpRHxvLqYkeYNYfRZjL9")
            });
            //use a custom header
            let jwt_header = Header {
                alg: Algorithm::HS512,
                ..Default::default()
            };
            //build u the jwt token
            let token = encode(
                &jwt_header,
                &jwt_payload,
                &EncodingKey::from_secret(jwt_secret.as_bytes()),
            )
            .unwrap();

            //build up response
            let response: ApiResponse<JwtPayload, _> = ApiResponse::<JwtPayload, _> {
                success: true,
                message: String::from("user successfully retrieved"),
                data: Some(JwtPayload {
                    token,
                    token_type: String::from("Bearer"),
                }),
                error: None::<String>,
            };

            //build the response
            let response: ApiSuccessResponse<UserInformation> =
                ApiSuccessResponse::<UserInformation> {
                    success: true,
                    message: String::from("user successfully logged in"),
                    data: Some(result),
                };
            Ok((StatusCode::Ok, Json(response)))