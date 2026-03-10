// version 4 - what is does?
//Dynamically extracts values from return statements within a single if/else block to replace them with a selection polynomial.

// v2 → hardcoded condition
// v3 → AST extraction of the developer's condition
// v4 → dynamic condition + dynamic branch value extraction
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Stmt, Expr};

#[proc_macro_attribute]
pub fn zk_optimize(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let inputs = &input_fn.sig.inputs;
    let output = &input_fn.sig.output;

    let mut final_code = quote! { #input_fn }; 

    // 1. Find the If Statement
    if let Some(Stmt::Expr(Expr::If(if_statement), _)) = input_fn.block.stmts.first() {
        
        let dynamic_condition = &if_statement.cond;

        // 2. Dig into the True Path to find the return value
        let mut dynamic_true = quote!(0); // Default fallback
        if let Some(Stmt::Expr(Expr::Return(ret), _)) = if_statement.then_branch.stmts.first() {
            if let Some(val) = &ret.expr {
                dynamic_true = quote!(#val); // We ripped out the pure value!
            }
        }

        // 3. Dig into the False Path to find the return value
        let mut dynamic_false = quote!(0); // Default fallback
        if let Some((_, else_expr)) = &if_statement.else_branch {
            if let Expr::Block(else_block) = &**else_expr {
                if let Some(Stmt::Expr(Expr::Return(ret), _)) = else_block.block.stmts.first() {
                    if let Some(val) = &ret.expr {
                        dynamic_false = quote!(#val); // We ripped out the pure value!
                    }
                }
            }
        }

        println!(" EXTRACTION COMPLETE:");
        println!("Condition: {}", quote!(#dynamic_condition));
        println!("True Path: {}", quote!(#dynamic_true));
        println!("False Path: {}", quote!(#dynamic_false));

        // 4. THE DYNAMIC MATH INJECTION
        final_code = quote! {
            fn #fn_name(#inputs) #output {
                let condition_bool = #dynamic_condition;
                let condition_int = condition_bool as u32;
                let inv_condition = 1 - condition_int;
                
                // NO MORE HARDCODED NUMBERS! We inject exactly what's needed.
                return (condition_int * #dynamic_true) + (inv_condition * #dynamic_false);
            }
        };
    }

    TokenStream::from(final_code)
}
