// version 7  - what is does?
// Uses a "Sniper" function to flawlessly match and flatten multiple parallel state variables across branches, regardless of invisible compiler metadata.

// v2 → hardcoded condition
// v3 → AST extraction of the developer's condition
// v4 → dynamic condition + dynamic branch value extraction
// v5 → safety validation of branch expressions before transformation
// v6 → SSA-style state flattening of variable updates  
// v7 → Multi-Variable State Circuit

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Stmt, Expr};

//  THE SNIPER: Extracts the pure variable name without invisible macro hygiene tags
fn get_pure_name(expr: &Expr) -> Option<String> {
    if let Expr::Path(p) = expr {
        if let Some(seg) = p.path.segments.first() {
            return Some(seg.ident.to_string());
        }
    }
    None
}

#[proc_macro_attribute]
pub fn zk_optimize(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let inputs = &input_fn.sig.inputs;
    let output = &input_fn.sig.output;

    let mut setup_stmts = Vec::new();
    let mut if_stmt_found = None;
    let mut return_expr = quote! { 0 };

    for stmt in &input_fn.block.stmts {
        match stmt {
            Stmt::Local(_) => setup_stmts.push(quote!(#stmt)),
            Stmt::Expr(Expr::If(i), _) => if_stmt_found = Some(i),
            Stmt::Expr(Expr::Return(r), _) => {
                if let Some(e) = &r.expr { return_expr = quote!(#e); }
            },
            Stmt::Expr(e, None) => return_expr = quote!(#e),
            _ => {}
        }
    }

    if let Some(if_statement) = if_stmt_found {
        let condition = &if_statement.cond;
        let mut updates = Vec::new();

        println!("--------------------------------------------------");
        
        for stmt in &if_statement.then_branch.stmts {
            let inner_expr = match stmt {
                Stmt::Expr(e, _) => Some(e),
                _ => None,
            };

            if let Some(Expr::Assign(ass)) = inner_expr {
                let var_name = &ass.left;
                let path_a_val = &ass.right;
                
                // Use the Sniper to get the pure name
                let pure_var_name = get_pure_name(var_name).unwrap_or_default();
                
                let mut path_b_val = quote!(#var_name); // Default fallback

                if let Some((_, else_expr)) = &if_statement.else_branch {
                    if let Expr::Block(eb) = &**else_expr {
                        for e_stmt in &eb.block.stmts {
                            let e_inner = match e_stmt {
                                Stmt::Expr(e, _) => Some(e),
                                _ => None,
                            };
                            if let Some(Expr::Assign(e_ass)) = e_inner {
                                // Use the Sniper again for the Else branch
                                let pure_else_name = get_pure_name(&e_ass.left).unwrap_or_default();
                                
                                // Flawless equality check!
                                if pure_var_name == pure_else_name && !pure_var_name.is_empty() {
                                    let right_side = &e_ass.right;
                                    path_b_val = quote!(#right_side);
                                    println!(" MATCH FOUND FOR: {}", pure_var_name);
                                }
                            }
                        }
                    }
                }

                updates.push(quote! {
                    #var_name = (cond * (#path_a_val)) + (inv_cond * (#path_b_val));
                });
            }
        }

        let final_code = quote! {
            fn #fn_name(#inputs) #output {
                #(#setup_stmts)*
                let cond = (#condition) as u32;
                let inv_cond = 1 - cond;
                #(#updates)*
                return #return_expr;
            }
        };

        println!(" INJECTED ZK CIRCUIT:\n{}", final_code.to_string());
        println!("--------------------------------------------------");
        return TokenStream::from(final_code);
    }

    TokenStream::from(quote!{#input_fn})
}
