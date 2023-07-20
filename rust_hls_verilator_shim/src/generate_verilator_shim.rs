mod shim_generating_visitor;

use std::io;

use extract_rust_hdl_interface::Direction;
use extract_rust_hdl_interface::{RustHdlModule, SignalType};
use std::panic::catch_unwind;
use syn::{visit, ForeignItem, ImplItem, ItemMod};
use thiserror::Error;

use proc_macro2::{Ident, Span, TokenStream};
use quote::format_ident;
use quote::quote;
use rust_hls_macro_lib::synthesized_struct_name;

#[derive(Error, Debug)]
pub enum GenerateVerilatorShimError {
    #[error("Failed to generate verilator shim")]
    PanickedWhileGeneratingVerilatorShim(),
    #[error("Generator did not create exactly one file. Should not happen.")]
    NotExactlyOneResult(),
    #[error(transparent)]
    FailedWhileGeneratingVerilatorShims(#[from] io::Error),
    #[error(transparent)]
    FailedToParseGeneratedError(#[from] syn::Error),
}

use syn::visit_mut::VisitMut;

use self::shim_generating_visitor::ShimGeneratingVisitor;

struct TraceFunctionRemover {}

impl TraceFunctionRemover {
    fn new() -> Self {
        Self {}
    }
}

impl VisitMut for TraceFunctionRemover {
    fn visit_item_impl_mut(&mut self, item_impl: &mut syn::ItemImpl) {
        let is_trace_function = |item: &ImplItem| match item {
            syn::ImplItem::Fn(impl_fn) => {
                if impl_fn.sig.ident.to_string() == "open_trace" {
                    true
                } else if impl_fn.sig.ident.to_string() == "trace_at" {
                    true
                } else {
                    false
                }
            }
            _ => false,
        };

        let items = &mut item_impl.items;

        let mut i = 0;
        while i < items.len() {
            if is_trace_function(&items[i]) {
                items.remove(i);
            } else {
                i += 1;
            }
        }

        syn::visit_mut::visit_item_impl_mut(self, item_impl);
    }
    fn visit_item_foreign_mod_mut(&mut self, item_impl: &mut syn::ItemForeignMod) {
        let is_trace_function = |item: &ForeignItem| match item {
            ForeignItem::Fn(impl_fn) => {
                if impl_fn.sig.ident.to_string().ends_with("trace") {
                    true
                } else {
                    false
                }
            }
            _ => false,
        };

        let items = &mut item_impl.items;

        let mut i = 0;
        while i < items.len() {
            if is_trace_function(&items[i]) {
                items.remove(i);
            } else {
                i += 1;
            }
        }
        syn::visit_mut::visit_item_foreign_mod_mut(self, item_impl);
    }
    fn visit_path_mut(&mut self, item_impl: &mut syn::Path) {
        let stringified_path = quote! {#item_impl}.to_string();
        let reference_path = quote!(::verilated::vcd::Vcd).to_string();
        if stringified_path == reference_path {
            *item_impl = syn::parse2(quote!(u32)).unwrap();
        } else {
            syn::visit_mut::visit_path_mut(self, item_impl);
        }
    }
}

/// The generated Rust shim has a dependency on verilated for tracing.
/// This function removes the tracing functions from the generated shim.
fn remove_tracing_functions(verilated_module: &mut ItemMod) -> () {
    let mut visitor = TraceFunctionRemover::new();
    syn::visit_mut::visit_item_mod_mut(&mut visitor, verilated_module);
}

pub struct GeneratedVerilatorShim {
    pub cpp: String,
    pub function_name: String,
    pub module_name: Ident,
    pub struct_name: Ident,
    pub rust_module: ItemMod,
}

pub fn generate_verilator_shim_from_rusthdl_module(
    rust_hdl_module: &RustHdlModule,
    // function name
    function_name: &str,
) -> Result<GeneratedVerilatorShim, GenerateVerilatorShimError> {
    let rust_module_name = format!("{}_verilated", function_name);
    let rust_struct_name = synthesized_struct_name(&rust_module_name);

    let fields: Vec<TokenStream> = rust_hdl_module
        .signals
        .iter()
        .map(|signal| {
            let name = format_ident!("{}", &signal.internal_name);
            let ty = match signal.signal_type {
                SignalType::Bit => quote! { bool },
                SignalType::Bits(x) => {
                    let x = x as usize;
                    quote! { [bool; #x] }
                }
                SignalType::Clock => quote! { bool },
            };
            let attribute = match signal.direction {
                Direction::In => quote! { #[port(input)] },
                Direction::Out => quote! { #[port(output)] },
                Direction::InOut => quote! { #[port(inout)] },
            };
            return quote!( #attribute pub #name: #ty, );
        })
        .collect();
    let function_name_ident = format_ident!("{}", function_name);
    let struct_name_ident = format_ident!("{}", rust_struct_name);

    let rust_module = quote! (
        use verilated_module::module;

        #[module(#function_name_ident)]
        pub struct #struct_name_ident {
            #(#fields)*
        }
    );

    let file = syn::parse2::<syn::File>(rust_module)?;

    call_verilated_rs_generator(&file, function_name, &rust_struct_name, &rust_module_name)
}

/// Runs code from verilated-rs to generate a shim.
///
/// `dummy_module` is a Rust module that contains a module macro for verilated-rs
pub fn call_verilated_rs_generator(
    dummy_module: &syn::File,
    // function name / verilog module name
    function_name: &str,
    // ident of the generated struct
    rust_struct_name: &str,
    // ident of the generated module containing the struct
    rust_module_name: &str,
) -> Result<GeneratedVerilatorShim, GenerateVerilatorShimError> {
    let generated_files = catch_unwind(|| {
        let mut visitor = ShimGeneratingVisitor::default();
        visit::visit_file(&mut visitor, dummy_module);

        visitor.finalize()
    })
    .or_else(|_| Err(GenerateVerilatorShimError::PanickedWhileGeneratingVerilatorShim()))?;

    let generated_file = generated_files
        .into_iter()
        .next()
        .ok_or(GenerateVerilatorShimError::NotExactlyOneResult {})?;

    let rust_file: syn::File = syn::parse_str(&generated_file.rust)?;
    let mut rust_module = syn::ItemMod {
        attrs: rust_file.attrs,
        vis: syn::Visibility::Inherited,
        mod_token: syn::Token![mod](Span::call_site()),
        ident: format_ident!("{}", rust_module_name),
        content: Some((syn::token::Brace::default(), rust_file.items)),
        semi: None,
        unsafety: None,
    };

    remove_tracing_functions(&mut rust_module);

    return Ok(GeneratedVerilatorShim {
        cpp: generated_file.cpp,
        function_name: function_name.into(),
        module_name: format_ident!("{}", rust_module_name),
        rust_module: rust_module,
        struct_name: format_ident!("{}", rust_struct_name),
    });
}

#[cfg(test)]
mod tests {

    use extract_rust_hdl_interface::extract_rust_hdl_interface;
    use quote::ToTokens;

    use super::*;

    #[test]
    fn generating_verilator_shims_does_not_fail() {
        let input_file: syn::File = syn::parse2(quote!(
            #[module(my_counter)]
            pub struct MyCounterVerilated {
                #[port(input)]
                pub clock: bool,
                #[port(output)]
                pub led: [bool; 7],
            }
        ))
        .unwrap();

        call_verilated_rs_generator(
            &input_file,
            "my_counter",
            "MyCounterVerilated",
            "my_counter_verilated",
        )
        .unwrap();
    }

    #[test]
    fn generating_verilator_shims_returns_something_that_looks_correct() {
        let input_file: syn::File = syn::parse2(quote!(
            #[module(my_counter)]
            pub struct MyCounterVerilated {
                #[port(input)]
                pub clock: bool,
                #[port(output)]
                pub led: [bool; 7],
            }
        ))
        .unwrap();

        let result = call_verilated_rs_generator(
            &input_file,
            "my_counter",
            "MyCounterVerilated",
            "my_counter_verilated",
        )
        .unwrap();

        assert!(result
            .rust_module
            .to_token_stream()
            .to_string()
            .contains("impl MyCounterVerilated {"));
        assert!(result.cpp.contains("#include <Vmy_counter.h>"));
    }

    #[test]
    fn generating_verilator_shims_from_rusthdl_returns_something_that_looks_correct() {
        let rust_hdl_module = extract_rust_hdl_interface(
            r#"
        module my_counter (clock, led);
        input clock;
        output [6:0] led;
        endmodule"#,
            "my_counter",
        )
        .unwrap();

        let result =
            generate_verilator_shim_from_rusthdl_module(&rust_hdl_module, "my_counter").unwrap();

        assert!(result
            .rust_module
            .to_token_stream()
            .to_string()
            .contains("impl MyCounterVerilated {"));
        assert!(result.cpp.contains("#include <Vmy_counter.h>"));
    }

    #[test]
    fn generated_output_does_not_contain_tracing_functions() {
        let rust_hdl_module = extract_rust_hdl_interface(
            r#"
        module my_counter (clock, led);
        input clock;
        output [6:0] led;
        endmodule"#,
            "my_counter",
        )
        .unwrap();

        let result =
            generate_verilator_shim_from_rusthdl_module(&rust_hdl_module, "my_counter").unwrap();

        let module_string = result.rust_module.into_token_stream().to_string();

        assert!(!module_string.contains("trace"));
    }

    #[test]
    fn generated_output_removes_verilated_vcd() {
        let rust_hdl_module = extract_rust_hdl_interface(
            r#"
        module my_counter (clock, led);
        input clock;
        output [6:0] led;
        endmodule"#,
            "my_counter",
        )
        .unwrap();

        let result =
            generate_verilator_shim_from_rusthdl_module(&rust_hdl_module, "my_counter").unwrap();

        let module_string = result.rust_module.into_token_stream().to_string();

        assert!(!module_string.contains("Vcd"));
    }
}
