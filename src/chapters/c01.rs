pub fn format_string() {
  // Formatted print

  /*
   *  format!: write formatted text to String
   *  print!: same as format! but the text is printed to the console (io::stdout).
   *  println!: same as print! but a newline is appended.
   *  eprint!: same as format! but the text is printed to the standard error (io::stderr).
   *  eprintln!: same as eprint!but a newline is appended.
   */

  println!("{} days", 31);
  // Without a suffix, 31 becomes an i32. You can change what type 31 is
  // by providing a suffix.

  // As can named arguments.
  println!(
    "{subject} {verb} {object}",
    object = "the lazy dog",
    subject = "the quick brown fox",
    verb = "jumps over"
  );

  // Special formatting can be specified after a `:`.
  println!(
    "{} of {:b} people know binary, the other half doesn't",
    1, 2
  );

  // You can right-align text with a specified width. This will output
  // "     1". 5 white spaces and a "1".
  println!("{number:>width$}", number = 1, width = 6);

  // You can pad numbers with extra zeroes. This will output "000001".
  println!("{number:>0width$}", number = 1, width = 6);

  // ACTIVITIES

  // A01 Fix the examples
  println!("My name is {0}, {1} {0}", "Bond", "James");

  // Derive the `fmt::Debug` implementation for `Structure`. `Structure`
  // is a structure which contains a single `i32`.
  #[derive(Debug)]
  struct Structure(i32);
  // The problem with `derive` is there is no control over how
  // the results look. What if I want this to just show a `3`?
  println!("This struct `{:?}` ~won't~ WILL print...", Structure(3));

  // A02: Add a println! macro that prints: Pi is roughly 3.142 by controlling the number of decimal places shown.
  let pi: f32 = 3.141562654;
  println!("Pi number is {0:.3}", pi)
}

pub fn display() {
  // Import (via `use`) the `fmt` module to make it available.
  use std::fmt;

  // Define a structure for which `fmt::Display` will be implemented. This is
  // a tuple struct named `Structure` that contains an `i32`.
  struct Structure(i32);

  // To use the `{}` marker, the trait `fmt::Display` must be implemented
  // manually for the type.
  impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      // Write strictly the first element into the supplied output
      // stream: `f`. Returns `fmt::Result` which indicates whether the
      // operation succeeded or failed. Note that `write!` uses syntax which
      // is very similar to `println!`.
      write!(f, "{}", self.0)
    }
  }

  println!("Now the structure `{}` is printed with display", Structure(7));

  // ACTIVITIES

  impl fmt::Debug for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      // Write strictly the first element into the supplied output
      // stream: `f`. Returns `fmt::Result` which indicates whether the
      // operation succeeded or failed. Note that `write!` uses syntax which
      // is very similar to `println!`.
      write!(f, "Structure value: {}", self.0)
    }
  }

  println!("Structure debugged: {:?}", Structure(6));
}
