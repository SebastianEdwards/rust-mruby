extern mod mruby;

fn main() {
  let rubyland = mruby::MRuby::new();
  let ruby_proc = rubyland.compile_string("'Hello, world!'");

  let result = rubyland.run_proc(ruby_proc);

  match result.to_str() {
    Some(string) => println!("The return value was: {:?}", string),
    None => println!("The return value was not a string")
  }
}
