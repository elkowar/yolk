# Rhai Standard Library builtins
Rhai standard library functions.

Note that the typesignatures here do look a bit weird.
This is simply a result of how we generate the documentation,
and can't easily be improved.

Just try your best to ignore it...

---

**namespace**: `global`

---

## Array.is_empty

```rust,ignore
get$is_empty(array: &mut Array) -> bool
```

> Return true if the array is empty.

---
## Array.len

```rust,ignore
get$len(array: &mut Array) -> i64
```

> Number of elements in the array.

---
## Blob.is_empty

```rust,ignore
get$is_empty(blob: &mut Blob) -> bool
```

> Return true if the BLOB is empty.

---
## Blob.len

```rust,ignore
get$len(blob: &mut Blob) -> i64
```

> Return the length of the BLOB.
> 
> #### Example
> 
> ```rhai
> let b = blob(10, 0x42);
> 
> print(b);           // prints "[4242424242424242 4242]"
> 
> print(b.len());     // prints 10
> ```

---
## E

```rust,ignore
E() -> f64
```

> Return the natural number _e_.

---
## Instant.elapsed

```rust,ignore
get$elapsed(timestamp: Instant) -> Result<Dynamic, Box<EvalAltResult>>
```

> Return the number of seconds between the current system time and the timestamp.
> 
> #### Example
> 
> ```rhai
> let now = timestamp();
> 
> sleep(10.0);            // sleep for 10 seconds
> 
> print(now.elapsed);     // prints 10.???
> ```

---
## PI

```rust,ignore
PI() -> f64
```

> Return the number π.

---
## Range<int>.end

```rust,ignore
get$end(range: &mut Range<i64>) -> i64
```

> Return the end of the exclusive range.

---
## Range<int>.is_empty

```rust,ignore
get$is_empty(range: &mut Range<i64>) -> bool
```

> Return true if the range contains no items.

---
## Range<int>.is_exclusive

```rust,ignore
get$is_exclusive(range: &mut Range<i64>) -> bool
```

> Return `true` if the range is exclusive.

---
## Range<int>.is_inclusive

```rust,ignore
get$is_inclusive(range: &mut Range<i64>) -> bool
```

> Return `true` if the range is inclusive.

---
## Range<int>.start

```rust,ignore
get$start(range: &mut Range<i64>) -> i64
```

> Return the start of the exclusive range.

---
## RangeInclusive<int>.end

```rust,ignore
get$end(range: &mut RangeInclusive<i64>) -> i64
```

> Return the end of the inclusive range.

---
## RangeInclusive<int>.is_empty

```rust,ignore
get$is_empty(range: &mut RangeInclusive<i64>) -> bool
```

> Return true if the range contains no items.

---
## RangeInclusive<int>.is_exclusive

```rust,ignore
get$is_exclusive(range: &mut RangeInclusive<i64>) -> bool
```

> Return `true` if the range is exclusive.

---
## RangeInclusive<int>.is_inclusive

```rust,ignore
get$is_inclusive(range: &mut RangeInclusive<i64>) -> bool
```

> Return `true` if the range is inclusive.

---
## RangeInclusive<int>.start

```rust,ignore
get$start(range: &mut RangeInclusive<i64>) -> i64
```

> Return the start of the inclusive range.

---
## String.bytes

```rust,ignore
get$bytes(string: &str) -> i64
```

> Return the length of the string, in number of bytes used to store it in UTF-8 encoding.
> 
> #### Example
> 
> ```rhai
> let text = "朝には紅顔ありて夕べには白骨となる";
> 
> print(text.bytes);      // prints 51
> ```

---
## String.chars

```rust,ignore
get$chars(string: &str) -> CharsStream
```

> Return an iterator over all the characters in the string.
> 
> #### Example
> 
> ```rhai
> for ch in "hello, world!".chars {"
> print(ch);
> }
> ```

---
## String.is_empty

```rust,ignore
get$is_empty(string: &str) -> bool
```

> Return true if the string is empty.

---
## String.len

```rust,ignore
get$len(string: &str) -> i64
```

> Return the length of the string, in number of characters.
> 
> #### Example
> 
> ```rhai
> let text = "朝には紅顔ありて夕べには白骨となる";
> 
> print(text.len);        // prints 17
> ```

---
## abs

```rust,ignore
abs(x: f64) -> f64
abs(x: i64) -> Result<i64, Box<EvalAltResult>>
abs(x: f32) -> f32
abs(x: i128) -> Result<i128, Box<EvalAltResult>>
abs(x: i8) -> Result<i8, Box<EvalAltResult>>
abs(x: i16) -> Result<i16, Box<EvalAltResult>>
abs(x: i32) -> Result<i32, Box<EvalAltResult>>
```

> Return the absolute value of the floating-point number.

---
## acos

```rust,ignore
acos(x: f64) -> f64
```

> Return the arc-cosine of the floating-point number, in radians.

---
## acosh

```rust,ignore
acosh(x: f64) -> f64
```

> Return the arc-hyperbolic-cosine of the floating-point number, in radians.

---
## all

```rust,ignore
all(array: &mut Array, filter: FnPtr) -> Result<bool, Box<EvalAltResult>>
all(array: &mut Array, filter: &str) -> Result<bool, Box<EvalAltResult>>
```

> Return `true` if all elements in the array return `true` when applied the `filter` function.
> 
> #### No Function Parameter
> 
> Array element (mutable) is bound to `this`.
> 
> This method is marked _pure_; the `filter` function should not mutate array elements.
> 
> #### Function Parameters
> 
> * `element`: copy of array element
> * `index` _(optional)_: current index in the array
> 
> #### Example
> 
> ```rhai
> let x = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 5];
> 
> print(x.all(|v| v > 3));        // prints false
> 
> print(x.all(|v| v > 1));        // prints true
> 
> print(x.all(|v, i| i > v));     // prints false
> ```

---
## append

```rust,ignore
append(blob1: &mut Blob, blob2: Blob)
append(string: &mut ImmutableString, mut item: Dynamic)
append(blob: &mut Blob, character: char)
append(string: &mut ImmutableString, utf8: Blob)
append(array: &mut Array, new_array: Array)
append(blob: &mut Blob, string: &str)
append(blob: &mut Blob, value: i64)
```

> Add another BLOB to the end of the BLOB.
> 
> #### Example
> 
> ```rhai
> let b1 = blob(5, 0x42);
> let b2 = blob(3, 0x11);
> 
> b1.push(b2);
> 
> print(b1);      // prints "[4242424242111111]"
> ```

---
## as_string

```rust,ignore
as_string(blob: Blob) -> String
```

> Convert the BLOB into a string.
> 
> The byte stream must be valid UTF-8, otherwise an error is raised.
> 
> #### Example
> 
> ```rhai
> let b = blob(5, 0x42);
> 
> let x = b.as_string();
> 
> print(x);       // prints "FFFFF"
> ```

---
## asin

```rust,ignore
asin(x: f64) -> f64
```

> Return the arc-sine of the floating-point number, in radians.

---
## asinh

```rust,ignore
asinh(x: f64) -> f64
```

> Return the arc-hyperbolic-sine of the floating-point number, in radians.

---
## atan

```rust,ignore
atan(x: f64) -> f64
atan(x: f64, y: f64) -> f64
```

> Return the arc-tangent of the floating-point number, in radians.

---
## atanh

```rust,ignore
atanh(x: f64) -> f64
```

> Return the arc-hyperbolic-tangent of the floating-point number, in radians.

---
## bits

```rust,ignore
bits(value: i64) -> Result<BitRange, Box<EvalAltResult>>
bits(value: i64, range: RangeInclusive<i64>) -> Result<BitRange, Box<EvalAltResult>>
bits(value: i64, range: Range<i64>) -> Result<BitRange, Box<EvalAltResult>>
bits(value: i64, from: i64) -> Result<BitRange, Box<EvalAltResult>>
bits(value: i64, from: i64, len: i64) -> Result<BitRange, Box<EvalAltResult>>
```

> Return an iterator over all the bits in the number.
> 
> #### Example
> 
> ```rhai
> let x = 123456;
> 
> for bit in x.bits() {
> print(bit);
> }
> ```

---
## blob

```rust,ignore
blob() -> Blob
blob(len: i64) -> Result<Blob, Box<EvalAltResult>>
blob(len: i64, value: i64) -> Result<Blob, Box<EvalAltResult>>
```

> Return a new, empty BLOB.

---
## bytes

```rust,ignore
bytes(string: &str) -> i64
```

> Return the length of the string, in number of bytes used to store it in UTF-8 encoding.
> 
> #### Example
> 
> ```rhai
> let text = "朝には紅顔ありて夕べには白骨となる";
> 
> print(text.bytes);      // prints 51
> ```

---
## ceiling

```rust,ignore
ceiling(x: f64) -> f64
```

> Return the smallest whole number larger than or equals to the floating-point number.

---
## chars

```rust,ignore
chars(string: &str) -> CharsStream
chars(string: &str, range: Range<i64>) -> CharsStream
chars(string: &str, range: RangeInclusive<i64>) -> CharsStream
chars(string: &str, start: i64) -> CharsStream
chars(string: &str, start: i64, len: i64) -> CharsStream
```

> Return an iterator over the characters in the string.
> 
> #### Example
> 
> ```rhai
> for ch in "hello, world!".chars() {
> print(ch);
> }
> ```

---
## chop

```rust,ignore
chop(array: &mut Array, len: i64)
chop(blob: &mut Blob, len: i64)
```

> Cut off the head of the array, leaving a tail of the specified length.
> 
> * If `len` ≤ 0, the array is cleared.
> * If `len` ≥ length of array, the array is not modified.
> 
> #### Example
> 
> ```rhai
> let x = [1, 2, 3, 4, 5];
> 
> x.chop(3);
> 
> print(x);       // prints "[3, 4, 5]"
> 
> x.chop(10);
> 
> print(x);       // prints "[3, 4, 5]"
> ```

---
## clear

```rust,ignore
clear(blob: &mut Blob)
clear(array: &mut Array)
clear(string: &mut ImmutableString)
clear(map: &mut Map)
```

> Clear the BLOB.

---
## contains

```rust,ignore
contains(array: &mut Array, value: Dynamic) -> Result<bool, Box<EvalAltResult>>
contains(string: &str, match_string: &str) -> bool
contains(range: &mut RangeInclusive<i64>, value: i64) -> bool
contains(string: &str, character: char) -> bool
contains(blob: &mut Blob, value: i64) -> bool
contains(map: &mut Map, property: &str) -> bool
contains(range: &mut Range<i64>, value: i64) -> bool
```

> Return `true` if the array contains an element that equals `value`.
> 
> The operator `==` is used to compare elements with `value` and must be defined,
> otherwise `false` is assumed.
> 
> This function also drives the `in` operator.
> 
> #### Example
> 
> ```rhai
> let x = [1, 2, 3, 4, 5];
> 
> // The 'in' operator calls 'contains' in the background
> if 4 in x {
> print("found!");
> }
> ```

---
## cos

```rust,ignore
cos(x: f64) -> f64
```

> Return the cosine of the floating-point number in radians.

---
## cosh

```rust,ignore
cosh(x: f64) -> f64
```

> Return the hyperbolic cosine of the floating-point number in radians.

---
## crop

```rust,ignore
crop(string: &mut ImmutableString, range: RangeInclusive<i64>)
crop(string: &mut ImmutableString, start: i64)
crop(string: &mut ImmutableString, range: Range<i64>)
crop(string: &mut ImmutableString, start: i64, len: i64)
```

> Remove all characters from the string except those within an inclusive `range`.
> 
> #### Example
> 
> ```rhai
> let text = "hello, world!";
> 
> text.crop(2..=8);
> 
> print(text);        // prints "llo, wo"
> ```

---
## debug

```rust,ignore
debug() -> ImmutableString
debug(unit: ()) -> ImmutableString
debug(number: f32) -> ImmutableString
debug(map: &mut Map) -> ImmutableString
debug(number: f64) -> ImmutableString
debug(item: &mut Dynamic) -> ImmutableString
debug(array: &mut Array) -> ImmutableString
debug(character: char) -> ImmutableString
debug(f: &mut FnPtr) -> ImmutableString
debug(value: bool) -> ImmutableString
debug(string: &str) -> ImmutableString
```

> Return the empty string.

---
## dedup

```rust,ignore
dedup(array: &mut Array)
dedup(array: &mut Array, comparer: &str) -> Result<(), Box<EvalAltResult>>
dedup(array: &mut Array, comparer: FnPtr)
```

> Remove duplicated _consecutive_ elements from the array.
> 
> The operator `==` is used to compare elements and must be defined,
> otherwise `false` is assumed.
> 
> #### Example
> 
> ```rhai
> let x = [1, 2, 2, 2, 3, 4, 3, 3, 2, 1];
> 
> x.dedup();
> 
> print(x);       // prints "[1, 2, 3, 4, 3, 2, 1]"
> ```

---
## drain

```rust,ignore
drain(array: &mut Array, filter: FnPtr) -> Result<Array, Box<EvalAltResult>>
drain(blob: &mut Blob, range: RangeInclusive<i64>) -> Blob
drain(map: &mut Map, filter: FnPtr) -> Result<Map, Box<EvalAltResult>>
drain(array: &mut Array, filter: &str) -> Result<Array, Box<EvalAltResult>>
drain(blob: &mut Blob, range: Range<i64>) -> Blob
drain(array: &mut Array, range: RangeInclusive<i64>) -> Array
drain(array: &mut Array, range: Range<i64>) -> Array
drain(blob: &mut Blob, start: i64, len: i64) -> Blob
drain(array: &mut Array, start: i64, len: i64) -> Array
```

> Remove all elements in the array that returns `true` when applied the `filter` function and
> return them as a new array.
> 
> #### No Function Parameter
> 
> Array element (mutable) is bound to `this`.
> 
> #### Function Parameters
> 
> * `element`: copy of array element
> * `index` _(optional)_: current index in the array
> 
> #### Example
> 
> ```rhai
> let x = [1, 2, 3, 4, 5];
> 
> let y = x.drain(|v| v < 3);
> 
> print(x);       // prints "[3, 4, 5]"
> 
> print(y);       // prints "[1, 2]"
> 
> let z = x.drain(|v, i| v + i > 5);
> 
> print(x);       // prints "[3, 4]"
> 
> print(z);       // prints "[5]"
> ```

---
## elapsed

```rust,ignore
elapsed(timestamp: Instant) -> Result<Dynamic, Box<EvalAltResult>>
```

> Return the number of seconds between the current system time and the timestamp.
> 
> #### Example
> 
> ```rhai
> let now = timestamp();
> 
> sleep(10.0);            // sleep for 10 seconds
> 
> print(now.elapsed);     // prints 10.???
> ```

---
## end

```rust,ignore
end(range: &mut Range<i64>) -> i64
end(range: &mut RangeInclusive<i64>) -> i64
```

> Return the end of the exclusive range.

---
## ends_with

```rust,ignore
ends_with(string: &str, match_string: &str) -> bool
```

> Return `true` if the string ends with a specified string.
> 
> #### Example
> 
> ```rhai
> let text = "hello, world!";
> 
> print(text.ends_with("world!"));    // prints true
> 
> print(text.ends_with("hello"));     // prints false
> ```

---
## exit

```rust,ignore
exit() -> Result<Dynamic, Box<EvalAltResult>>
exit(value: Dynamic) -> Result<Dynamic, Box<EvalAltResult>>
```

> Exit the script evaluation immediately with `()` as exit value.
> 
> #### Example
> ```rhai
> exit();
> ```

---
## exp

```rust,ignore
exp(x: f64) -> f64
```

> Return the exponential of the floating-point number.

---
## extract

```rust,ignore
extract(blob: &mut Blob, range: RangeInclusive<i64>) -> Blob
extract(array: &mut Array, range: RangeInclusive<i64>) -> Array
extract(array: &mut Array, start: i64) -> Array
extract(blob: &mut Blob, range: Range<i64>) -> Blob
extract(blob: &mut Blob, start: i64) -> Blob
extract(array: &mut Array, range: Range<i64>) -> Array
extract(blob: &mut Blob, start: i64, len: i64) -> Blob
extract(array: &mut Array, start: i64, len: i64) -> Array
```

> Copy an inclusive `range` of the BLOB and return it as a new BLOB.
> 
> #### Example
> 
> ```rhai
> let b = blob();
> 
> b += 1; b += 2; b += 3; b += 4; b += 5;
> 
> print(b.extract(1..=3));    // prints "[020304]"
> 
> print(b);                   // prints "[0102030405]"
> ```

---
## f32.is_zero

```rust,ignore
get$is_zero(x: f32) -> bool
```

> Return true if the floating-point number is zero.

---
## fill_with

```rust,ignore
fill_with(map: &mut Map, map2: Map)
```

> Add all property values of another object map into the object map.
> Only properties that do not originally exist in the object map are added.
> 
> #### Example
> 
> ```rhai
> let m = #{a:1, b:2, c:3};
> let n = #{a: 42, d:0};
> 
> m.fill_with(n);
> 
> print(m);       // prints "#{a:1, b:2, c:3, d:0}"
> ```

---
## filter

```rust,ignore
filter(array: &mut Array, filter_func: &str) -> Result<Array, Box<EvalAltResult>>
filter(array: &mut Array, filter: FnPtr) -> Result<Array, Box<EvalAltResult>>
filter(map: &mut Map, filter: FnPtr) -> Result<Map, Box<EvalAltResult>>
```

> Iterate through all the elements in the array, applying a function named by `filter` to each
> element in turn, and return a copy of all elements (in order) that return `true` as a new array.
> 
> #### Deprecated API
> 
> This method is deprecated and will be removed from the next major version.
> Use `array.filter(Fn("fn_name"))` instead.
> 
> #### Function Parameters
> 
> A function with the same name as the value of `filter` must exist taking these parameters:
> 
> * `element`: copy of array element
> * `index` _(optional)_: current index in the array
> 
> #### Example
> 
> ```rhai
> fn screen(x, i) { x * i >= 10 }
> 
> let x = [1, 2, 3, 4, 5];
> 
> let y = x.filter("is_odd");
> 
> print(y);       // prints "[1, 3, 5]"
> 
> let y = x.filter("screen");
> 
> print(y);       // prints "[12, 20]"
> ```

---
## find

```rust,ignore
find(array: &mut Array, filter: FnPtr) -> Result<Dynamic, Box<EvalAltResult>>
find(array: &mut Array, filter: FnPtr, start: i64) -> Result<Dynamic, Box<EvalAltResult>>
```

> Iterate through all the elements in the array, applying a `filter` function to each element
> in turn, and return a copy of the first element that returns `true`. If no element returns
> `true`, `()` is returned.
> 
> #### No Function Parameter
> 
> Array element (mutable) is bound to `this`.
> 
> #### Function Parameters
> 
> * `element`: copy of array element
> * `index` _(optional)_: current index in the array
> 
> #### Example
> 
> ```rhai
> let x = [1, 2, 3, 5, 8, 13];
> 
> print(x.find(|v| v > 3));                    // prints 5: 5 > 3
> 
> print(x.find(|v| v > 13) ?? "not found");    // prints "not found": nothing is > 13
> 
> print(x.find(|v, i| v * i > 13));            // prints 5: 3 * 5 > 13
> ```

---
## find_map

```rust,ignore
find_map(array: &mut Array, filter: FnPtr) -> Result<Dynamic, Box<EvalAltResult>>
find_map(array: &mut Array, filter: FnPtr, start: i64) -> Result<Dynamic, Box<EvalAltResult>>
```

> Iterate through all the elements in the array, applying a `mapper` function to each element
> in turn, and return the first result that is not `()`. Otherwise, `()` is returned.
> 
> #### No Function Parameter
> 
> Array element (mutable) is bound to `this`.
> 
> This method is marked _pure_; the `mapper` function should not mutate array elements.
> 
> #### Function Parameters
> 
> * `element`: copy of array element
> * `index` _(optional)_: current index in the array
> 
> #### Example
> 
> ```rhai
> let x = [#{alice: 1}, #{bob: 2}, #{clara: 3}];
> 
> print(x.find_map(|v| v.alice));                  // prints 1
> 
> print(x.find_map(|v| v.dave) ?? "not found");    // prints "not found"
> 
> print(x.find_map(|| this.dave) ?? "not found");  // prints "not found"
> ```

---
## float.ceiling

```rust,ignore
get$ceiling(x: f64) -> f64
```

> Return the smallest whole number larger than or equals to the floating-point number.

---
## float.floor

```rust,ignore
get$floor(x: f64) -> f64
```

> Return the largest whole number less than or equals to the floating-point number.

---
## float.fraction

```rust,ignore
get$fraction(x: f64) -> f64
```

> Return the fractional part of the floating-point number.

---
## float.int

```rust,ignore
get$int(x: f64) -> f64
```

> Return the integral part of the floating-point number.

---
## float.is_finite

```rust,ignore
get$is_finite(x: f64) -> bool
```

> Return `true` if the floating-point number is finite.

---
## float.is_infinite

```rust,ignore
get$is_infinite(x: f64) -> bool
```

> Return `true` if the floating-point number is infinite.

---
## float.is_nan

```rust,ignore
get$is_nan(x: f64) -> bool
```

> Return `true` if the floating-point number is `NaN` (Not A Number).

---
## float.is_zero

```rust,ignore
get$is_zero(x: f64) -> bool
```

> Return true if the floating-point number is zero.

---
## float.round

```rust,ignore
get$round(x: f64) -> f64
```

> Return the nearest whole number closest to the floating-point number.
> Rounds away from zero.

---
## floor

```rust,ignore
floor(x: f64) -> f64
```

> Return the largest whole number less than or equals to the floating-point number.

---
## for_each

```rust,ignore
for_each(array: &mut Array, map: FnPtr) -> Result<(), Box<EvalAltResult>>
```

> Iterate through all the elements in the array, applying a `process` function to each element in turn.
> Each element is bound to `this` before calling the function.
> 
> #### Function Parameters
> 
> * `this`: bound to array element (mutable)
> * `index` _(optional)_: current index in the array
> 
> #### Example
> 
> ```rhai
> let x = [1, 2, 3, 4, 5];
> 
> x.for_each(|| this *= this);
> 
> print(x);       // prints "[1, 4, 9, 16, 25]"
> 
> x.for_each(|i| this *= i);
> 
> print(x);       // prints "[0, 2, 6, 12, 20]"
> ```

---
## fraction

```rust,ignore
fraction(x: f64) -> f64
```

> Return the fractional part of the floating-point number.

---
## get

```rust,ignore
get(map: &mut Map, property: &str) -> Dynamic
get(blob: &mut Blob, index: i64) -> i64
get(string: &str, index: i64) -> Dynamic
get(array: &mut Array, index: i64) -> Dynamic
```

> Get the value of the `property` in the object map and return a copy.
> 
> If `property` does not exist in the object map, `()` is returned.
> 
> #### Example
> 
> ```rhai
> let m = #{a: 1, b: 2, c: 3};
> 
> print(m.get("b"));      // prints 2
> 
> print(m.get("x"));      // prints empty (for '()')
> ```

---
## get_bit

```rust,ignore
get_bit(value: i64, bit: i64) -> Result<bool, Box<EvalAltResult>>
```

> Return `true` if the specified `bit` in the number is set.
> 
> If `bit` < 0, position counts from the MSB (Most Significant Bit).
> 
> #### Example
> 
> ```rhai
> let x = 123456;
> 
> print(x.get_bit(5));    // prints false
> 
> print(x.get_bit(6));    // prints true
> 
> print(x.get_bit(-48));  // prints true on 64-bit
> ```

---
## get_bits

```rust,ignore
get_bits(value: i64, range: RangeInclusive<i64>) -> Result<i64, Box<EvalAltResult>>
get_bits(value: i64, range: Range<i64>) -> Result<i64, Box<EvalAltResult>>
get_bits(value: i64, start: i64, bits: i64) -> Result<i64, Box<EvalAltResult>>
```

> Return an inclusive range of bits in the number as a new number.
> 
> #### Example
> 
> ```rhai
> let x = 123456;
> 
> print(x.get_bits(5..=9));       // print 18
> ```

---
## get_fn_metadata_list

```rust,ignore
get_fn_metadata_list() -> Array
get_fn_metadata_list(name: &str) -> Array
get_fn_metadata_list(name: &str, params: i64) -> Array
```

> Return an array of object maps containing metadata of all script-defined functions.

---
## hypot

```rust,ignore
hypot(x: f64, y: f64) -> f64
```

> Return the hypotenuse of a triangle with sides `x` and `y`.

---
## index_of

```rust,ignore
index_of(array: &mut Array, value: Dynamic) -> Result<i64, Box<EvalAltResult>>
index_of(array: &mut Array, filter: FnPtr) -> Result<i64, Box<EvalAltResult>>
index_of(array: &mut Array, filter: &str) -> Result<i64, Box<EvalAltResult>>
index_of(string: &str, find_string: &str) -> i64
index_of(string: &str, character: char) -> i64
index_of(array: &mut Array, filter: &str, start: i64) -> Result<i64, Box<EvalAltResult>>
index_of(array: &mut Array, filter: FnPtr, start: i64) -> Result<i64, Box<EvalAltResult>>
index_of(string: &str, find_string: &str, start: i64) -> i64
index_of(string: &str, character: char, start: i64) -> i64
index_of(array: &mut Array, value: Dynamic, start: i64) -> Result<i64, Box<EvalAltResult>>
```

> Find the first element in the array that equals a particular `value` and return its index.
> If no element equals `value`, `-1` is returned.
> 
> The operator `==` is used to compare elements with `value` and must be defined,
> otherwise `false` is assumed.
> 
> #### Example
> 
> ```rhai
> let x = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 5];
> 
> print(x.index_of(4));       // prints 3 (first index)
> 
> print(x.index_of(9));       // prints -1
> 
> print(x.index_of("foo"));   // prints -1: strings do not equal numbers
> ```

---
## insert

```rust,ignore
insert(blob: &mut Blob, index: i64, value: i64)
insert(array: &mut Array, index: i64, item: Dynamic)
```

> Add a byte `value` to the BLOB at a particular `index` position.
> 
> * If `index` < 0, position counts from the end of the BLOB (`-1` is the last byte).
> * If `index` < -length of BLOB, the byte value is added to the beginning of the BLOB.
> * If `index` ≥ length of BLOB, the byte value is appended to the end of the BLOB.
> 
> Only the lower 8 bits of the `value` are used; all other bits are ignored.
> 
> #### Example
> 
> ```rhai
> let b = blob(5, 0x42);
> 
> b.insert(2, 0x18);
> 
> print(b);       // prints "[4242184242]"
> ```

---
## int

```rust,ignore
int(x: f64) -> f64
```

> Return the integral part of the floating-point number.

---
## int.bits

```rust,ignore
get$bits(value: i64) -> Result<BitRange, Box<EvalAltResult>>
```

> Return an iterator over all the bits in the number.
> 
> #### Example
> 
> ```rhai
> let x = 123456;
> 
> for bit in x.bits {
> print(bit);
> }
> ```

---
## int.is_even

```rust,ignore
get$is_even(x: i64) -> bool
```

> Return true if the number is even.

---
## int.is_odd

```rust,ignore
get$is_odd(x: i64) -> bool
```

> Return true if the number is odd.

---
## int.is_zero

```rust,ignore
get$is_zero(x: i64) -> bool
```

> Return true if the number is zero.

---
## is_anonymous

```rust,ignore
is_anonymous(fn_ptr: &mut FnPtr) -> bool
```

> Return `true` if the function is an anonymous function.
> 
> #### Example
> 
> ```rhai
> let f = |x| x * 2;
> 
> print(f.is_anonymous);      // prints true
> ```

---
## is_empty

```rust,ignore
is_empty(string: &str) -> bool
is_empty(blob: &mut Blob) -> bool
is_empty(array: &mut Array) -> bool
is_empty(map: &mut Map) -> bool
is_empty(range: &mut Range<i64>) -> bool
is_empty(range: &mut RangeInclusive<i64>) -> bool
```

> Return true if the string is empty.

---
## is_even

```rust,ignore
is_even(x: i128) -> bool
is_even(x: u32) -> bool
is_even(x: u16) -> bool
is_even(x: i64) -> bool
is_even(x: u64) -> bool
is_even(x: u128) -> bool
is_even(x: i32) -> bool
is_even(x: i16) -> bool
is_even(x: u8) -> bool
is_even(x: i8) -> bool
```

> Return true if the number is even.

---
## is_exclusive

```rust,ignore
is_exclusive(range: &mut RangeInclusive<i64>) -> bool
is_exclusive(range: &mut Range<i64>) -> bool
```

> Return `true` if the range is exclusive.

---
## is_finite

```rust,ignore
is_finite(x: f64) -> bool
```

> Return `true` if the floating-point number is finite.

---
## is_inclusive

```rust,ignore
is_inclusive(range: &mut RangeInclusive<i64>) -> bool
is_inclusive(range: &mut Range<i64>) -> bool
```

> Return `true` if the range is inclusive.

---
## is_infinite

```rust,ignore
is_infinite(x: f64) -> bool
```

> Return `true` if the floating-point number is infinite.

---
## is_nan

```rust,ignore
is_nan(x: f64) -> bool
```

> Return `true` if the floating-point number is `NaN` (Not A Number).

---
## is_odd

```rust,ignore
is_odd(x: i128) -> bool
is_odd(x: u32) -> bool
is_odd(x: u16) -> bool
is_odd(x: i64) -> bool
is_odd(x: u64) -> bool
is_odd(x: u128) -> bool
is_odd(x: i32) -> bool
is_odd(x: i16) -> bool
is_odd(x: u8) -> bool
is_odd(x: i8) -> bool
```

> Return true if the number is odd.

---
## is_zero

```rust,ignore
is_zero(x: u8) -> bool
is_zero(x: i8) -> bool
is_zero(x: i16) -> bool
is_zero(x: i32) -> bool
is_zero(x: u64) -> bool
is_zero(x: u128) -> bool
is_zero(x: f64) -> bool
is_zero(x: u16) -> bool
is_zero(x: i64) -> bool
is_zero(x: f32) -> bool
is_zero(x: i128) -> bool
is_zero(x: u32) -> bool
```

> Return true if the number is zero.

---
## keys

```rust,ignore
keys(map: &mut Map) -> Array
```

> Return an array with all the property names in the object map.
> 
> #### Example
> 
> ```rhai
> let m = #{a:1, b:2, c:3};
> 
> print(m.keys());        // prints ["a", "b", "c"]
> ```

---
## len

```rust,ignore
len(map: &mut Map) -> i64
len(string: &str) -> i64
len(array: &mut Array) -> i64
len(blob: &mut Blob) -> i64
```

> Return the number of properties in the object map.

---
## ln

```rust,ignore
ln(x: f64) -> f64
```

> Return the natural log of the floating-point number.

---
## log

```rust,ignore
log(x: f64) -> f64
log(x: f64, base: f64) -> f64
```

> Return the log of the floating-point number with base 10.

---
## make_lower

```rust,ignore
make_lower(character: &mut char)
make_lower(string: &mut ImmutableString)
```

> Convert the character to lower-case.
> 
> #### Example
> 
> ```rhai
> let ch = 'A';
> 
> ch.make_lower();
> 
> print(ch);          // prints 'a'
> ```

---
## make_upper

```rust,ignore
make_upper(character: &mut char)
make_upper(string: &mut ImmutableString)
```

> Convert the character to upper-case.
> 
> #### Example
> 
> ```rhai
> let ch = 'a';
> 
> ch.make_upper();
> 
> print(ch);          // prints 'A'
> ```

---
## map

```rust,ignore
map(array: &mut Array, map: FnPtr) -> Result<Array, Box<EvalAltResult>>
map(array: &mut Array, mapper: &str) -> Result<Array, Box<EvalAltResult>>
```

> Iterate through all the elements in the array, applying a `mapper` function to each element
> in turn, and return the results as a new array.
> 
> #### No Function Parameter
> 
> Array element (mutable) is bound to `this`.
> 
> This method is marked _pure_; the `mapper` function should not mutate array elements.
> 
> #### Function Parameters
> 
> * `element`: copy of array element
> * `index` _(optional)_: current index in the array
> 
> #### Example
> 
> ```rhai
> let x = [1, 2, 3, 4, 5];
> 
> let y = x.map(|v| v * v);
> 
> print(y);       // prints "[1, 4, 9, 16, 25]"
> 
> let y = x.map(|v, i| v * i);
> 
> print(y);       // prints "[0, 2, 6, 12, 20]"
> ```

---
## max

```rust,ignore
max(string1: ImmutableString, string2: ImmutableString) -> ImmutableString
max(x: u64, y: u64) -> u64
max(x: f64, y: i64) -> f64
max(x: i64, y: i64) -> i64
max(x: f64, y: f32) -> f64
max(x: u32, y: u32) -> u32
max(x: i64, y: f32) -> f32
max(x: i64, y: f64) -> f64
max(x: f32, y: i64) -> f32
max(x: f32, y: f32) -> f32
max(x: f64, y: f64) -> f64
max(x: u16, y: u16) -> u16
max(x: i32, y: i32) -> i32
max(x: i8, y: i8) -> i8
max(x: i128, y: i128) -> i128
max(x: u128, y: u128) -> u128
max(x: f32, y: f64) -> f64
max(x: u8, y: u8) -> u8
max(x: i16, y: i16) -> i16
max(char1: char, char2: char) -> char
```

> Return the string that is lexically greater than the other string.
> 
> #### Example
> 
> ```rhai
> max("hello", "world");      // returns "world"
> ```

---
## min

```rust,ignore
min(x: i8, y: i8) -> i8
min(x: i32, y: i32) -> i32
min(x: u16, y: u16) -> u16
min(x: f64, y: f64) -> f64
min(char1: char, char2: char) -> char
min(x: i16, y: i16) -> i16
min(x: f32, y: f64) -> f64
min(x: u128, y: u128) -> u128
min(x: u8, y: u8) -> u8
min(x: i128, y: i128) -> i128
min(x: f64, y: f32) -> f64
min(x: f64, y: i64) -> f64
min(x: i64, y: i64) -> i64
min(string1: ImmutableString, string2: ImmutableString) -> ImmutableString
min(x: u64, y: u64) -> u64
min(x: f32, y: i64) -> f32
min(x: f32, y: f32) -> f32
min(x: i64, y: f64) -> f64
min(x: i64, y: f32) -> f32
min(x: u32, y: u32) -> u32
```

> Return the character that is lexically smaller than the other character.
> 
> #### Example
> 
> ```rhai
> max('h', 'w');      // returns 'h'
> ```

---
## mixin

```rust,ignore
mixin(map: &mut Map, map2: Map)
```

> Add all property values of another object map into the object map.
> Existing property values of the same names are replaced.
> 
> #### Example
> 
> ```rhai
> let m = #{a:1, b:2, c:3};
> let n = #{a: 42, d:0};
> 
> m.mixin(n);
> 
> print(m);       // prints "#{a:42, b:2, c:3, d:0}"
> ```

---
## name

```rust,ignore
name(fn_ptr: &mut FnPtr) -> ImmutableString
```

> Return the name of the function.
> 
> #### Example
> 
> ```rhai
> fn double(x) { x * 2 }
> 
> let f = Fn("double");
> 
> print(f.name);      // prints "double"
> ```

---
## pad

```rust,ignore
pad(blob: &mut Blob, len: i64, value: i64) -> Result<(), Box<EvalAltResult>>
pad(string: &mut ImmutableString, len: i64, padding: &str) -> Result<(), Box<EvalAltResult>>
pad(string: &mut ImmutableString, len: i64, character: char) -> Result<(), Box<EvalAltResult>>
pad(array: &mut Array, len: i64, item: Dynamic) -> Result<(), Box<EvalAltResult>>
```

> Pad the BLOB to at least the specified length with copies of a specified byte `value`.
> 
> If `len` ≤ length of BLOB, no padding is done.
> 
> Only the lower 8 bits of the `value` are used; all other bits are ignored.
> 
> #### Example
> 
> ```rhai
> let b = blob(3, 0x42);
> 
> b.pad(5, 0x18)
> 
> print(b);               // prints "[4242421818]"
> 
> b.pad(3, 0xab)
> 
> print(b);               // prints "[4242421818]"
> ```

---
## parse_be_float

```rust,ignore
parse_be_float(blob: &mut Blob, range: Range<i64>) -> f64
parse_be_float(blob: &mut Blob, range: RangeInclusive<i64>) -> f64
parse_be_float(blob: &mut Blob, start: i64, len: i64) -> f64
```

> Parse the bytes within an exclusive `range` in the BLOB as a `FLOAT`
> in big-endian byte order.
> 
> * If number of bytes in `range` < number of bytes for `FLOAT`, zeros are padded.
> * If number of bytes in `range` > number of bytes for `FLOAT`, extra bytes are ignored.

---
## parse_be_int

```rust,ignore
parse_be_int(blob: &mut Blob, range: RangeInclusive<i64>) -> i64
parse_be_int(blob: &mut Blob, range: Range<i64>) -> i64
parse_be_int(blob: &mut Blob, start: i64, len: i64) -> i64
```

> Parse the bytes within an inclusive `range` in the BLOB as an `INT`
> in big-endian byte order.
> 
> * If number of bytes in `range` < number of bytes for `INT`, zeros are padded.
> * If number of bytes in `range` > number of bytes for `INT`, extra bytes are ignored.
> 
> ```rhai
> let b = blob();
> 
> b += 1; b += 2; b += 3; b += 4; b += 5;
> 
> let x = b.parse_be_int(1..=3);  // parse three bytes
> 
> print(x.to_hex());              // prints "0203040000...00"
> ```

---
## parse_float

```rust,ignore
parse_float(string: &str) -> Result<f64, Box<EvalAltResult>>
```

> Parse a string into a floating-point number.
> 
> #### Example
> 
> ```rhai
> let x = parse_int("123.456");
> 
> print(x);       // prints 123.456
> ```

---
## parse_int

```rust,ignore
parse_int(string: &str) -> Result<i64, Box<EvalAltResult>>
parse_int(string: &str, radix: i64) -> Result<i64, Box<EvalAltResult>>
```

> Parse a string into an integer number.
> 
> #### Example
> 
> ```rhai
> let x = parse_int("123");
> 
> print(x);       // prints 123
> ```

---
## parse_json

```rust,ignore
parse_json(json: &str) -> Result<Dynamic, Box<EvalAltResult>>
```

> Parse a JSON string into a value.
> 
> #### Example
> 
> ```rhai
> let m = parse_json(`{"a":1, "b":2, "c":3}`);
> 
> print(m);       // prints #{"a":1, "b":2, "c":3}
> ```

---
## parse_le_float

```rust,ignore
parse_le_float(blob: &mut Blob, range: Range<i64>) -> f64
parse_le_float(blob: &mut Blob, range: RangeInclusive<i64>) -> f64
parse_le_float(blob: &mut Blob, start: i64, len: i64) -> f64
```

> Parse the bytes within an exclusive `range` in the BLOB as a `FLOAT`
> in little-endian byte order.
> 
> * If number of bytes in `range` < number of bytes for `FLOAT`, zeros are padded.
> * If number of bytes in `range` > number of bytes for `FLOAT`, extra bytes are ignored.

---
## parse_le_int

```rust,ignore
parse_le_int(blob: &mut Blob, range: Range<i64>) -> i64
parse_le_int(blob: &mut Blob, range: RangeInclusive<i64>) -> i64
parse_le_int(blob: &mut Blob, start: i64, len: i64) -> i64
```

> Parse the bytes within an exclusive `range` in the BLOB as an `INT`
> in little-endian byte order.
> 
> * If number of bytes in `range` < number of bytes for `INT`, zeros are padded.
> * If number of bytes in `range` > number of bytes for `INT`, extra bytes are ignored.
> 
> ```rhai
> let b = blob();
> 
> b += 1; b += 2; b += 3; b += 4; b += 5;
> 
> let x = b.parse_le_int(1..3);   // parse two bytes
> 
> print(x.to_hex());              // prints "0302"
> ```

---
## pop

```rust,ignore
pop(array: &mut Array) -> Dynamic
pop(blob: &mut Blob) -> i64
pop(string: &mut ImmutableString) -> Dynamic
pop(string: &mut ImmutableString, len: i64) -> ImmutableString
```

> Remove the last element from the array and return it.
> 
> If the array is empty, `()` is returned.
> 
> #### Example
> 
> ```rhai
> let x = [1, 2, 3];
> 
> print(x.pop());     // prints 3
> 
> print(x);           // prints "[1, 2]"
> ```

---
## print

```rust,ignore
print() -> ImmutableString
print(string: ImmutableString) -> ImmutableString
print(value: bool) -> ImmutableString
print(array: &mut Array) -> ImmutableString
print(character: char) -> ImmutableString
print(item: &mut Dynamic) -> ImmutableString
print(number: f64) -> ImmutableString
print(map: &mut Map) -> ImmutableString
print(number: f32) -> ImmutableString
print(unit: ()) -> ImmutableString
```

> Return the empty string.

---
## push

```rust,ignore
push(blob: &mut Blob, value: i64)
push(array: &mut Array, item: Dynamic)
```

> Add a new byte `value` to the end of the BLOB.
> 
> Only the lower 8 bits of the `value` are used; all other bits are ignored.
> 
> #### Example
> 
> ```rhai
> let b = blob();
> 
> b.push(0x42);
> 
> print(b);       // prints "[42]"
> ```

---
## range

```rust,ignore
range(range: std::ops::Range<i32>, step: i32) -> Result<StepRange<i32>, Box<EvalAltResult>>
range(range: std::ops::Range<i8>, step: i8) -> Result<StepRange<i8>, Box<EvalAltResult>>
range(from: u64, to: u64) -> Range<u64>
range(from: i64, to: i64) -> Range<i64>
range(range: std::ops::Range<FLOAT>, step: f64) -> Result<StepRange<FLOAT>, Box<EvalAltResult>>
range(range: std::ops::Range<u32>, step: u32) -> Result<StepRange<u32>, Box<EvalAltResult>>
range(range: std::ops::Range<u64>, step: u64) -> Result<StepRange<u64>, Box<EvalAltResult>>
range(from: u32, to: u32) -> Range<u32>
range(range: std::ops::Range<i128>, step: i128) -> Result<StepRange<i128>, Box<EvalAltResult>>
range(from: u16, to: u16) -> Range<u16>
range(from: i32, to: i32) -> Range<i32>
range(range: std::ops::Range<i64>, step: i64) -> Result<StepRange<i64>, Box<EvalAltResult>>
range(from: i8, to: i8) -> Range<i8>
range(range: std::ops::Range<u8>, step: u8) -> Result<StepRange<u8>, Box<EvalAltResult>>
range(range: std::ops::Range<u128>, step: u128) -> Result<StepRange<u128>, Box<EvalAltResult>>
range(from: i128, to: i128) -> Range<i128>
range(from: u8, to: u8) -> Range<u8>
range(range: std::ops::Range<u16>, step: u16) -> Result<StepRange<u16>, Box<EvalAltResult>>
range(from: u128, to: u128) -> Range<u128>
range(from: i16, to: i16) -> Range<i16>
range(range: std::ops::Range<i16>, step: i16) -> Result<StepRange<i16>, Box<EvalAltResult>>
range(from: f64, to: f64, step: f64) -> Result<StepRange<FLOAT>, Box<EvalAltResult>>
range(from: i128, to: i128, step: i128) -> Result<StepRange<i128>, Box<EvalAltResult>>
range(from: i8, to: i8, step: i8) -> Result<StepRange<i8>, Box<EvalAltResult>>
range(from: u8, to: u8, step: u8) -> Result<StepRange<u8>, Box<EvalAltResult>>
range(from: i16, to: i16, step: i16) -> Result<StepRange<i16>, Box<EvalAltResult>>
range(from: u64, to: u64, step: u64) -> Result<StepRange<u64>, Box<EvalAltResult>>
range(from: i32, to: i32, step: i32) -> Result<StepRange<i32>, Box<EvalAltResult>>
range(from: u128, to: u128, step: u128) -> Result<StepRange<u128>, Box<EvalAltResult>>
range(from: u32, to: u32, step: u32) -> Result<StepRange<u32>, Box<EvalAltResult>>
range(from: u16, to: u16, step: u16) -> Result<StepRange<u16>, Box<EvalAltResult>>
range(from: i64, to: i64, step: i64) -> Result<StepRange<i64>, Box<EvalAltResult>>
```

> Return an iterator over an exclusive range, each iteration increasing by `step`.
> 
> If `range` is reversed and `step` < 0, iteration goes backwards.
> 
> Otherwise, if `range` is empty, an empty iterator is returned.
> 
> #### Example
> 
> ```rhai
> // prints all values from 8 to 17 in steps of 3
> for n in range(8..18, 3) {
> print(n);
> }
> 
> // prints all values down from 18 to 9 in steps of -3
> for n in range(18..8, -3) {
> print(n);
> }
> ```

---
## reduce

```rust,ignore
reduce(array: &mut Array, reducer: &str) -> Result<Dynamic, Box<EvalAltResult>>
reduce(array: &mut Array, reducer: FnPtr) -> Result<Dynamic, Box<EvalAltResult>>
reduce(array: &mut Array, reducer: FnPtr, initial: Dynamic) -> Result<Dynamic, Box<EvalAltResult>>
reduce(array: &mut Array, reducer: &str, initial: Dynamic) -> Result<Dynamic, Box<EvalAltResult>>
```

> Reduce an array by iterating through all elements while applying a function named by `reducer`.
> 
> #### Deprecated API
> 
> This method is deprecated and will be removed from the next major version.
> Use `array.reduce(Fn("fn_name"))` instead.
> 
> #### Function Parameters
> 
> A function with the same name as the value of `reducer` must exist taking these parameters:
> 
> * `result`: accumulated result, initially `()`
> * `element`: copy of array element
> * `index` _(optional)_: current index in the array
> 
> #### Example
> 
> ```rhai
> fn process(r, x) {
> x + (r ?? 0)
> }
> fn process_extra(r, x, i) {
> x + i + (r ?? 0)
> }
> 
> let x = [1, 2, 3, 4, 5];
> 
> let y = x.reduce("process");
> 
> print(y);       // prints 15
> 
> let y = x.reduce("process_extra");
> 
> print(y);       // prints 25
> ```

---
## reduce_rev

```rust,ignore
reduce_rev(array: &mut Array, reducer: &str) -> Result<Dynamic, Box<EvalAltResult>>
reduce_rev(array: &mut Array, reducer: FnPtr) -> Result<Dynamic, Box<EvalAltResult>>
reduce_rev(array: &mut Array, reducer: FnPtr, initial: Dynamic) -> Result<Dynamic, Box<EvalAltResult>>
reduce_rev(array: &mut Array, reducer: &str, initial: Dynamic) -> Result<Dynamic, Box<EvalAltResult>>
```

> Reduce an array by iterating through all elements, in _reverse_ order,
> while applying a function named by `reducer`.
> 
> #### Deprecated API
> 
> This method is deprecated and will be removed from the next major version.
> Use `array.reduce_rev(Fn("fn_name"))` instead.
> 
> #### Function Parameters
> 
> A function with the same name as the value of `reducer` must exist taking these parameters:
> 
> * `result`: accumulated result, initially `()`
> * `element`: copy of array element
> * `index` _(optional)_: current index in the array
> 
> #### Example
> 
> ```rhai
> fn process(r, x) {
> x + (r ?? 0)
> }
> fn process_extra(r, x, i) {
> x + i + (r ?? 0)
> }
> 
> let x = [1, 2, 3, 4, 5];
> 
> let y = x.reduce_rev("process");
> 
> print(y);       // prints 15
> 
> let y = x.reduce_rev("process_extra");
> 
> print(y);       // prints 25
> ```

---
## remove

```rust,ignore
remove(array: &mut Array, index: i64) -> Dynamic
remove(string: &mut ImmutableString, sub_string: &str)
remove(string: &mut ImmutableString, character: char)
remove(blob: &mut Blob, index: i64) -> i64
remove(map: &mut Map, property: &str) -> Dynamic
```

> Remove the element at the specified `index` from the array and return it.
> 
> * If `index` < 0, position counts from the end of the array (`-1` is the last element).
> * If `index` < -length of array, `()` is returned.
> * If `index` ≥ length of array, `()` is returned.
> 
> #### Example
> 
> ```rhai
> let x = [1, 2, 3];
> 
> print(x.remove(1));     // prints 2
> 
> print(x);               // prints "[1, 3]"
> 
> print(x.remove(-2));    // prints 1
> 
> print(x);               // prints "[3]"
> ```

---
## replace

```rust,ignore
replace(string: &mut ImmutableString, find_string: &str, substitute_string: &str)
replace(string: &mut ImmutableString, find_character: char, substitute_character: char)
replace(string: &mut ImmutableString, find_string: &str, substitute_character: char)
replace(string: &mut ImmutableString, find_character: char, substitute_string: &str)
```

> Replace all occurrences of the specified sub-string in the string with another string.
> 
> #### Example
> 
> ```rhai
> let text = "hello, world! hello, foobar!";
> 
> text.replace("hello", "hey");
> 
> print(text);        // prints "hey, world! hey, foobar!"
> ```

---
## retain

```rust,ignore
retain(blob: &mut Blob, range: Range<i64>) -> Blob
retain(array: &mut Array, filter: &str) -> Result<Array, Box<EvalAltResult>>
retain(array: &mut Array, range: RangeInclusive<i64>) -> Array
retain(blob: &mut Blob, range: RangeInclusive<i64>) -> Blob
retain(array: &mut Array, filter: FnPtr) -> Result<Array, Box<EvalAltResult>>
retain(map: &mut Map, filter: FnPtr) -> Result<Map, Box<EvalAltResult>>
retain(array: &mut Array, range: Range<i64>) -> Array
retain(blob: &mut Blob, start: i64, len: i64) -> Blob
retain(array: &mut Array, start: i64, len: i64) -> Array
```

> Remove all bytes in the BLOB not within an exclusive `range` and return them as a new BLOB.
> 
> #### Example
> 
> ```rhai
> let b1 = blob();
> 
> b1 += 1; b1 += 2; b1 += 3; b1 += 4; b1 += 5;
> 
> let b2 = b1.retain(1..4);
> 
> print(b1);      // prints "[020304]"
> 
> print(b2);      // prints "[0105]"
> 
> let b3 = b1.retain(1..3);
> 
> print(b1);      // prints "[0304]"
> 
> print(b2);      // prints "[01]"
> ```

---
## reverse

```rust,ignore
reverse(array: &mut Array)
reverse(blob: &mut Blob)
```

> Reverse all the elements in the array.
> 
> #### Example
> 
> ```rhai
> let x = [1, 2, 3, 4, 5];
> 
> x.reverse();
> 
> print(x);       // prints "[5, 4, 3, 2, 1]"
> ```

---
## round

```rust,ignore
round(x: f64) -> f64
```

> Return the nearest whole number closest to the floating-point number.
> Rounds away from zero.

---
## set

```rust,ignore
set(blob: &mut Blob, index: i64, value: i64)
set(string: &mut ImmutableString, index: i64, character: char)
set(array: &mut Array, index: i64, value: Dynamic)
set(map: &mut Map, property: &str, value: Dynamic)
```

> Set the particular `index` position in the BLOB to a new byte `value`.
> 
> * If `index` < 0, position counts from the end of the BLOB (`-1` is the last byte).
> * If `index` < -length of BLOB, the BLOB is not modified.
> * If `index` ≥ length of BLOB, the BLOB is not modified.
> 
> #### Example
> 
> ```rhai
> let b = blob();
> 
> b += 1; b += 2; b += 3; b += 4; b += 5;
> 
> b.set(0, 0x42);
> 
> print(b);           // prints "[4202030405]"
> 
> b.set(-3, 0);
> 
> print(b);           // prints "[4202000405]"
> 
> b.set(99, 123);
> 
> print(b);           // prints "[4202000405]"
> ```

---
## set_bit

```rust,ignore
set_bit(value: &mut i64, bit: i64, new_value: bool) -> Result<(), Box<EvalAltResult>>
```

> Set the specified `bit` in the number if the new value is `true`.
> Clear the `bit` if the new value is `false`.
> 
> If `bit` < 0, position counts from the MSB (Most Significant Bit).
> 
> #### Example
> 
> ```rhai
> let x = 123456;
> 
> x.set_bit(5, true);
> 
> print(x);               // prints 123488
> 
> x.set_bit(6, false);
> 
> print(x);               // prints 123424
> 
> x.set_bit(-48, false);
> 
> print(x);               // prints 57888 on 64-bit
> ```

---
## set_bits

```rust,ignore
set_bits(value: &mut i64, range: Range<i64>, new_value: i64) -> Result<(), Box<EvalAltResult>>
set_bits(value: &mut i64, range: RangeInclusive<i64>, new_value: i64) -> Result<(), Box<EvalAltResult>>
set_bits(value: &mut i64, bit: i64, bits: i64, new_value: i64) -> Result<(), Box<EvalAltResult>>
```

> Replace an exclusive range of bits in the number with a new value.
> 
> #### Example
> 
> ```rhai
> let x = 123456;
> 
> x.set_bits(5..10, 42);
> 
> print(x);           // print 123200
> ```

---
## set_tag

```rust,ignore
set_tag(value: &mut Dynamic, tag: i64) -> Result<(), Box<EvalAltResult>>
```

> Set the _tag_ of a `Dynamic` value.
> 
> #### Example
> 
> ```rhai
> let x = "hello, world!";
> 
> x.tag = 42;
> 
> print(x.tag);           // prints 42
> ```

---
## shift

```rust,ignore
shift(blob: &mut Blob) -> i64
shift(array: &mut Array) -> Dynamic
```

> Remove the first byte from the BLOB and return it.
> 
> If the BLOB is empty, zero is returned.
> 
> #### Example
> 
> ```rhai
> let b = blob();
> 
> b += 1; b += 2; b += 3; b += 4; b += 5;
> 
> print(b.shift());       // prints 1
> 
> print(b);               // prints "[02030405]"
> ```

---
## sign

```rust,ignore
sign(x: i32) -> i64
sign(x: i16) -> i64
sign(x: i8) -> i64
sign(x: i128) -> i64
sign(x: f32) -> Result<i64, Box<EvalAltResult>>
sign(x: i64) -> i64
sign(x: f64) -> Result<i64, Box<EvalAltResult>>
```

> Return the sign (as an integer) of the number according to the following:
> 
> * `0` if the number is zero
> * `1` if the number is positive
> * `-1` if the number is negative

---
## sin

```rust,ignore
sin(x: f64) -> f64
```

> Return the sine of the floating-point number in radians.

---
## sinh

```rust,ignore
sinh(x: f64) -> f64
```

> Return the hyperbolic sine of the floating-point number in radians.

---
## sleep

```rust,ignore
sleep(seconds: i64)
sleep(seconds: f64)
```

> Block the current thread for a particular number of `seconds`.
> 
> #### Example
> 
> ```rhai
> // Do nothing for 10 seconds!
> sleep(10);
> ```

---
## some

```rust,ignore
some(array: &mut Array, filter: FnPtr) -> Result<bool, Box<EvalAltResult>>
some(array: &mut Array, filter: &str) -> Result<bool, Box<EvalAltResult>>
```

> Return `true` if any element in the array that returns `true` when applied the `filter` function.
> 
> #### No Function Parameter
> 
> Array element (mutable) is bound to `this`.
> 
> This method is marked _pure_; the `filter` function should not mutate array elements.
> 
> #### Function Parameters
> 
> * `element`: copy of array element
> * `index` _(optional)_: current index in the array
> 
> #### Example
> 
> ```rhai
> let x = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 5];
> 
> print(x.some(|v| v > 3));       // prints true
> 
> print(x.some(|v| v > 10));      // prints false
> 
> print(x.some(|v, i| i > v));    // prints true
> ```

---
## sort

```rust,ignore
sort(array: &mut Array) -> Result<(), Box<EvalAltResult>>
sort(array: &mut Array, comparer: FnPtr)
sort(array: &mut Array, comparer: &str) -> Result<(), Box<EvalAltResult>>
```

> Sort the array.
> 
> All elements in the array must be of the same data type.
> 
> #### Supported Data Types
> 
> * integer numbers
> * floating-point numbers
> * decimal numbers
> * characters
> * strings
> * booleans
> * `()`
> 
> #### Example
> 
> ```rhai
> let x = [1, 3, 5, 7, 9, 2, 4, 6, 8, 10];
> 
> x.sort();
> 
> print(x);       // prints "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]"
> ```

---
## splice

```rust,ignore
splice(array: &mut Array, range: Range<i64>, replace: Array)
splice(blob: &mut Blob, range: Range<i64>, replace: Blob)
splice(array: &mut Array, range: RangeInclusive<i64>, replace: Array)
splice(blob: &mut Blob, range: RangeInclusive<i64>, replace: Blob)
splice(array: &mut Array, start: i64, len: i64, replace: Array)
splice(blob: &mut Blob, start: i64, len: i64, replace: Blob)
```

> Replace an exclusive range of the array with another array.
> 
> #### Example
> 
> ```rhai
> let x = [1, 2, 3, 4, 5];
> let y = [7, 8, 9, 10];
> 
> x.splice(1..3, y);
> 
> print(x);       // prints "[1, 7, 8, 9, 10, 4, 5]"
> ```

---
## split

```rust,ignore
split(string: &str) -> Array
split(blob: &mut Blob, index: i64) -> Blob
split(string: &str, delimiter: char) -> Array
split(array: &mut Array, index: i64) -> Array
split(string: &str, delimiter: &str) -> Array
split(string: &mut ImmutableString, index: i64) -> Array
split(string: &str, delimiter: &str, segments: i64) -> Array
split(string: &str, delimiter: char, segments: i64) -> Array
```

> Split the string into segments based on whitespaces, returning an array of the segments.
> 
> #### Example
> 
> ```rhai
> let text = "hello, world! hello, foo!";
> 
> print(text.split());        // prints ["hello,", "world!", "hello,", "foo!"]
> ```

---
## split_rev

```rust,ignore
split_rev(string: &str, delimiter: &str) -> Array
split_rev(string: &str, delimiter: char) -> Array
split_rev(string: &str, delimiter: &str, segments: i64) -> Array
split_rev(string: &str, delimiter: char, segments: i64) -> Array
```

> Split the string into segments based on a `delimiter` string, returning an array of the
> segments in _reverse_ order.
> 
> #### Example
> 
> ```rhai
> let text = "hello, world! hello, foo!";
> 
> print(text.split_rev("ll"));    // prints ["o, foo!", "o, world! he", "he"]
> ```

---
## sqrt

```rust,ignore
sqrt(x: f64) -> f64
```

> Return the square root of the floating-point number.

---
## start

```rust,ignore
start(range: &mut Range<i64>) -> i64
start(range: &mut RangeInclusive<i64>) -> i64
```

> Return the start of the exclusive range.

---
## starts_with

```rust,ignore
starts_with(string: &str, match_string: &str) -> bool
```

> Return `true` if the string starts with a specified string.
> 
> #### Example
> 
> ```rhai
> let text = "hello, world!";
> 
> print(text.starts_with("hello"));   // prints true
> 
> print(text.starts_with("world"));   // prints false
> ```

---
## sub_string

```rust,ignore
sub_string(string: &str, range: Range<i64>) -> ImmutableString
sub_string(string: &str, range: RangeInclusive<i64>) -> ImmutableString
sub_string(string: &str, start: i64) -> ImmutableString
sub_string(string: &str, start: i64, len: i64) -> ImmutableString
```

> Copy an exclusive range of characters from the string and return it as a new string.
> 
> #### Example
> 
> ```rhai
> let text = "hello, world!";
> 
> print(text.sub_string(3..7));   // prints "lo, "
> ```

---
## tag

```rust,ignore
tag(value: &mut Dynamic) -> i64
```

> Return the _tag_ of a `Dynamic` value.
> 
> #### Example
> 
> ```rhai
> let x = "hello, world!";
> 
> x.tag = 42;
> 
> print(x.tag);           // prints 42
> ```

---
## take

```rust,ignore
take(value: &mut Dynamic) -> Result<Dynamic, Box<EvalAltResult>>
```

> Take ownership of the data in a `Dynamic` value and return it.
> The data is _NOT_ cloned.
> 
> The original value is replaced with `()`.
> 
> #### Example
> 
> ```rhai
> let x = 42;
> 
> print(take(x));         // prints 42
> 
> print(x);               // prints ()
> ```

---
## tan

```rust,ignore
tan(x: f64) -> f64
```

> Return the tangent of the floating-point number in radians.

---
## tanh

```rust,ignore
tanh(x: f64) -> f64
```

> Return the hyperbolic tangent of the floating-point number in radians.

---
## timestamp

```rust,ignore
timestamp() -> Instant
```

> Create a timestamp containing the current system time.
> 
> #### Example
> 
> ```rhai
> let now = timestamp();
> 
> sleep(10.0);            // sleep for 10 seconds
> 
> print(now.elapsed);     // prints 10.???
> ```

---
## to_array

```rust,ignore
to_array(blob: &mut Blob) -> Array
```

> Convert the BLOB into an array of integers.
> 
> #### Example
> 
> ```rhai
> let b = blob(5, 0x42);
> 
> let x = b.to_array();
> 
> print(x);       // prints "[66, 66, 66, 66, 66]"
> ```

---
## to_binary

```rust,ignore
to_binary(value: i32) -> ImmutableString
to_binary(value: u64) -> ImmutableString
to_binary(value: u128) -> ImmutableString
to_binary(value: u8) -> ImmutableString
to_binary(value: i8) -> ImmutableString
to_binary(value: i16) -> ImmutableString
to_binary(value: i128) -> ImmutableString
to_binary(value: u32) -> ImmutableString
to_binary(value: u16) -> ImmutableString
to_binary(value: i64) -> ImmutableString
```

> Convert the `value` into a string in binary format.

---
## to_blob

```rust,ignore
to_blob(string: &str) -> Blob
```

> Convert the string into an UTF-8 encoded byte-stream as a BLOB.
> 
> #### Example
> 
> ```rhai
> let text = "朝には紅顔ありて夕べには白骨となる";
> 
> let bytes = text.to_blob();
> 
> print(bytes.len());     // prints 51
> ```

---
## to_chars

```rust,ignore
to_chars(string: &str) -> Array
```

> Return an array containing all the characters of the string.
> 
> #### Example
> 
> ```rhai
> let text = "hello";
> 
> print(text.to_chars());     // prints "['h', 'e', 'l', 'l', 'o']"
> ```

---
## to_debug

```rust,ignore
to_debug(unit: ()) -> ImmutableString
to_debug(number: f32) -> ImmutableString
to_debug(map: &mut Map) -> ImmutableString
to_debug(number: f64) -> ImmutableString
to_debug(item: &mut Dynamic) -> ImmutableString
to_debug(character: char) -> ImmutableString
to_debug(array: &mut Array) -> ImmutableString
to_debug(value: bool) -> ImmutableString
to_debug(f: &mut FnPtr) -> ImmutableString
to_debug(string: &str) -> ImmutableString
```

> Convert the unit into a string in debug format.

---
## to_degrees

```rust,ignore
to_degrees(x: f64) -> f64
```

> Convert radians to degrees.

---
## to_float

```rust,ignore
to_float(_)
to_float(_)
to_float(_)
to_float(_)
to_float(_)
to_float(_)
to_float(_)
to_float(_)
to_float(_)
to_float(x: f32) -> f64
to_float(_)
to_float(_)
```

> Convert the 32-bit floating-point number to 64-bit.

---
## to_hex

```rust,ignore
to_hex(value: i16) -> ImmutableString
to_hex(value: i8) -> ImmutableString
to_hex(value: u8) -> ImmutableString
to_hex(value: u128) -> ImmutableString
to_hex(value: u64) -> ImmutableString
to_hex(value: i32) -> ImmutableString
to_hex(value: i64) -> ImmutableString
to_hex(value: u16) -> ImmutableString
to_hex(value: u32) -> ImmutableString
to_hex(value: i128) -> ImmutableString
```

> Convert the `value` into a string in hex format.

---
## to_int

```rust,ignore
to_int(_)
to_int(_)
to_int(_)
to_int(_)
to_int(_)
to_int(_)
to_int(_)
to_int(_)
to_int(_)
to_int(x: f32) -> Result<i64, Box<EvalAltResult>>
to_int(_)
to_int(_)
to_int(x: f64) -> Result<i64, Box<EvalAltResult>>
```

> Convert the floating-point number into an integer.

---
## to_json

```rust,ignore
to_json(map: &mut Map) -> String
```

> Return the JSON representation of the object map.
> 
> #### Data types
> 
> Only the following data types should be kept inside the object map:
> `INT`, `FLOAT`, `ImmutableString`, `char`, `bool`, `()`, `Array`, `Map`.
> 
> #### Errors
> 
> Data types not supported by JSON serialize into formats that may
> invalidate the result.
> 
> #### Example
> 
> ```rhai
> let m = #{a:1, b:2, c:3};
> 
> print(m.to_json());     // prints {"a":1, "b":2, "c":3}
> ```

---
## to_lower

```rust,ignore
to_lower(character: char) -> char
to_lower(string: ImmutableString) -> ImmutableString
```

> Convert the character to lower-case and return it as a new character.
> 
> #### Example
> 
> ```rhai
> let ch = 'A';
> 
> print(ch.to_lower());       // prints 'a'
> 
> print(ch);                  // prints 'A'
> ```

---
## to_octal

```rust,ignore
to_octal(value: u16) -> ImmutableString
to_octal(value: i64) -> ImmutableString
to_octal(value: i128) -> ImmutableString
to_octal(value: u32) -> ImmutableString
to_octal(value: i16) -> ImmutableString
to_octal(value: u8) -> ImmutableString
to_octal(value: i8) -> ImmutableString
to_octal(value: u64) -> ImmutableString
to_octal(value: u128) -> ImmutableString
to_octal(value: i32) -> ImmutableString
```

> Convert the `value` into a string in octal format.

---
## to_radians

```rust,ignore
to_radians(x: f64) -> f64
```

> Convert degrees to radians.

---
## to_string

```rust,ignore
to_string(map: &mut Map) -> ImmutableString
to_string(number: f64) -> ImmutableString
to_string(unit: ()) -> ImmutableString
to_string(number: f32) -> ImmutableString
to_string(value: bool) -> ImmutableString
to_string(string: ImmutableString) -> ImmutableString
to_string(item: &mut Dynamic) -> ImmutableString
to_string(character: char) -> ImmutableString
to_string(array: &mut Array) -> ImmutableString
```

> Convert the object map into a string.

---
## to_upper

```rust,ignore
to_upper(string: ImmutableString) -> ImmutableString
to_upper(character: char) -> char
```

> Convert the string to all upper-case and return it as a new string.
> 
> #### Example
> 
> ```rhai
> let text = "hello, world!"
> 
> print(text.to_upper());     // prints "HELLO, WORLD!"
> 
> print(text);                // prints "hello, world!"
> ```

---
## trim

```rust,ignore
trim(string: &mut ImmutableString)
```

> Remove whitespace characters from both ends of the string.
> 
> #### Example
> 
> ```rhai
> let text = "   hello     ";
> 
> text.trim();
> 
> print(text);    // prints "hello"
> ```

---
## truncate

```rust,ignore
truncate(blob: &mut Blob, len: i64)
truncate(array: &mut Array, len: i64)
truncate(string: &mut ImmutableString, len: i64)
```

> Cut off the BLOB at the specified length.
> 
> * If `len` ≤ 0, the BLOB is cleared.
> * If `len` ≥ length of BLOB, the BLOB is not truncated.
> 
> #### Example
> 
> ```rhai
> let b = blob();
> 
> b += 1; b += 2; b += 3; b += 4; b += 5;
> 
> b.truncate(3);
> 
> print(b);           // prints "[010203]"
> 
> b.truncate(10);
> 
> print(b);           // prints "[010203]"
> ```

---
## values

```rust,ignore
values(map: &mut Map) -> Array
```

> Return an array with all the property values in the object map.
> 
> #### Example
> 
> ```rhai
> let m = #{a:1, b:2, c:3};
> 
> print(m.values());      // prints "[1, 2, 3]""
> ```

---
## write_ascii

```rust,ignore
write_ascii(blob: &mut Blob, range: Range<i64>, string: &str)
write_ascii(blob: &mut Blob, range: RangeInclusive<i64>, string: &str)
write_ascii(blob: &mut Blob, start: i64, len: i64, string: &str)
```

> Write an ASCII string to the bytes within an exclusive `range` in the BLOB.
> 
> Each ASCII character encodes to one single byte in the BLOB.
> Non-ASCII characters are ignored.
> 
> * If number of bytes in `range` < length of `string`, extra bytes in `string` are not written.
> * If number of bytes in `range` > length of `string`, extra bytes in `range` are not modified.
> 
> ```rhai
> let b = blob(8);
> 
> b.write_ascii(1..5, "hello, world!");
> 
> print(b);       // prints "[0068656c6c000000]"
> ```

---
## write_be

```rust,ignore
write_be(blob: &mut Blob, range: Range<i64>, value: i64)
write_be(blob: &mut Blob, range: Range<i64>, value: f64)
write_be(blob: &mut Blob, range: RangeInclusive<i64>, value: i64)
write_be(blob: &mut Blob, range: RangeInclusive<i64>, value: f64)
write_be(blob: &mut Blob, start: i64, len: i64, value: f64)
write_be(blob: &mut Blob, start: i64, len: i64, value: i64)
```

> Write an `INT` value to the bytes within an exclusive `range` in the BLOB
> in big-endian byte order.
> 
> * If number of bytes in `range` < number of bytes for `INT`, extra bytes in `INT` are not written.
> * If number of bytes in `range` > number of bytes for `INT`, extra bytes in `range` are not modified.
> 
> ```rhai
> let b = blob(8, 0x42);
> 
> b.write_be_int(1..3, 0x99);
> 
> print(b);       // prints "[4200004242424242]"
> ```

---
## write_le

```rust,ignore
write_le(blob: &mut Blob, range: Range<i64>, value: f64)
write_le(blob: &mut Blob, range: Range<i64>, value: i64)
write_le(blob: &mut Blob, range: RangeInclusive<i64>, value: f64)
write_le(blob: &mut Blob, range: RangeInclusive<i64>, value: i64)
write_le(blob: &mut Blob, start: i64, len: i64, value: i64)
write_le(blob: &mut Blob, start: i64, len: i64, value: f64)
```

> Write a `FLOAT` value to the bytes within an exclusive `range` in the BLOB
> in little-endian byte order.
> 
> * If number of bytes in `range` < number of bytes for `FLOAT`, extra bytes in `FLOAT` are not written.
> * If number of bytes in `range` > number of bytes for `FLOAT`, extra bytes in `range` are not modified.

---
## write_utf8

```rust,ignore
write_utf8(blob: &mut Blob, range: Range<i64>, string: &str)
write_utf8(blob: &mut Blob, range: RangeInclusive<i64>, string: &str)
write_utf8(blob: &mut Blob, start: i64, len: i64, string: &str)
```

> Write a string to the bytes within an exclusive `range` in the BLOB in UTF-8 encoding.
> 
> * If number of bytes in `range` < length of `string`, extra bytes in `string` are not written.
> * If number of bytes in `range` > length of `string`, extra bytes in `range` are not modified.
> 
> ```rhai
> let b = blob(8);
> 
> b.write_utf8(1..5, "朝には紅顔ありて夕べには白骨となる");
> 
> print(b);       // prints "[00e69c9de3000000]"
> ```

---
## zip

```rust,ignore
zip(array1: &mut Array, array2: Array, map: FnPtr) -> Result<Array, Box<EvalAltResult>>
```

> Iterate through all elements in two arrays, applying a `mapper` function to them,
> and return a new array containing the results.
> 
> #### Function Parameters
> 
> * `array1`: First array
> * `array2`: Second array
> * `index` _(optional)_: current index in the array
> 
> #### Example
> 
> ```rhai
> let x = [1, 2, 3, 4, 5];
> let y = [9, 8, 7, 6];
> 
> let z = x.zip(y, |a, b| a + b);
> 
> print(z);       // prints [10, 10, 10, 10]
> 
> let z = x.zip(y, |a, b, i| a + b + i);
> 
> print(z);       // prints [10, 11, 12, 13]
> ```

---
