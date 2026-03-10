// version 3 - what is does?
// Intercept the function at compile time, pattern-match the AST to detect `if` statements,
// extract the developer’s condition dynamically, and inject it into a branchless polynomial template for ZK-style execution.

// v2 → hardcoded condition
// v3 → AST extraction of the developer's condition

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Stmt, Expr};

#[proc_macro_attribute]
pub fn zk_optimize(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 1. Read the incoming function
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let inputs = &input_fn.sig.inputs;
    let output = &input_fn.sig.output;

    // We set a fallback: If we don't find an `if` statement, just return the normal code.
    let mut final_code = quote! { #input_fn }; 

    // 2. THE PATTERN MATCH (Opening the Russian Doll)
    // We check the first line of code inside the function block. 
    // If it perfectly matches the shape of an `If` statement, we unlock it!
    if let Some(Stmt::Expr(Expr::If(if_statement), _)) = input_fn.block.stmts.first() {
        
        //  WE CAUGHT IT! Extract the exact condition the developer wrote.
        let dynamic_condition = &if_statement.cond;

        println!(" Unlocked the AST! Found condition: {}", quote!(#dynamic_condition).to_string());

        // 3. THE DYNAMIC INJECTION
        // We build the new function, injecting the developer's exact logic
        // into our ZK math template using the `#` hashtag syntax!
        final_code = quote! {
            fn #fn_name(#inputs) #output {
                // Look closely: We are pasting their dynamic condition right here!
                let condition_bool = #dynamic_condition;
                let condition_int = condition_bool as u32;
                let inv_condition = 1 - condition_int;
                
                // (For this step, we leave 5 and 9 hardcoded to prove the condition extraction works)
                return (condition_int * 5) + (inv_condition * 9);
            }
        };
    }

    println!(" INJECTED THIS CODE INTO THE COMPILER:");
// // //     println!("{}", final_code.to_string());
// // //     println!("--------------------------------------------------");

// // //     TokenStream::from(final_code)
// // // }
