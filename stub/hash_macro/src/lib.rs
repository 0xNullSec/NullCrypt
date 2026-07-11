use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use proc_macro::TokenStream;
//use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, LitStr};


fn djb2_compute(s: &str, mut hash: u64) -> u64 {
    for c in s.bytes() {
        hash = hash.wrapping_shl(5).wrapping_add(hash).wrapping_add(c as u64);
    }
    hash
}

// lib.rs
#[proc_macro]
pub fn calc_hash(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let function_name = input_str.value();

    // Genera la semilla basada en el nanosegundo de compilación
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let hash_result = djb2_compute(&function_name, seed);

    // Devuelve la tupla para ser asignada a una constante
    let expanded = quote! {
        (#hash_result as u64, #seed as u64)
    };

    TokenStream::from(expanded)
}



/*
use proc_macro::TokenStream;
//use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, LitStr};


fn djb2_compute(s: &str) -> u64 {
    let mut hash: u64 = 0x7734773477347734;
    for c in s.bytes() {
        hash = hash.wrapping_shl(5).wrapping_add(hash).wrapping_add(c as u64);
    }
    hash
}

// Macro procedimental: 
// Es una función de Rust que se ejecuta dentro del compilador. 
// Recibe el código fuente como entrada (Tokens), puedes ejecutar cualquier código Rust sobre esa entrada, 
// y devuelves nuevo código
#[proc_macro]
pub fn calc_hash(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let function_name = input_str.value();

    let hash_result = djb2_compute(&function_name);
    let expanded = quote! {
        #hash_result as u64
    };

    TokenStream::from(expanded)
}
*/