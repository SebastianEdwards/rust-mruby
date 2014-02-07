extern mod mruby;

fn main() {
    use mruby::{MRuby, Bool, String, Fixnum, Float, None};

    let ruby_land = MRuby::new();
    let ruby_proc = ruby_land.compile_string("'Hello, world!'");

    let result = ruby_land.run_proc(ruby_proc);

    match result {
        Bool(b) => println!("The return value was a bool: {:?}", b),
        String(s) => println!("The return value was a string: {:?}", s),
        Fixnum(i) => println!("The return value was a fixnum: {:?}", i),
        Float(f) => println!("The return value was a float: {:?}", f),
        None => println!("I don't know how to handle this return type yet"),
    }
}
