use clap::Args;

pub fn example_fn() {
  let s = "This is from the example command";
  println!("{}", s);
}

#[derive(Args)]
pub struct ExampleArguments {
    arg: String,
}

pub fn example_fn_with_args(args: &ExampleArguments) {
  println!("{}", args.arg);
}
