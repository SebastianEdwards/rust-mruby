extern mod mruby;

#[no_mangle]
extern "C" fn rusty_method() {
    println!("I've become a Ruby method!");
}

fn main() {
    use mruby::{MRuby};

    let ruby_land = MRuby::new();
    ruby_land.create_global_method("rusty_method", rusty_method);
    let ruby_proc = ruby_land.compile_string("rusty_method");
    ruby_land.run_proc(ruby_proc);
}
