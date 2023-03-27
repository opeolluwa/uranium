// Copyright 2022 The raccoon Authors. All Rights Reserved.
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
use serde::{Deserialize, Serialize};
use std::fmt;

///email payload
/// contains raw HTML to be injected into HTML template
/// the email subject,
/// the email recipient name
/// the email recipient address
#[derive(Debug, Serialize, Deserialize)]
pub struct EmailPayload<T> {
    pub recipient_name: String,
    pub recipient_address: String,
    pub data: T,
    pub email_subject: String,
}
impl<T: fmt::Display> fmt::Display for EmailPayload<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(recipient_name: {}\nrecipient_address: {}\ndata: {}\nemail_subject: {})",
            self.recipient_name, self.recipient_address, self.data, self.email_subject
        )
    }
}
