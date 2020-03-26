// Use Cargo features to include multiple modules
// depending on what features we pass to Cargo.

// If this feature is specifed, use the function 'calc_money_owed' from the second
// lib instead of the first one. Needs to be the same as what's in the Cargo.toml

#[cfg(feature = "calc_interest")]
use applyinterest as ext;

#[cfg(feature = "neg_interest")]
use neginterest as ext;

// We need a 'default' case otherwise Rust won't find ext crate
#[cfg(not(any(feature = "neg_interest", feature = "calc_interest")))]
use nointerest as ext;


fn main(){
    // Use this as a guideline for the amount of interest that we owe
    let base: f32 = 2.22;
    println!("Getting total interest owed for €{} ", base);
    let result = ext::calc_money_owed(base);
    println!("You owe: €{}", result);

    // Now call a function that exists in only one of the crates, causing a compile time error 
    // unless that crate's Cargo feature is specified
    let balance = ext::get_balance();
    println!("Bal = {}", balance);
}