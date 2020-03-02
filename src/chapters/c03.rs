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
      bottom_right
    }
  }

  let base_point = Point { x: 2., y: 1. };

  println!("Square {}", create_square(base_point, 4.4))
}
