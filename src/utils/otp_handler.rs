// Copyright 2022 The Racoon Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use once_cell::sync::Lazy;
use otp_rs::TOTP;
use racoon_macros::racoon_error;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use validator::Validate;

use crate::models::users::{AccountStatus, UserModel};

// TOTP encryption key
pub static OTP: Lazy<TOTP> = Lazy::new(|| -> TOTP {
    TOTP::new(&env::var("TOTP_ENCRYPTION_KEY").expect("TOTP secret missing"))
});

/// OTP VALIDITY PERIOD SET TO 5 MINUTES
pub const OTP_VALIDITY: u64 = 60 * 5;

//get the current time stamp
static CURRENT_TIMESTAMP: Lazy<u64> = Lazy::new(|| {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
});

///one time password
#[derive(Debug, Serialize, Deserialize, Validate, sqlx::FromRow, Default)]
pub struct Otp {
    pub id: Uuid,
    pub token: String,
    pub is_expired: bool,
}

/// ipl display for Otp
impl std::fmt::Display for Otp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}\ntoken{}is_expired\n{}",
            self.id, self.token, self.is_expired
        )
    }
}
/// helper function for OTP
impl Otp {
    /// create new otp;
    pub fn new() -> Self {
        let id = Uuid::new_v4();
        let token = OTP.generate(OTP_VALIDITY, *CURRENT_TIMESTAMP).unwrap();
        Self {
            id,
            token: token.to_string(),
            ..Default::default()
        }
    }

    /// save a newly created OTP to the database
    pub async fn save(&self, db_connection: &Pool<Postgres>) -> Self {
        let otp = sqlx::query_as::<_, Otp>(
            "INSERT INTO one_time_passwords (id, token)
       VALUES ($1, $2) RETURNING *",
        )
        .bind(&self.id)
        .bind(&self.token)
        .fetch_one(db_connection)
        .await;

        if otp.is_err() {
            racoon_error!("An exception  was encountered while inserting OTP into the database");
            println!("{:?}\n", otp);
        }
        Self { ..otp.unwrap() }
    }

    /// link a newly created otp to a user using the user Id
    pub async fn link_to_user(&self, user_id: Uuid, db_connection: &Pool<Postgres>) -> UserModel {
        let linked_user = sqlx::query_as::<_, UserModel>(
            "UPDATE user_information SET otp_id = $1 WHERE id = $2 RETURNING *",
        )
        .bind(Uuid::from(self.id))
        .bind(Uuid::from(user_id))
        .fetch_one(db_connection)
        .await;
        if linked_user.is_err() {
            racoon_error!("An exception  was encountered while linking user Id to OTP");
            println!("{:?}\n", linked_user);
        }
        linked_user.ok().unwrap()
    }

    // fetch and verify otp
    pub async fn validate_otp(otp_id: Uuid, token: &str, db_connection: &Pool<Postgres>) -> bool {
        let verifiable_otp =
            sqlx::query_as::<_, Otp>("SELECT * FROM one_time_passwords WHERE id = $1")
                .bind(Uuid::from(otp_id))
                // .bind(token.trim())
                .fetch_one(db_connection)
                .await;

        if verifiable_otp.is_err() {
            return false;
        }

        println!("{:?}", verifiable_otp);
        true
    }
    /// unlink otp from user
    pub async fn _unlink_from_user(
        &self,
        user_id: Uuid,
        db_connection: &Pool<Postgres>,
    ) -> UserModel {
        let linked_user = sqlx::query_as::<_, UserModel>(
            "UPDATE user_information SET otp_id = $1, account_status = $2 WHERE id = $3 RETURNING *",
        )
        .bind(String::from("null"))
        .bind(AccountStatus::Active)
        .bind(Uuid::from(user_id))
        .fetch_one(db_connection)
        .await;
        if linked_user.is_err() {
            racoon_error!("An exception  was encountered while unlinking user Id from OTP");
            println!("{:?}\n", linked_user);
        }
        linked_user.ok().unwrap()
    }
    /*  /// validate otp
    /// accept the otp that was generated  as function params,
    /// verify the
    pub fn validate_otp(otp: &str) -> bool {
        let otp = otp.trim().parse::<u32>().unwrap();
        OTP.verify(otp, OTP_VALIDITY, *CURRENT_TIMESTAMP)
    } */
}

/// tests
#[cfg(test)]
mod test {
    use super::Otp;
    #[test]
    fn otp_is_generated() {
        let otp = Otp::new();
        let otp: u32 = otp.token.parse().unwrap();
        assert!(otp > 0)
    }
}
