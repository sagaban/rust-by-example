pub fn primitives_and_operations() {
  /*
   * Primitives
   * Rust provides access to a wide variety of primitives. A sample includes:
   *
   * Scalar Types
   *  - signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
   *  - unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
   *  - floating point: f32, f64
   *  - char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
   *  - bool either true or false
   *  - and the unit type (), whose only possible value is an empty tuple: ()
   *
   * Despite the value of a unit type being a tuple, it is not considered a compound
   * type because it does not contain multiple values.
   *
   * Compound Types
   * - arrays like [1, 2, 3]
   * - tuples like (1, true)
   */

  // Variables can be type annotated.
  let logical: bool = true;

  let a_float: f64 = 1.0; // Regular annotation
  let an_integer = 5i32; // Suffix annotation

  // Or a default will be used.
  let default_float = 3.0; // `f64`
  let default_integer = 7; // `i32`

  // A type can also be inferred from context
  let mut inferred_type = 12; // Type i64 is inferred from another line
  inferred_type = 4294967296i64;

  // A mutable variable's value can be changed.
  let mut mutable = 12; // Mutable `i32`
  mutable = 21;

  // Error! The type of a variable can't be changed.
  // mutable = true;

  // Variables can be overwritten with shadowing.
  let mutable = true;

  // Literals and operators

  // Integer addition
  println!("1 + 2 = {}", 1u32 + 2);

  // Integer subtraction
  println!("1 - 2 = {}", 1i32 - 2);
  // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
  // This is a problem only if the result is negative
  // 3u32 - 2 will work

  // Short-circuiting boolean logic
  println!("true AND false is {}", true && false);
  println!("true OR false is {}", true || false);
  println!("NOT true is {}", !true);

  // Bitwise operations
  println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
  println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
  println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
  println!("1 << 5 is {}", 1u32 << 5);
  println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

  // Use underscores to improve readability!
  println!("One million is written as {}", 1_000_000u32);
}
