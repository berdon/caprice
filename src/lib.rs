//! Caprice is a [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop) for Rust projects featuring an easy to use, zsh like
//! autocomplete feature.
//!
//!
//! # Example:
//! ```
//! use caprice::Caprice;
//! let mut caprice = Caprice::new(functor);
//!
//! // set some tokens
//! caprice.set_tokens(&vec![
//!    "some_token".to_owned(),
//!    "some_other_token".to_owned(),
//!    "none".to_owned(),
//! ]);
//!
//! // initialise caprice
//! // opting out the alternate terminal
//! caprice.init(false).unwrap();
//!
//! // run the parser in a loop    
//! loop {
//!     // ignoring possible token return
//!     if let Ok(_) = caprice.parse() {}
//!     else {
//!         break
//!     }
//! }
//!
//! // if the users types one of the givens tokens,
//! // the functor will be applied to it
//! fn functor(s: String) -> Result<(), std::io::Error> {
//!    println!("{} activated", s);
//!
//!    Ok(())
//! }  
//! ```

mod caprice_scanner;
pub mod caprice;
pub use caprice::Caprice;
/// Caprice uses the popular Rust type alias for Result<T, E>,
/// where E is std::io::Error
pub type Result<T> = std::result::Result<T, std::io::Error>;
