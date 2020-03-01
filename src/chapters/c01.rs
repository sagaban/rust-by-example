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

  println!(
    "Now the structure `{}` is printed with display",
    Structure(7)
  );

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

pub fn testcase_list() {
  use std::fmt; // Import the `fmt` module.

  // Define a structure named `List` containing a `Vec`.
  struct List(Vec<i32>);

  impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      // Extract the value using tuple indexing,
      // and create a reference to `vec`.
      let vec = &self.0;

      write!(f, "[")?;
      // try!(write!(f, "["));

      // Iterate over `v` in `vec` while enumerating the iteration
      // count in `count`.
      for (count, v) in vec.iter().enumerate() {
        // For every element except the first, add a comma.
        // Use the ? operator, or try!, to return on errors.
        if count != 0 {
          write!(f, ", ")?;
        }
        write!(f, "{}", v)?;
      }

      // Close the opened bracket and return a fmt::Result value.
      write!(f, "]")
    }
  }

  let mut v = List(vec![1, 2, 3]);
  println!("len {}", v.0.len());
  v.0.push(4);
  println!("{}", v);

  // ACTIVITIES
  impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      // Extract the value using tuple indexing,
      // and create a reference to `vec`.
      let vec = &self.0;

      write!(f, "[")?;
      // try!(write!(f, "["));

      // Iterate over `v` in `vec` while enumerating the iteration
      // count in `count`.
      for (count, v) in vec.iter().enumerate() {
        // For every element except the first, add a comma.
        // Use the ? operator, or try!, to return on errors.
        if count != 0 {
          write!(f, ", ")?;
        }
        write!(f, "{}: '{}'", count, v)?;
      }

      // Close the opened bracket and return a fmt::Result value.
      write!(f, "]")
    }
  }

  let v2 = List(vec![9, 8, 33]);

  println!("{:?}", v2);
}

pub fn formatting() {
  /*
   * This formatting functionality is implemented via traits, and there is one trait
   * for each argument type. The most common formatting trait is Display, which
   * handles cases where the argument type is left unspecified: {} for instance.
   */

  use std::fmt::{self, Display, Formatter};

  struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
  }

  impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
      let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

      // `write!` is like `format!`, but it will write the formatted string
      // into a buffer (the first argument)
      write!(
        f,
        "{}: {:.3}°{} {:.3}°{}",
        self.name,
        self.lat.abs(),
        lat_c,
        self.lon.abs(),
        lon_c
      )
    }
  }

  #[derive(Debug)]
  struct Color {
    red: u8,
    green: u8,
    blue: u8,
  }

  for city in [
    City {
      name: "Dublin",
      lat: 53.347778,
      lon: -6.259722,
    },
    City {
      name: "Oslo",
      lat: 59.95,
      lon: 10.75,
    },
    City {
      name: "Vancouver",
      lat: 49.25,
      lon: -123.1,
    },
  ]
  .iter()
  {
    println!("{}", *city);
  }

  let colors = [
    Color {
      red: 128,
      green: 255,
      blue: 90,
    },
    Color {
      red: 0,
      green: 3,
      blue: 254,
    },
    Color {
      red: 0,
      green: 0,
      blue: 0,
    },
  ];

  for color in colors.iter() {
    println!("{:?}", *color);
  }

  impl Display for Color {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      // let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };

      let exa = format!("{:02X}{:02X}{:02X}", self.red, self.green, self.blue);

      // `write!` is like `format!`, but it will write the formatted string
      // into a buffer (the first argument)
      write!(
        f,
        "RGB ({}, {}, {}) 0x{}",
        self.red, self.green, self.blue, exa
      )

      // Alternative

      // write!(
      //   f,
      //   "RGB ({red}, {green}, {blue}) 0x{red:02X}{green:02X}{blue:02X}",
      //   red = self.red,
      //   green = self.green, blue=self.blue
      // )
    }
  }

  for color in colors.iter() {
    println!("{}", *color);
  }
}
