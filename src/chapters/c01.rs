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

  // 01 Fix the examples
  println!("My name is {0}, {1} {0}", "Bond", "James");

  #[derive(Debug)]
  struct Structure(i32);
  println!("This struct `{:?}` ~won't~ WILL print...", Structure(3));

  // A02: Add a println! macro that prints: Pi is roughly 3.142 by controlling the number of decimal places shown.
  let Pi: f32 = 3.141562654;
  println!("Pi number is {0:.3}", Pi)
}
