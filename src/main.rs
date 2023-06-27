fn main() {
    println!("Hello, world!");
}

/// This function is 120 columns wide and is left alone. This comment is 120 columns wide and the formatter is also fine
fn my_super_cool_function_name(my_very_cool_argument_name: String, my_other_very_cool_argument_name: String) -> String {
    unimplemented!()
}

pub enum Severity {
    /// But here, this comment is 120 columns wide and the formatter wants to split it up onto two separate lines still.
    Error,

    /// This comment is 119 columns wide and works perfectly. Lorem ipsum. lorem ipsum. lorem ipsum. lorem ipsum lorem.
    Warning,
}
