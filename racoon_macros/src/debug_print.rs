use console::Style;
/// debug print
/// check if the environment is development or production
/// if development, print some stuff for easy development else dont print them

pub fn debug_print<T: std::fmt::Debug>(message: &str, data: T) {
    let cyan = Style::new().cyan();
    println!(" {}:: {message}: {:#?}", cyan.apply_to("DEBUG PRINT"), data);
}
