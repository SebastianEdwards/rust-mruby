//! MRuby bindings for Rust

#[crate_id="github.com/SebastianEdwards/rust-mruby#mruby:0.1"];

#[comment = "MRuby bindings for Rust"];
#[license = "ASL2"];
#[crate_type = "rlib"];

pub mod raw;

pub struct MRuby {
    mrb: *raw::MrbState
}

impl MRuby {
    pub fn new() -> MRuby {
        MRuby { mrb: unsafe { raw::mrb_open() } }
    }

    pub fn compile_string(&self, source: &str) -> *raw::MrbProc {
        unsafe {
            let context = raw::mrbc_context_new(self.mrb);

            raw::mrbc_filename(self.mrb, context, "rust.rb".to_c_str().unwrap());

            let c_source = source.to_c_str().unwrap();
            let parser = raw::mrb_parse_string(self.mrb, c_source, context);

            raw::mrbc_context_free(self.mrb, context);

            raw::mrb_generate_code(self.mrb, parser)
        }
    }

    pub fn run_proc(&self, ruby_proc: *raw::MrbProc) -> Value {
        unsafe { value_from_raw(raw::mrb_run(self.mrb, ruby_proc, raw::mrb_top_self(self.mrb))) }
    }
}

impl Drop for MRuby {
    fn drop(&mut self) {
        unsafe { raw::mrb_close(self.mrb); }
    }
}

pub enum Value {
    None,
    Bool(bool),
    Fixnum(i32),
    Float(f64),
    String(~str),
}

fn value_from_raw(raw: raw::mruby_Value) -> Value {
    match raw.tt {
        raw::MRB_TT_FALSE => Bool(false),
        raw::MRB_TT_TRUE => Bool(true),
        raw::MRB_TT_FLOAT => unsafe { Float(raw.unchecked_to_f64()) },
        raw::MRB_TT_FIXNUM => unsafe { Fixnum(raw.unchecked_to_i32()) },
        raw::MRB_TT_STRING => unsafe { String(raw.unchecked_to_str()) },
        _ => { None },
    }
}
