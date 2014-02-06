//! MRuby bindings for Rust

#[crate_id="github.com/SebastianEdwards/rust-mruby#mruby:0.1"];

#[comment = "MRuby bindings for Rust"];
#[license = "MIT"];
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

  pub fn run_proc(&self, ruby_proc: *raw::MrbProc) -> raw::mruby_Value {
    unsafe {
      raw::mrb_run(self.mrb, ruby_proc, raw::mrb_top_self(self.mrb))
    }
  }
}

impl Drop for MRuby {
  fn drop(&mut self) {
    unsafe { raw::mrb_close(self.mrb); }
  }
}
