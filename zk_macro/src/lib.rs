// version 5 - what is does?
//Introduces a recursive security scanner that detects and aborts optimization if it finds dangerous side effects like function calls.
//This version prioritized safety; it realized that you can't multiply a "side effect" by zero, so it returns the original code if the logic is too complex for a pure math circuit.

// v2 → hardcoded condition
// v3 → AST extraction of the developer's condition
// v4 → dynamic condition + dynamic branch value extraction
// v5 → safety validation of branch expressions before transformation
    
 extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Stmt, Expr};

//  THE RADAR: Scans the code tree for dangerous side-effects
fn is_safe_for_zk(expr: &Expr) -> bool {
    match expr {
        Expr::Lit(_) => true,  // Safe: Just a number (e.g., 5)
        Expr::Path(_) => true, // Safe: Just a variable (e.g., base_fee)
        Expr::Paren(p) => is_safe_for_zk(&p.expr), // Safe: Math in parentheses
        Expr::Binary(b) => is_safe_for_zk(&b.left) && is_safe_for_zk(&b.right), // Safe: Math (a * b)
        //  ANYTHING ELSE (like Expr::Call) IS DANGEROUS!
        _ => false, 
    }
}

#[proc_macro_attribute]
pub fn zk_optimize(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let inputs = &input_fn.sig.inputs;
    let output = &input_fn.sig.output;

    let mut final_code = quote! { #input_fn }; 

    if let Some(Stmt::Expr(Expr::If(if_statement), _)) = input_fn.block.stmts.first() {
        let dynamic_condition = &if_statement.cond;

        let mut true_expr: Option<&Expr> = None;
        if let Some(Stmt::Expr(Expr::Return(ret), _)) = if_statement.then_branch.stmts.first() {
            true_expr = ret.expr.as_deref();
        }

        let mut false_expr: Option<&Expr> = None;
        if let Some((_, else_expr)) = &if_statement.else_branch {
            if let Expr::Block(else_block) = &**else_expr {
                if let Some(Stmt::Expr(Expr::Return(ret), _)) = else_block.block.stmts.first() {
                    false_expr = ret.expr.as_deref();
                }
            }
        }

        //  THE SECURITY CHECK
        if let (Some(t_expr), Some(f_expr)) = (true_expr, false_expr) {
            
            // Pass both paths through the Radar...
            if is_safe_for_zk(t_expr) && is_safe_for_zk(f_expr) {
                // ALL CLEAR! Inject the ZK Math.
                final_code = quote! {
                    fn #fn_name(#inputs) #output {
                        let condition_bool = #dynamic_condition;
                        let condition_int = condition_bool as u32;
                        let inv_condition = 1 - condition_int;
                        return (condition_int * #t_expr) + (inv_condition * #f_expr);
                    }
                };
            } else {
                //  RADAR TRIGGERED! A function call or side-effect was detected.
                // We abort the injection and return the original, safe `if/else` code.
                println!(" ZK-RADAR ALERT in '{}': Dangerous side-effect detected! Aborting math optimization to protect state.", fn_name);
            }
        }
    }

    TokenStream::from(final_code)
}
