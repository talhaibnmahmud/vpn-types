use std::io;
use vpn_types::error::{BoxedError, ErrorExtension};

fn main() {
    // Create a sample error with a source
    let source_error = io::Error::new(io::ErrorKind::NotFound, "Underlying cause");
    let main_error = io::Error::new(io::ErrorKind::Other, source_error);

    // Use display_chain
    println!("{}", main_error.display_chain());

    // Use display_chain_with_message
    println!(
        "{}",
        main_error.display_chain_with_message("An error occurred while processing the request")
    );

    let err: BoxedError = io::Error::new(io::ErrorKind::Other, "Some error").into();
    println!("{}", err.display_chain());
}
