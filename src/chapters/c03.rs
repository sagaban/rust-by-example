/* Custom Types
 *
 * Rust custom data types are formed mainly through the two keywords:
 *
 * struct: define a structure
 * enum: define an enumeration
 * Constants can also be created via the `const` and `static` keywords.
 */

pub fn structures() {
  /*
   * Structures
   * There are three types of structures ("structs") that can be created using the
   * struct keyword:
   *
   * Tuple structs, which are, basically, named tuples.
   * The classic C structs
   * Unit structs, which are field-less, are useful for generics.
   */

  #[derive(Debug)]
  struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
  }

  // A unit struct
  struct Nil;

  // A tuple struct
  struct Pair(i32, f32);

  // A struct with two fields
  struct Point {
    x: f32,
    y: f32,
  }

  // Structs can be reused as fields of another struct
  #[allow(dead_code)]
  struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
  }

  // Create struct with field init shorthand
  let name = "Peter";
  let age = 27;
  let peter = Person { name, age };

  // Print debug struct
  println!("{:?}", peter);

  // Instantiate a `Point`
  let point: Point = Point { x: 10.3, y: 0.4 };

  // Access the fields of the point
  println!("point coordinates: ({}, {})", point.x, point.y);

  // Make a new point by using struct update syntax to use the fields of our
  // other one
  let bottom_right = Point { x: 5.2, ..point };

  // `bottom_right.y` will be the same as `point.y` because we used that field
  // from `point`
  println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

  // Destructure the point using a `let` binding
  let Point {
    x: top_edge,
    y: left_edge,
  } = point;

  let _rectangle = Rectangle {
    // struct instantiation is an expression too
    top_left: Point {
      x: left_edge,
      y: top_edge,
    },
    bottom_right: bottom_right,
  };

  // Instantiate a unit struct
  let _nil = Nil;

  // Instantiate a tuple struct
  let pair = Pair(1, 0.1);

  // Access the fields of a tuple struct
  println!("pair contains {:?} and {:?}", pair.0, pair.1);

  // Destructure a tuple struct
  let Pair(integer, decimal) = pair;

  println!("pair contains {:?} and {:?}", integer, decimal);

  // ACTIVITIES
  // A01
  use std::fmt; // Import the `fmt` module.

  impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let Point { x: top, y: left } = self.top_left;
      let Point {
        x: bottom,
        y: right,
      } = self.bottom_right;

      // `write!` is like `format!`, but it will write the formatted string
      // into a buffer (the first argument)
      write!(
        f,
        "top-left: ({}, {}), bottom-right: ({}, {})",
        top, left, bottom, right
      )
    }
  }

  fn cal_area(r: &Rectangle) -> f32 {
    let Point { x: top, y: left } = r.top_left;
    let Point {
      x: bottom,
      y: right,
    } = r.bottom_right;
    return (top - bottom) * (right - left);
  }
  println!(
    "The area of the rectangle {} is {}",
    _rectangle,
    cal_area(&_rectangle)
  );

  //A02
  fn create_square(p: Point, borders_len: f32) -> Rectangle {
    let Point { x: bottom, y: left } = p;
    let top_left = Point {
      x: bottom + borders_len,
      y: left,
    };
    let bottom_right = Point {
      x: bottom,
      y: left + borders_len,
    };
    Rectangle {
      top_left,
      bottom_right,
    }
  }

  let base_point = Point { x: 2., y: 1. };

  println!("Square {}", create_square(base_point, 4.4))
}

pub fn enums() {
  /*
   * The enum keyword allows the creation of a type which may be one of a few
   * different variants. Any variant which is valid as a struct is also valid as
   * an enum.
   */

  // Create an `enum` to classify a web event. Note how both
  // names and type information together specify the variant:
  // `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
  // Each is different and independent.
  enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
  }

  // A function which takes a `WebEvent` enum as an argument and
  // returns nothing.
  fn inspect(event: WebEvent) {
    match event {
      WebEvent::PageLoad => println!("page loaded"),
      WebEvent::PageUnload => println!("page unloaded"),
      // Destructure `c` from inside the `enum`.
      WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
      WebEvent::Paste(s) => println!("pasted \"{}\".", s),
      // Destructure `Click` into `x` and `y`.
      WebEvent::Click { x, y } => {
        println!("clicked at x={}, y={}.", x, y);
      }
    }
  }

  let pressed = WebEvent::KeyPress('x');
  // `to_owned()` creates an owned `String` from a string slice.
  let pasted = WebEvent::Paste("my text".to_owned());
  let click = WebEvent::Click { x: 20, y: 80 };
  let load = WebEvent::PageLoad;
  let unload = WebEvent::PageUnload;

  inspect(pressed);
  inspect(pasted);
  inspect(click);
  inspect(load);
  inspect(unload);
}

pub fn alias() {
  /*
   * Type aliases
   * If you use a type alias, you can refer to each enum variant via its alias.
   * This might be useful if the enum's name is too long or too generic, and you
   * want to rename it.
   */

  enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
  }

  // Creates a type alias
  type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

  // We can refer to each variant via its alias, not its long and inconvenient
  // name.
  let x = Operations::Add;

  // The most common place you'll see this is in impl blocks using the Self alias.
  impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
      match self {
        Self::Add => x + y,
        Self::Subtract => x - y,
      }
    }
  }

  println!("{}", x.run(3, 5));
  println!("{}", Operations::run(&x, 4, 8));
}

pub fn _use() {
  // An attribute to hide warnings for unused code.
  #![allow(dead_code)]

  enum Status {
    Rich,
    Poor,
  }
  enum Work {
    Civilian,
    Soldier,
  }

  // Explicitly `use` each name so they are available without
  // manual scoping.
  use Status::{Poor, Rich};
  // Automatically `use` each name inside `Work`.
  use Work::*;

  // Equivalent to `Status::Poor`.
  let status = Poor;
  // Equivalent to `Work::Civilian`.
  let work = Civilian;

  match status {
    // Note the lack of scoping because of the explicit `use` above.
    Rich => println!("The rich have lots of money!"),
    Poor => println!("The poor have no money..."),
  }

  match work {
    // Note again the lack of scoping.
    Civilian => println!("Civilians work!"),
    Soldier => println!("Soldiers fight!"),
  }
}

pub fn c_liKe_enums() {
  // An attribute to hide warnings for unused code.
  #![allow(dead_code)]

  // enum with implicit discriminator (starts at 0)
  enum Number {
    Zero,
    One,
    Two,
  }

  // enum with explicit discriminator
  enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
  }

  fn main() {
    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
    println!("grass is #{:06x}", Color::Green as i32);
  }
  main();
}

pub fn testcase_linked_list() {
  use List::*;

  enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
  }

  // Methods can be attached to an enum
  impl List {
    // Create an empty list
    fn new() -> List {
      // `Nil` has type `List`
      Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
      // `Cons` also has type List
      Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
      // `self` has to be matched, because the behavior of this method
      // depends on the variant of `self`
      // `self` has type `&List`, and `*self` has type `List`, matching on a
      // concrete type `T` is preferred over a match on a reference `&T`
      match *self {
        // Can't take ownership of the tail, because `self` is borrowed;
        // instead take a reference to the tail
        // Cons(head, ref tail) => {
        // println!("getting length {}", head);
        //   return 1 + tail.len();
        // }
        // Base Case: An empty list has zero length
        Cons(_, ref tail) => 1 + tail.len(),
        Nil => 0,
      }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
      match *self {
        Cons(head, ref tail) => {
          // `format!` is similar to `print!`, but returns a heap
          // allocated string instead of printing to the console
          format!("{}, {}", head, tail.stringify())
        }
        Nil => format!("Nil"),
      }
    }
  }

  fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
  }
  main();
}

pub fn constants() {
  /*
   * Rust has two different types of constants which can be declared in any scope
   * including global. Both require explicit type annotation:
   *
   * const: An unchangeable value (the common case)
   * static: A possibly mutable variable with 'static lifetime. The static lifetime
   * is inferred and does not have to be specified. Accessing or modifying a mutable
   * static variable is unsafe.
   */

  // Globals are declared outside all other scopes.
  static LANGUAGE: &str = "Rust";
  const THRESHOLD: i32 = 10;

  fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
  }

  fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line
  }
  main();
}
