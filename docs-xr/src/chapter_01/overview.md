# Project Overview
The project is structured using a combination of Node.js/Typescript convections and Cargo workspace organization. 

The project is organized into the following crates:

1. racoon_core (root directory) -  a core library for the project, which contains the core logic for the project
1. racoon_macros - an encapsulation of the `print!()` macro in Rust, implemented with semantic color for error report and debugging in development
3. racoon_mailer - a library for sending emails to users, 