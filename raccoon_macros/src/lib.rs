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

use console::Style;

/// debug message
#[macro_export]
macro_rules! raccoon_debug {
    ($message:expr, $data:expr) => {
        $crate::_debug_print($message, $data)
    };
}

/// error message
#[macro_export]
macro_rules! raccoon_error {
  ($first:tt $(,$rest:tt)*) => {
    $crate::_error_print(&concat!($first$(," ", $rest)*));
  }
}

///informatory message
#[macro_export]
macro_rules! raccoon_info {
  ($first:tt $(,$rest:tt)*) => {
    $crate::_info_print(&concat!($first$(," ", $rest)*));
  }
}

/// debug print in bold blue color
pub fn _debug_print<T: std::fmt::Debug>(message: &str, data: Option<T>) {
    let debug_color = Style::new().blue().bold();
    println!(
        "{} {}\n{:?}",
        debug_color.apply_to("RACOON DEBUG:"),
        message,
        data
    );
}

/// print an error message in bold red color
pub fn _error_print(message: &str) {
    let red_color = Style::new().red().bold();
    println!("{} {}", red_color.apply_to("RACOON ERROR:"), message,);
}

/// print information in bold green text
pub fn _info_print(message: &str) {
    let info_color = Style::new().green().bold();
    println!("{} {}", info_color.apply_to("RACOON INFO:"), message,);
}
