//! Module generator copied from https://github.com/djg/verilated-rs
//!
//! Removed support for loading verilog from files
//!
//! Adjusted for syn 2

use fnv::FnvHashSet;

use std::io::prelude::*;

use syn::visit::Visit;
use syn::{self, AttrStyle, Attribute, Fields, Generics, ItemStruct, Meta, Visibility};

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => panic!("{} failed with {}", stringify!($e), e),
        }
    };
}

fn check_name(attr: &Attribute, name: &str) -> bool {
    if let Some(ident) = attr.meta.path().get_ident() {
        ident.to_string() == name
    } else {
        false
    }
}

fn is_public(vis: &Visibility) -> bool {
    match vis {
        Visibility::Public(..) => true,
        _ => false,
    }
}

fn assert_no_generics(generics: &Generics) {
    assert!(generics.params.is_empty());
    assert!(generics.where_clause.is_none());
}

struct StructFinder {
    structs: FnvHashSet<String>,
}

impl<'ast> Visit<'ast> for StructFinder {
    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        let any_module = i
            .attrs
            .iter()
            .any(|attr| attr.style == AttrStyle::Outer && check_name(attr, "module"));
        if any_module {
            self.structs.insert(i.ident.to_string());
        }
    }
}

struct Port {
    name: String,
    ty: String,
}

struct Ports {
    clock: Option<Port>,
    reset: Option<Port>,
    inputs: Vec<Port>,
    outputs: Vec<Port>,
    inouts: Vec<Port>,
}

pub struct GenModuleResult {
    pub cpp: String,
    pub rust: String,
}

fn generate_shims(
    rust_name: &str,
    verilog_name: &str,
    item_struct: &ItemStruct,
) -> GenModuleResult {
    let mut rs_out = Vec::new();
    let mut cpp_out = Vec::new();

    let ports = extract_ports(&item_struct.fields);

    t!(writeln!(
        rs_out,
        r#"use std::path::Path;

mod ffi {{
#[allow(non_camel_case_types)]
pub enum {c_ty} {{}}

extern {{
    pub fn {c_ty}_new() -> *mut {c_ty};
    pub fn {c_ty}_delete({c_ty}: *mut {c_ty});
    pub fn {c_ty}_eval({c_ty}: *mut {c_ty});
    pub fn {c_ty}_trace({c_ty}: *mut {c_ty}, vcd: *mut ::verilated::vcd::VcdC, levels: ::std::os::raw::c_int);
    pub fn {c_ty}_final({c_ty}: *mut {c_ty});"#,
        c_ty = verilog_name
    ));

    t!(writeln!(
        cpp_out,
        r#"#include <V{c_ty}.h>

extern "C" {{
// CONSTRUCTORS
V{c_ty}*
{c_ty}_new() {{
V{c_ty}*ptr = new V{c_ty}();"#,
        c_ty = verilog_name
    ));

    if let Some(clock) = &ports.clock {
        t!(writeln!(cpp_out, "    ptr->{clk} = 0;", clk = clock.name));
    }

    if let Some(reset) = &ports.reset {
        t!(writeln!(cpp_out, "    ptr->{rst} = 0;", rst = reset.name));
    }

    t!(writeln!(
        cpp_out,
        r#"    return ptr;
}}

void
{c_ty}_delete(V{c_ty}* __ptr) {{
delete __ptr;
}}

// API METHODS
void
{c_ty}_eval(V{c_ty}* __ptr) {{
__ptr->eval();
}}

void
{c_ty}_trace(V{c_ty}* __ptr, VerilatedVcdC* __tfp, int __levels) {{
__ptr->trace(__tfp, __levels);
}}

void
{c_ty}_final(V{c_ty}* __ptr) {{
__ptr->final();
}}
"#,
        c_ty = verilog_name
    ));

    write_clock(&mut rs_out, &mut cpp_out, verilog_name, &ports.clock);
    write_reset(&mut rs_out, &mut cpp_out, verilog_name, &ports.reset);

    t!(writeln!(cpp_out, "  // PORTS"));
    write_inputs(&mut rs_out, &mut cpp_out, verilog_name, &ports.inputs);
    write_outputs(&mut rs_out, &mut cpp_out, verilog_name, &ports.outputs);
    write_inouts(&mut rs_out, &mut cpp_out, verilog_name, &ports.inouts);

    t!(writeln!(
        rs_out,
        r#"    }}
}}
"#
    ));

    t!(writeln!(cpp_out, r#"}}"#));

    t!(writeln!(
        rs_out,
        r#"pub struct {rs_ty}(*mut ffi::{c_ty}, Option<::verilated::vcd::Vcd>);

impl Default for {rs_ty} {{
fn default() -> Self {{
    let ptr = unsafe {{ ffi::{c_ty}_new() }};
    assert!(!ptr.is_null());
    {rs_ty}(ptr, None)
}}
}}

impl Drop for {rs_ty} {{
fn drop(&mut self) {{
    unsafe {{
        ffi::{c_ty}_delete(self.0);
    }}
}}
}}

#[allow(dead_code, non_snake_case)]
impl {rs_ty} {{"#,
        c_ty = verilog_name,
        rs_ty = rust_name
    ));

    for input in &ports.inputs {
        t!(writeln!(
            rs_out,
            r#"    pub fn set_{input}(&mut self, v: {ty}) {{
    unsafe {{ ffi::{c_ty}_set_{input}(self.0, v); }}
}}
"#,
            c_ty = verilog_name,
            input = input.name,
            ty = &input.ty
        ));
    }

    for output in &ports.outputs {
        t!(writeln!(
            rs_out,
            r#"    pub fn {output}(&self) -> {ty} {{
    unsafe {{ ffi::{c_ty}_get_{output}(self.0) }}
}}
"#,
            c_ty = verilog_name,
            output = output.name,
            ty = &output.ty
        ));
    }

    for inout in &ports.inouts {
        t!(writeln!(
            rs_out,
            r#"    pub fn set_{inout}(&mut self, v: {ty}) {{
    unsafe {{ ffi::{c_ty}_set_{inout}(self.0, v); }}
}}

pub fn {inout}(&self) -> {ty} {{
    unsafe {{ ffi::{c_ty}_get_{inout}(self.0) }}
}}
"#,
            c_ty = verilog_name,
            inout = inout.name,
            ty = &inout.ty
        ));
    }

    t!(writeln!(
        rs_out,
        r#"
pub fn eval(&mut self) {{
    unsafe {{
        ffi::{c_ty}_eval(self.0);
    }}
}}

pub fn finish(&mut self) {{
    unsafe {{
        ffi::{c_ty}_final(self.0);
    }}
}}
"#,
        c_ty = verilog_name,
    ));

    // Tracing API
    t!(writeln!(
        rs_out,
        r#"    pub fn open_trace<P: AsRef<Path>>(&mut self, path: P, levels: i32) -> std::io::Result<()> {{
    ::verilated::trace_ever_on(true);
    let mut vcd = ::verilated::vcd::Vcd::default();
    unsafe {{
        ffi::{c_ty}_trace(self.0, vcd.0, levels);
    }}
    vcd.open(path)?;
    self.1 = Some(vcd);
    Ok(())
}}

pub fn trace_at(&mut self, nanos: ::std::time::Duration) {{
    if let Some(ref mut vcd) = self.1 {{
        let timeui = nanos.as_secs() * 1_000_000_000 + u64::from(nanos.subsec_nanos());
        vcd.dump(timeui);
    }}
}}
"#,
        c_ty = verilog_name
    ));

    if let Some(clock) = ports.clock {
        t!(writeln!(
            rs_out,
            r#"    pub fn clock_toggle(&mut self) {{
    unsafe {{
        ffi::{c_ty}_{clk}_toggle(self.0);
    }}
}}
"#,
            c_ty = verilog_name,
            clk = clock.name
        ));
    } else {
        t!(writeln!(
            rs_out,
            r#"    pub fn clock_toggle(&mut self) {{
unimplemented!();
}}
"#
        ));
    }

    if let Some(reset) = ports.reset {
        t!(writeln!(
            rs_out,
            r#"    pub fn reset_toggle(&mut self) {{
    unsafe {{
        ffi::{c_ty}_{rst}_toggle(self.0);
    }}
}}
"#,
            c_ty = verilog_name,
            rst = reset.name
        ));
    } else {
        t!(writeln!(
            rs_out,
            r#"    fn reset_up(&mut self) {{
unimplemented!();
}}

fn reset_down(&mut self) {{
unimplemented!();
}}
"#
        ));
    }

    t!(writeln!(rs_out, r#"}}"#));

    GenModuleResult {
        cpp: String::from_utf8(cpp_out).expect("Failed to parse generated cpp code as utf8 string"),
        rust: String::from_utf8(rs_out)
            .expect("Failed to parse generated rust code as utf8 string"),
    }
}

#[derive(Default)]
pub struct ShimGeneratingVisitor {
    generated_modules: Vec<GenModuleResult>,
}

impl ShimGeneratingVisitor {
    pub fn finalize(self) -> Vec<GenModuleResult> {
        self.generated_modules
    }
}

impl<'ast> Visit<'ast> for ShimGeneratingVisitor {
    fn visit_item_struct(&mut self, item_struct: &'ast ItemStruct) {
        if !is_public(&item_struct.vis) {
            return;
        }
        assert_no_generics(&item_struct.generics);
        for attr in &item_struct.attrs {
            let acc = find_module_attrs(attr);
            if !acc.is_empty() {
                let rust_name = item_struct.ident.to_string();
                let verilog_name = &acc[0];
                let result = generate_shims(&rust_name, &verilog_name, item_struct);
                self.generated_modules.push(result);
            }
        }
    }
}

fn find_module_attrs(attr: &Attribute) -> Vec<String> {
    let mut acc = Vec::new();
    match attr.meta {
        Meta::List(ref list) if attr.meta.path().get_ident().unwrap().to_string() == "module" => {
            let ident: syn::Ident =
                syn::parse2(list.tokens.clone()).expect("Expected exactly one argument, an ident.");

            acc.push(ident.to_string());
        }
        _ => {}
    }
    acc
}

enum PortAttr {
    None,
    Clock,
    Reset,
    Input,
    Output,
    InOut,
}

fn find_port_attr(attrs: &[Attribute]) -> PortAttr {
    attrs.iter().fold(PortAttr::None, |pa, attr| {
        let meta = attr.meta.clone();
        match meta {
            syn::Meta::List(ref items) => {
                if items
                    .path
                    .get_ident()
                    .map(|i| i.to_string())
                    .unwrap_or("notport".into())
                    != "port"
                {
                    return pa;
                }

                let ident: syn::Ident = syn::parse2(items.tokens.clone())
                    .expect("Expected exactly one argument, an ident.");

                return match ident.to_string().as_str() {
                    "clock" => PortAttr::Clock,
                    "reset" => PortAttr::Reset,
                    "input" => PortAttr::Input,
                    "output" => PortAttr::Output,
                    "inout" => PortAttr::InOut,
                    _ => panic!("invalid argument"),
                };
            }
            syn::Meta::Path(_) => {
                panic!("expected one argument");
            }
            _ => pa,
        }
    })
}

fn expr2width(e: &syn::Expr) -> usize {
    match e {
        syn::Expr::Lit(ref l) => match l.lit {
            syn::Lit::Int(ref a) => a.base10_parse().unwrap(),
            _ => panic!("unknown literal: {:?}", l),
        },
        _ => panic!("unknown expr: {:?}", e),
    }
}

fn ty2name(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(syn::TypePath { ref path, .. }) => {
            let last = path.segments.last().unwrap();
            if last.ident.to_string() == "bool" {
                "u8".to_string()
            } else {
                panic!("only support bool");
            }
        }
        syn::Type::Array(arr) => {
            assert!(ty2name(&*arr.elem) == "u8");
            match expr2width(&arr.len) {
                0..=8 => "u8".to_string(),
                9..=16 => "u16".to_string(),
                17..=32 => "u32".to_string(),
                33..=64 => "u64".to_string(),
                _ => unreachable!(),
            }
        }
        _ => panic!("unknown ty {:?}", ty),
    }
}

fn rust2ffi(ty: &str) -> String {
    match ty {
        "u8" => "::std::os::raw::c_uchar".to_string(),
        "u16" => "::std::os::raw::c_ushort".to_string(),
        "u32" => "::std::os::raw::c_uint".to_string(),
        "u64" => "::std::os::raw::c_ulong".to_string(),
        s => s.to_string(),
    }
}

fn rust2ver(ty: &str) -> String {
    match ty {
        "u8" => "vluint8_t".to_string(),
        "u16" => "vluint16_t".to_string(),
        "u32" => "vluint32_t".to_string(),
        "u64" => "vluint64_t".to_string(),
        s => s.to_string(),
    }
}

fn extract_ports(fields: &Fields) -> Ports {
    let ports = Ports {
        clock: None,
        reset: None,
        inputs: Vec::new(),
        outputs: Vec::new(),
        inouts: Vec::new(),
    };

    match fields {
        Fields::Named(ref fields) => fields.named.iter().fold(ports, |mut ports, field| {
            let port = match field.ident.clone() {
                Some(name) => {
                    let name = name.to_string();
                    let ty = ty2name(&field.ty);
                    Port { name, ty }
                }
                None => panic!("no tuple structs in FFI"),
            };

            if is_public(&field.vis) {
                match find_port_attr(&field.attrs) {
                    PortAttr::Clock => {
                        if ports.clock.is_some() {
                            panic!("only one clock allowed in FFI");
                        }
                        ports.clock = Some(port);
                    }
                    PortAttr::Reset => {
                        if ports.reset.is_some() {
                            panic!("only one reset allowed in FFI");
                        }
                        ports.reset = Some(port);
                    }
                    PortAttr::Input => {
                        ports.inputs.push(port);
                    }
                    PortAttr::Output => {
                        ports.outputs.push(port);
                    }
                    PortAttr::InOut => {
                        ports.inouts.push(port);
                    }
                    _ => {}
                }
            }
            ports
        }),
        _ => panic!("no tuple structs in FFI"),
    }
}

fn write_clock<W>(rs_out: &mut W, cpp_out: &mut W, c_ty: &str, clock: &Option<Port>)
where
    W: Write,
{
    if let Some(ref clock) = clock {
        t!(writeln!(
            rs_out,
            r#"        pub fn {c_ty}_{clk}_toggle({c_ty}: *mut {c_ty});"#,
            c_ty = c_ty,
            clk = clock.name
        ));

        t!(writeln!(
            cpp_out,
            r#"  void
  {c_ty}_{clk}_toggle(V{c_ty}* __ptr) {{
    __ptr->{clk} = !__ptr->{clk};
  }}
"#,
            c_ty = c_ty,
            clk = clock.name
        ));
    }
}

fn write_reset<W>(rs_out: &mut W, cpp_out: &mut W, c_ty: &str, reset: &Option<Port>)
where
    W: Write,
{
    if let Some(ref reset) = reset {
        t!(writeln!(
            rs_out,
            r#"        pub fn {c_ty}_{rst}_toggle({c_ty}: *mut {c_ty});"#,
            c_ty = c_ty,
            rst = reset.name
        ));

        t!(writeln!(
            cpp_out,
            r#"  void
  {c_ty}_{rst}_toggle(V{c_ty}* __ptr) {{
    __ptr->{rst} = !__ptr->{rst};
  }}
"#,
            c_ty = c_ty,
            rst = reset.name
        ));
    }
}

fn write_inputs<W>(rs_out: &mut W, cpp_out: &mut W, c_ty: &str, inputs: &[Port])
where
    W: Write,
{
    for input in inputs {
        t!(writeln!(
            rs_out,
            r#"        pub fn {c_ty}_set_{input}({c_ty}: *mut {c_ty}, v: {ffi_ty});"#,
            c_ty = c_ty,
            input = input.name,
            ffi_ty = rust2ffi(&input.ty)
        ));

        t!(writeln!(
            cpp_out,
            r#"  void
  {c_ty}_set_{input}(V{c_ty}* __ptr, {v_ty} __v) {{
    __ptr->{input} = __v;
  }}
"#,
            c_ty = c_ty,
            input = input.name,
            v_ty = rust2ver(&input.ty)
        ));
    }
}

fn write_outputs<W>(rs_out: &mut W, cpp_out: &mut W, c_ty: &str, outputs: &[Port])
where
    W: Write,
{
    for output in outputs {
        t!(writeln!(
            rs_out,
            r#"        pub fn {c_ty}_get_{output}({c_ty}: *mut {c_ty}) -> {ffi_ty};"#,
            c_ty = c_ty,
            output = output.name,
            ffi_ty = rust2ffi(&output.ty)
        ));

        t!(writeln!(
            cpp_out,
            r#"  {v_ty}
  {c_ty}_get_{output}(V{c_ty}* __ptr) {{
    return __ptr->{output};
  }}
"#,
            c_ty = c_ty,
            output = output.name,
            v_ty = rust2ver(&output.ty)
        ));
    }
}

fn write_inouts<W>(rs_out: &mut W, cpp_out: &mut W, c_ty: &str, inouts: &[Port])
where
    W: Write,
{
    for inout in inouts {
        t!(writeln!(
            rs_out,
            r#"        pub fn {c_ty}_set_{inout}({c_ty}: *mut {c_ty}, v: {ffi_ty});
        pub fn {c_ty}_get_{inout}({c_ty}: *mut {c_ty}) -> {ffi_ty};"#,
            c_ty = c_ty,
            inout = inout.name,
            ffi_ty = rust2ffi(&inout.ty)
        ));

        t!(writeln!(
            cpp_out,
            r#"  void
  {c_ty}_set_{inout}(V{c_ty}* __ptr, {v_ty} __v) {{
    __ptr->{inout} = __v;
  }}

  {v_ty}
  {c_ty}_get_{inout}(V{c_ty}* __ptr) {{
    return __ptr->{inout};
  }}
"#,
            c_ty = c_ty,
            inout = inout.name,
            v_ty = rust2ver(&inout.ty)
        ));
    }
}
