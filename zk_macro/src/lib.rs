// version 5 - what is does?
// Shifts from simple return values to tracking the state of a single variable (Static Single Assignment) at a fixed position in the code.

// v2 → hardcoded condition
// v3 → AST extraction of the developer's condition
// v4 → dynamic condition + dynamic branch value extraction
// v5 → safety validation of branch expressions before transformation
// v6 → SSA-style state flattening of variable updates  
 
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Stmt, Expr, Pat};

#[proc_macro_attribute]
pub fn zk_optimize(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let inputs = &input_fn.sig.inputs;
    let output = &input_fn.sig.output;

    // 1. Capture the initial variable declaration (e.g., let mut risk_score = 10)
    let mut initial_var_name = quote!{};
    let mut initial_val = quote!{};
    
    if let Some(Stmt::Local(local)) = input_fn.block.stmts.get(0) {
        if let Pat::Ident(ref id) = local.pat {
            let var_ident = &id.ident;
            initial_var_name = quote!(#var_ident);
            if let Some(init) = &local.init {
                let expr = &init.expr;
                initial_val = quote!(#expr);
            }
        }
    }

    // 2. Identify the IF block
    // In syn 2.0, an expression with a semicolon is Stmt::Expr(expr, Some(semi))
    if let Some(Stmt::Expr(Expr::If(if_statement), _)) = input_fn.block.stmts.get(1) {
        let condition = &if_statement.cond;

        // Extract "Path A" logic (The 'Then' branch)
        let mut path_a_logic = quote!{};
        if let Some(Stmt::Expr(Expr::Assign(ass), _)) = if_statement.then_branch.stmts.first() {
            let right = &ass.right;
            path_a_logic = quote!(#right);
        }

        // Extract "Path B" logic (The 'Else' branch)
        let mut path_b_logic = quote!{};
        if let Some((_, else_expr)) = &if_statement.else_branch {
            if let Expr::Block(eb) = &**else_expr {
                if let Some(Stmt::Expr(Expr::Assign(ass), _)) = eb.block.stmts.first() {
                    let right = &ass.right;
                    path_b_logic = quote!(#right);
                }
            }
        }

        // 3. THE MASTER FLATTENING: Generate the SSA Polynomial
        let final_code = quote! {
            fn #fn_name(#inputs) #output {
                let mut #initial_var_name = #initial_val;

                // Evaluate condition as a 1 or 0
                let cond = (#condition) as u32;
                let inv_cond = 1 - cond;

                // Pre-calculate both possible states
                // We use scopes here to avoid variable name collisions
                let val_a = { #path_a_logic };
                let val_b = { #path_b_logic };

                // The Selection Polynomial (Branchless)
                #initial_var_name = (cond * val_a) + (inv_cond * val_b);

                return #initial_var_name;
            }
        };

        println!(" SSA FLATTENING COMPLETE: Function '{}' is now branchless.", fn_name);
        return TokenStream::from(final_code);
    }

    TokenStream::from(quote!{#input_fn})
}
