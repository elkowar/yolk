<div class='rhai-doc'>

# Rhai Standard Library builtins
Rhai standard library functions.

Note that the typesignatures here do look a bit weird.
This is simply a result of how we generate the documentation,
and can't easily be improved.

Just try your best to ignore it...

---

**namespace**: `global`

---



<div class='doc-block'>

## Array.is_empty

<div class='doc-content'>

```rust,ignore
get$is_empty(array: &mut Array) -> bool
```

Return true if the array is empty.

</div>
</div>




<div class='doc-block'>

## Array.len

<div class='doc-content'>

```rust,ignore
get$len(array: &mut Array) -> i64
```

Number of elements in the array.

</div>
</div>




<div class='doc-block'>

## Blob.is_empty

<div class='doc-content'>

```rust,ignore
get$is_empty(blob: &mut Blob) -> bool
```

Return true if the BLOB is empty.

</div>
</div>




<div class='doc-block'>

## Blob.len

<div class='doc-content'>

```rust,ignore
get$len(blob: &mut Blob) -> i64
```

Return the length of the BLOB.

#### Example

```rhai
let b = blob(10, 0x42);

print(b);           // prints "[4242424242424242 4242]"

print(b.len());     // prints 10
```

</div>
</div>




<div class='doc-block'>

## E

<div class='doc-content'>

```rust,ignore
E() -> f64
```

Return the natural number _e_.

</div>
</div>




<div class='doc-block'>

## Instant.elapsed

<div class='doc-content'>

```rust,ignore
get$elapsed(timestamp: Instant) -> Result<Dynamic, Box<EvalAltResult>>
```

Return the number of seconds between the current system time and the timestamp.

#### Example

```rhai
let now = timestamp();

sleep(10.0);            // sleep for 10 seconds

print(now.elapsed);     // prints 10.???
```

</div>
</div>




<div class='doc-block'>

## PI

<div class='doc-content'>

```rust,ignore
PI() -> f64
```

Return the number π.

</div>
</div>




<div class='doc-block'>

## Range<int>.end

<div class='doc-content'>

```rust,ignore
get$end(range: &mut Range<i64>) -> i64
```

Return the end of the exclusive range.

</div>
</div>




<div class='doc-block'>

## Range<int>.is_empty

<div class='doc-content'>

```rust,ignore
get$is_empty(range: &mut Range<i64>) -> bool
```

Return true if the range contains no items.

</div>
</div>




<div class='doc-block'>

## Range<int>.is_exclusive

<div class='doc-content'>

```rust,ignore
get$is_exclusive(range: &mut Range<i64>) -> bool
```

Return `true` if the range is exclusive.

</div>
</div>




<div class='doc-block'>

## Range<int>.is_inclusive

<div class='doc-content'>

```rust,ignore
get$is_inclusive(range: &mut Range<i64>) -> bool
```

Return `true` if the range is inclusive.

</div>
</div>




<div class='doc-block'>

## Range<int>.start

<div class='doc-content'>

```rust,ignore
get$start(range: &mut Range<i64>) -> i64
```

Return the start of the exclusive range.

</div>
</div>




<div class='doc-block'>

## RangeInclusive<int>.end

<div class='doc-content'>

```rust,ignore
get$end(range: &mut RangeInclusive<i64>) -> i64
```

Return the end of the inclusive range.

</div>
</div>




<div class='doc-block'>

## RangeInclusive<int>.is_empty

<div class='doc-content'>

```rust,ignore
get$is_empty(range: &mut RangeInclusive<i64>) -> bool
```

Return true if the range contains no items.

</div>
</div>




<div class='doc-block'>

## RangeInclusive<int>.is_exclusive

<div class='doc-content'>

```rust,ignore
get$is_exclusive(range: &mut RangeInclusive<i64>) -> bool
```

Return `true` if the range is exclusive.

</div>
</div>




<div class='doc-block'>

## RangeInclusive<int>.is_inclusive

<div class='doc-content'>

```rust,ignore
get$is_inclusive(range: &mut RangeInclusive<i64>) -> bool
```

Return `true` if the range is inclusive.

</div>
</div>




<div class='doc-block'>

## RangeInclusive<int>.start

<div class='doc-content'>

```rust,ignore
get$start(range: &mut RangeInclusive<i64>) -> i64
```

Return the start of the inclusive range.

</div>
</div>




<div class='doc-block'>

## String.bytes

<div class='doc-content'>

```rust,ignore
get$bytes(string: &str) -> i64
```

Return the length of the string, in number of bytes used to store it in UTF-8 encoding.

#### Example

```rhai
let text = "朝には紅顔ありて夕べには白骨となる";

print(text.bytes);      // prints 51
```

</div>
</div>




<div class='doc-block'>

## String.chars

<div class='doc-content'>

```rust,ignore
get$chars(string: &str) -> CharsStream
```

Return an iterator over all the characters in the string.

#### Example

```rhai
for ch in "hello, world!".chars {"
print(ch);
}
```

</div>
</div>




<div class='doc-block'>

## String.is_empty

<div class='doc-content'>

```rust,ignore
get$is_empty(string: &str) -> bool
```

Return true if the string is empty.

</div>
</div>




<div class='doc-block'>

## String.len

<div class='doc-content'>

```rust,ignore
get$len(string: &str) -> i64
```

Return the length of the string, in number of characters.

#### Example

```rhai
let text = "朝には紅顔ありて夕べには白骨となる";

print(text.len);        // prints 17
```

</div>
</div>




<div class='doc-block'>

## abs

<div class='doc-content'>

```rust,ignore
abs(x: i32) -> Result<i32, Box<EvalAltResult>>
abs(x: i128) -> Result<i128, Box<EvalAltResult>>
abs(x: i16) -> Result<i16, Box<EvalAltResult>>
abs(x: i8) -> Result<i8, Box<EvalAltResult>>
abs(x: f32) -> f32
abs(x: f64) -> f64
abs(x: i64) -> Result<i64, Box<EvalAltResult>>
```

Return the absolute value of the number.

</div>
</div>




<div class='doc-block'>

## acos

<div class='doc-content'>

```rust,ignore
acos(x: f64) -> f64
```

Return the arc-cosine of the floating-point number, in radians.

</div>
</div>




<div class='doc-block'>

## acosh

<div class='doc-content'>

```rust,ignore
acosh(x: f64) -> f64
```

Return the arc-hyperbolic-cosine of the floating-point number, in radians.

</div>
</div>




<div class='doc-block'>

## all

<div class='doc-content'>

```rust,ignore
all(array: &mut Array, filter: FnPtr) -> Result<bool, Box<EvalAltResult>>
all(array: &mut Array, filter: &str) -> Result<bool, Box<EvalAltResult>>
```

Return `true` if all elements in the array return `true` when applied the `filter` function.

#### No Function Parameter

Array element (mutable) is bound to `this`.

This method is marked _pure_; the `filter` function should not mutate array elements.

#### Function Parameters

* `element`: copy of array element
* `index` _(optional)_: current index in the array

#### Example

```rhai
let x = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 5];

print(x.all(|v| v > 3));        // prints false

print(x.all(|v| v > 1));        // prints true

print(x.all(|v, i| i > v));     // prints false
```

</div>
</div>




<div class='doc-block'>

## append

<div class='doc-content'>

```rust,ignore
append(array: &mut Array, new_array: Array)
append(blob: &mut Blob, character: char)
append(string: &mut ImmutableString, mut item: Dynamic)
append(string: &mut ImmutableString, utf8: Blob)
append(blob1: &mut Blob, blob2: Blob)
append(blob: &mut Blob, string: &str)
append(blob: &mut Blob, value: i64)
```

Add all the elements of another array to the end of the array.

#### Example

```rhai
let x = [1, 2, 3];
let y = [true, 'x'];

x.append(y);

print(x);       // prints "[1, 2, 3, true, 'x']"
```

</div>
</div>




<div class='doc-block'>

## as_string

<div class='doc-content'>

```rust,ignore
as_string(blob: Blob) -> String
```

Convert the BLOB into a string.

The byte stream must be valid UTF-8, otherwise an error is raised.

#### Example

```rhai
let b = blob(5, 0x42);

let x = b.as_string();

print(x);       // prints "FFFFF"
```

</div>
</div>




<div class='doc-block'>

## asin

<div class='doc-content'>

```rust,ignore
asin(x: f64) -> f64
```

Return the arc-sine of the floating-point number, in radians.

</div>
</div>




<div class='doc-block'>

## asinh

<div class='doc-content'>

```rust,ignore
asinh(x: f64) -> f64
```

Return the arc-hyperbolic-sine of the floating-point number, in radians.

</div>
</div>




<div class='doc-block'>

## atan

<div class='doc-content'>

```rust,ignore
atan(x: f64) -> f64
atan(x: f64, y: f64) -> f64
```

Return the arc-tangent of the floating-point number, in radians.

</div>
</div>




<div class='doc-block'>

## atanh

<div class='doc-content'>

```rust,ignore
atanh(x: f64) -> f64
```

Return the arc-hyperbolic-tangent of the floating-point number, in radians.

</div>
</div>




<div class='doc-block'>

## bits

<div class='doc-content'>

```rust,ignore
bits(value: i64) -> Result<BitRange, Box<EvalAltResult>>
bits(value: i64, range: RangeInclusive<i64>) -> Result<BitRange, Box<EvalAltResult>>
bits(value: i64, from: i64) -> Result<BitRange, Box<EvalAltResult>>
bits(value: i64, range: Range<i64>) -> Result<BitRange, Box<EvalAltResult>>
bits(value: i64, from: i64, len: i64) -> Result<BitRange, Box<EvalAltResult>>
```

Return an iterator over all the bits in the number.

#### Example

```rhai
let x = 123456;

for bit in x.bits() {
print(bit);
}
```

</div>
</div>




<div class='doc-block'>

## blob

<div class='doc-content'>

```rust,ignore
blob() -> Blob
blob(len: i64) -> Result<Blob, Box<EvalAltResult>>
blob(len: i64, value: i64) -> Result<Blob, Box<EvalAltResult>>
```

Return a new, empty BLOB.

</div>
</div>




<div class='doc-block'>

## bytes

<div class='doc-content'>

```rust,ignore
bytes(string: &str) -> i64
```

Return the length of the string, in number of bytes used to store it in UTF-8 encoding.

#### Example

```rhai
let text = "朝には紅顔ありて夕べには白骨となる";

print(text.bytes);      // prints 51
```

</div>
</div>




<div class='doc-block'>

## ceiling

<div class='doc-content'>

```rust,ignore
ceiling(x: f64) -> f64
```

Return the smallest whole number larger than or equals to the floating-point number.

</div>
</div>




<div class='doc-block'>

## chars

<div class='doc-content'>

```rust,ignore
chars(string: &str) -> CharsStream
chars(string: &str, range: Range<i64>) -> CharsStream
chars(string: &str, start: i64) -> CharsStream
chars(string: &str, range: RangeInclusive<i64>) -> CharsStream
chars(string: &str, start: i64, len: i64) -> CharsStream
```

Return an iterator over the characters in the string.

#### Example

```rhai
for ch in "hello, world!".chars() {
print(ch);
}
```

</div>
</div>




<div class='doc-block'>

## chop

<div class='doc-content'>

```rust,ignore
chop(blob: &mut Blob, len: i64)
chop(array: &mut Array, len: i64)
```

Cut off the head of the BLOB, leaving a tail of the specified length.

* If `len` ≤ 0, the BLOB is cleared.
* If `len` ≥ length of BLOB, the BLOB is not modified.

#### Example

```rhai
let b = blob();

b += 1; b += 2; b += 3; b += 4; b += 5;

b.chop(3);

print(b);           // prints "[030405]"

b.chop(10);

print(b);           // prints "[030405]"
```

</div>
</div>




<div class='doc-block'>

## clear

<div class='doc-content'>

```rust,ignore
clear(string: &mut ImmutableString)
clear(array: &mut Array)
clear(map: &mut Map)
clear(blob: &mut Blob)
```

Clear the string, making it empty.

</div>
</div>




<div class='doc-block'>

## contains

<div class='doc-content'>

```rust,ignore
contains(array: &mut Array, value: Dynamic) -> Result<bool, Box<EvalAltResult>>
contains(string: &str, match_string: &str) -> bool
contains(range: &mut Range<i64>, value: i64) -> bool
contains(blob: &mut Blob, value: i64) -> bool
contains(range: &mut RangeInclusive<i64>, value: i64) -> bool
contains(map: &mut Map, property: &str) -> bool
contains(string: &str, character: char) -> bool
```

Return `true` if the array contains an element that equals `value`.

The operator `==` is used to compare elements with `value` and must be defined,
otherwise `false` is assumed.

This function also drives the `in` operator.

#### Example

```rhai
let x = [1, 2, 3, 4, 5];

// The 'in' operator calls 'contains' in the background
if 4 in x {
print("found!");
}
```

</div>
</div>




<div class='doc-block'>

## cos

<div class='doc-content'>

```rust,ignore
cos(x: f64) -> f64
```

Return the cosine of the floating-point number in radians.

</div>
</div>




<div class='doc-block'>

## cosh

<div class='doc-content'>

```rust,ignore
cosh(x: f64) -> f64
```

Return the hyperbolic cosine of the floating-point number in radians.

</div>
</div>




<div class='doc-block'>

## crop

<div class='doc-content'>

```rust,ignore
crop(string: &mut ImmutableString, range: RangeInclusive<i64>)
crop(string: &mut ImmutableString, start: i64)
crop(string: &mut ImmutableString, range: Range<i64>)
crop(string: &mut ImmutableString, start: i64, len: i64)
```

Remove all characters from the string except those within an inclusive `range`.

#### Example

```rhai
let text = "hello, world!";

text.crop(2..=8);

print(text);        // prints "llo, wo"
```

</div>
</div>




<div class='doc-block'>

## debug

<div class='doc-content'>

```rust,ignore
debug() -> ImmutableString
debug(character: char) -> ImmutableString
debug(item: &mut Dynamic) -> ImmutableString
debug(f: &mut FnPtr) -> ImmutableString
debug(value: bool) -> ImmutableString
debug(number: f32) -> ImmutableString
debug(string: &str) -> ImmutableString
debug(array: &mut Array) -> ImmutableString
debug(map: &mut Map) -> ImmutableString
debug(unit: ()) -> ImmutableString
debug(number: f64) -> ImmutableString
```

Return the empty string.

</div>
</div>




<div class='doc-block'>

## dedup

<div class='doc-content'>

```rust,ignore
dedup(array: &mut Array)
dedup(array: &mut Array, comparer: FnPtr)
dedup(array: &mut Array, comparer: &str) -> Result<(), Box<EvalAltResult>>
```

Remove duplicated _consecutive_ elements from the array.

The operator `==` is used to compare elements and must be defined,
otherwise `false` is assumed.

#### Example

```rhai
let x = [1, 2, 2, 2, 3, 4, 3, 3, 2, 1];

x.dedup();

print(x);       // prints "[1, 2, 3, 4, 3, 2, 1]"
```

</div>
</div>




<div class='doc-block'>

## drain

<div class='doc-content'>

```rust,ignore
drain(blob: &mut Blob, range: Range<i64>) -> Blob
drain(map: &mut Map, filter: FnPtr) -> Result<Map, Box<EvalAltResult>>
drain(blob: &mut Blob, range: RangeInclusive<i64>) -> Blob
drain(array: &mut Array, filter: FnPtr) -> Result<Array, Box<EvalAltResult>>
drain(array: &mut Array, range: RangeInclusive<i64>) -> Array
drain(array: &mut Array, range: Range<i64>) -> Array
drain(array: &mut Array, filter: &str) -> Result<Array, Box<EvalAltResult>>
drain(array: &mut Array, start: i64, len: i64) -> Array
drain(blob: &mut Blob, start: i64, len: i64) -> Blob
```

Remove all bytes in the BLOB within an exclusive `range` and return them as a new BLOB.

#### Example

```rhai
let b1 = blob();

b1 += 1; b1 += 2; b1 += 3; b1 += 4; b1 += 5;

let b2 = b1.drain(1..3);

print(b1);      // prints "[010405]"

print(b2);      // prints "[0203]"

let b3 = b1.drain(2..3);

print(b1);      // prints "[0104]"

print(b3);      // prints "[05]"
```

</div>
</div>




<div class='doc-block'>

## elapsed

<div class='doc-content'>

```rust,ignore
elapsed(timestamp: Instant) -> Result<Dynamic, Box<EvalAltResult>>
```

Return the number of seconds between the current system time and the timestamp.

#### Example

```rhai
let now = timestamp();

sleep(10.0);            // sleep for 10 seconds

print(now.elapsed);     // prints 10.???
```

</div>
</div>




<div class='doc-block'>

## end

<div class='doc-content'>

```rust,ignore
end(range: &mut RangeInclusive<i64>) -> i64
end(range: &mut Range<i64>) -> i64
```

Return the end of the inclusive range.

</div>
</div>




<div class='doc-block'>

## ends_with

<div class='doc-content'>

```rust,ignore
ends_with(string: &str, match_string: &str) -> bool
```

Return `true` if the string ends with a specified string.

#### Example

```rhai
let text = "hello, world!";

print(text.ends_with("world!"));    // prints true

print(text.ends_with("hello"));     // prints false
```

</div>
</div>




<div class='doc-block'>

## exit

<div class='doc-content'>

```rust,ignore
exit() -> Result<Dynamic, Box<EvalAltResult>>
exit(value: Dynamic) -> Result<Dynamic, Box<EvalAltResult>>
```

Exit the script evaluation immediately with `()` as exit value.

#### Example
```rhai
exit();
```

</div>
</div>




<div class='doc-block'>

## exp

<div class='doc-content'>

```rust,ignore
exp(x: f64) -> f64
```

Return the exponential of the floating-point number.

</div>
</div>




<div class='doc-block'>

## extract

<div class='doc-content'>

```rust,ignore
extract(blob: &mut Blob, range: Range<i64>) -> Blob
extract(array: &mut Array, start: i64) -> Array
extract(blob: &mut Blob, range: RangeInclusive<i64>) -> Blob
extract(array: &mut Array, range: RangeInclusive<i64>) -> Array
extract(array: &mut Array, range: Range<i64>) -> Array
extract(blob: &mut Blob, start: i64) -> Blob
extract(array: &mut Array, start: i64, len: i64) -> Array
extract(blob: &mut Blob, start: i64, len: i64) -> Blob
```

Copy an exclusive `range` of the BLOB and return it as a new BLOB.

#### Example

```rhai
let b = blob();

b += 1; b += 2; b += 3; b += 4; b += 5;

print(b.extract(1..3));     // prints "[0203]"

print(b);                   // prints "[0102030405]"
```

</div>
</div>




<div class='doc-block'>

## f32.is_zero

<div class='doc-content'>

```rust,ignore
get$is_zero(x: f32) -> bool
```

Return true if the floating-point number is zero.

</div>
</div>




<div class='doc-block'>

## fill_with

<div class='doc-content'>

```rust,ignore
fill_with(map: &mut Map, map2: Map)
```

Add all property values of another object map into the object map.
Only properties that do not originally exist in the object map are added.

#### Example

```rhai
let m = #{a:1, b:2, c:3};
let n = #{a: 42, d:0};

m.fill_with(n);

print(m);       // prints "#{a:1, b:2, c:3, d:0}"
```

</div>
</div>




<div class='doc-block'>

## filter

<div class='doc-content'>

```rust,ignore
filter(map: &mut Map, filter: FnPtr) -> Result<Map, Box<EvalAltResult>>
filter(array: &mut Array, filter: FnPtr) -> Result<Array, Box<EvalAltResult>>
filter(array: &mut Array, filter_func: &str) -> Result<Array, Box<EvalAltResult>>
```

Iterate through all the elements in the object map, applying a `filter` function to each
and return a new collection of all elements that return `true` as a new object map.

#### Function Parameters

* `key`: current key
* `value` _(optional)_: copy of element (bound to `this` if omitted)

#### Example

```rhai
let x = #{a:1, b:2, c:3, d:4, e:5};

let y = x.filter(|k| this >= 3);

print(y);       // prints #{"c":3, "d":4, "e":5}

let y = x.filter(|k, v| k != "d" && v < 5);

print(y);       // prints #{"a":1, "b":2, "c":3}
```

</div>
</div>




<div class='doc-block'>

## find

<div class='doc-content'>

```rust,ignore
find(array: &mut Array, filter: FnPtr) -> Result<Dynamic, Box<EvalAltResult>>
find(array: &mut Array, filter: FnPtr, start: i64) -> Result<Dynamic, Box<EvalAltResult>>
```

Iterate through all the elements in the array, applying a `filter` function to each element
in turn, and return a copy of the first element that returns `true`. If no element returns
`true`, `()` is returned.

#### No Function Parameter

Array element (mutable) is bound to `this`.

#### Function Parameters

* `element`: copy of array element
* `index` _(optional)_: current index in the array

#### Example

```rhai
let x = [1, 2, 3, 5, 8, 13];

print(x.find(|v| v > 3));                    // prints 5: 5 > 3

print(x.find(|v| v > 13) ?? "not found");    // prints "not found": nothing is > 13

print(x.find(|v, i| v * i > 13));            // prints 5: 3 * 5 > 13
```

</div>
</div>




<div class='doc-block'>

## find_map

<div class='doc-content'>

```rust,ignore
find_map(array: &mut Array, filter: FnPtr) -> Result<Dynamic, Box<EvalAltResult>>
find_map(array: &mut Array, filter: FnPtr, start: i64) -> Result<Dynamic, Box<EvalAltResult>>
```

Iterate through all the elements in the array, applying a `mapper` function to each element
in turn, and return the first result that is not `()`. Otherwise, `()` is returned.

#### No Function Parameter

Array element (mutable) is bound to `this`.

This method is marked _pure_; the `mapper` function should not mutate array elements.

#### Function Parameters

* `element`: copy of array element
* `index` _(optional)_: current index in the array

#### Example

```rhai
let x = [#{alice: 1}, #{bob: 2}, #{clara: 3}];

print(x.find_map(|v| v.alice));                  // prints 1

print(x.find_map(|v| v.dave) ?? "not found");    // prints "not found"

print(x.find_map(|| this.dave) ?? "not found");  // prints "not found"
```

</div>
</div>




<div class='doc-block'>

## float.ceiling

<div class='doc-content'>

```rust,ignore
get$ceiling(x: f64) -> f64
```

Return the smallest whole number larger than or equals to the floating-point number.

</div>
</div>




<div class='doc-block'>

## float.floor

<div class='doc-content'>

```rust,ignore
get$floor(x: f64) -> f64
```

Return the largest whole number less than or equals to the floating-point number.

</div>
</div>




<div class='doc-block'>

## float.fraction

<div class='doc-content'>

```rust,ignore
get$fraction(x: f64) -> f64
```

Return the fractional part of the floating-point number.

</div>
</div>




<div class='doc-block'>

## float.int

<div class='doc-content'>

```rust,ignore
get$int(x: f64) -> f64
```

Return the integral part of the floating-point number.

</div>
</div>




<div class='doc-block'>

## float.is_finite

<div class='doc-content'>

```rust,ignore
get$is_finite(x: f64) -> bool
```

Return `true` if the floating-point number is finite.

</div>
</div>




<div class='doc-block'>

## float.is_infinite

<div class='doc-content'>

```rust,ignore
get$is_infinite(x: f64) -> bool
```

Return `true` if the floating-point number is infinite.

</div>
</div>




<div class='doc-block'>

## float.is_nan

<div class='doc-content'>

```rust,ignore
get$is_nan(x: f64) -> bool
```

Return `true` if the floating-point number is `NaN` (Not A Number).

</div>
</div>




<div class='doc-block'>

## float.is_zero

<div class='doc-content'>

```rust,ignore
get$is_zero(x: f64) -> bool
```

Return true if the floating-point number is zero.

</div>
</div>




<div class='doc-block'>

## float.round

<div class='doc-content'>

```rust,ignore
get$round(x: f64) -> f64
```

Return the nearest whole number closest to the floating-point number.
Rounds away from zero.

</div>
</div>




<div class='doc-block'>

## floor

<div class='doc-content'>

```rust,ignore
floor(x: f64) -> f64
```

Return the largest whole number less than or equals to the floating-point number.

</div>
</div>




<div class='doc-block'>

## for_each

<div class='doc-content'>

```rust,ignore
for_each(array: &mut Array, map: FnPtr) -> Result<(), Box<EvalAltResult>>
```

Iterate through all the elements in the array, applying a `process` function to each element in turn.
Each element is bound to `this` before calling the function.

#### Function Parameters

* `this`: bound to array element (mutable)
* `index` _(optional)_: current index in the array

#### Example

```rhai
let x = [1, 2, 3, 4, 5];

x.for_each(|| this *= this);

print(x);       // prints "[1, 4, 9, 16, 25]"

x.for_each(|i| this *= i);

print(x);       // prints "[0, 2, 6, 12, 20]"
```

</div>
</div>




<div class='doc-block'>

## fraction

<div class='doc-content'>

```rust,ignore
fraction(x: f64) -> f64
```

Return the fractional part of the floating-point number.

</div>
</div>




<div class='doc-block'>

## get

<div class='doc-content'>

```rust,ignore
get(blob: &mut Blob, index: i64) -> i64
get(string: &str, index: i64) -> Dynamic
get(map: &mut Map, property: &str) -> Dynamic
get(array: &mut Array, index: i64) -> Dynamic
```

Get the byte value at the `index` position in the BLOB.

* If `index` < 0, position counts from the end of the BLOB (`-1` is the last element).
* If `index` < -length of BLOB, zero is returned.
* If `index` ≥ length of BLOB, zero is returned.

#### Example

```rhai
let b = blob();

b += 1; b += 2; b += 3; b += 4; b += 5;

print(b.get(0));        // prints 1

print(b.get(-1));       // prints 5

print(b.get(99));       // prints 0
```

</div>
</div>




<div class='doc-block'>

## get_bit

<div class='doc-content'>

```rust,ignore
get_bit(value: i64, bit: i64) -> Result<bool, Box<EvalAltResult>>
```

Return `true` if the specified `bit` in the number is set.

If `bit` < 0, position counts from the MSB (Most Significant Bit).

#### Example

```rhai
let x = 123456;

print(x.get_bit(5));    // prints false

print(x.get_bit(6));    // prints true

print(x.get_bit(-48));  // prints true on 64-bit
```

</div>
</div>




<div class='doc-block'>

## get_bits

<div class='doc-content'>

```rust,ignore
get_bits(value: i64, range: RangeInclusive<i64>) -> Result<i64, Box<EvalAltResult>>
get_bits(value: i64, range: Range<i64>) -> Result<i64, Box<EvalAltResult>>
get_bits(value: i64, start: i64, bits: i64) -> Result<i64, Box<EvalAltResult>>
```

Return an inclusive range of bits in the number as a new number.

#### Example

```rhai
let x = 123456;

print(x.get_bits(5..=9));       // print 18
```

</div>
</div>




<div class='doc-block'>

## get_fn_metadata_list

<div class='doc-content'>

```rust,ignore
get_fn_metadata_list() -> Array
get_fn_metadata_list(name: &str) -> Array
get_fn_metadata_list(name: &str, params: i64) -> Array
```

Return an array of object maps containing metadata of all script-defined functions.

</div>
</div>




<div class='doc-block'>

## hypot

<div class='doc-content'>

```rust,ignore
hypot(x: f64, y: f64) -> f64
```

Return the hypotenuse of a triangle with sides `x` and `y`.

</div>
</div>




<div class='doc-block'>

## index_of

<div class='doc-content'>

```rust,ignore
index_of(array: &mut Array, value: Dynamic) -> Result<i64, Box<EvalAltResult>>
index_of(array: &mut Array, filter: FnPtr) -> Result<i64, Box<EvalAltResult>>
index_of(string: &str, find_string: &str) -> i64
index_of(array: &mut Array, filter: &str) -> Result<i64, Box<EvalAltResult>>
index_of(string: &str, character: char) -> i64
index_of(array: &mut Array, filter: &str, start: i64) -> Result<i64, Box<EvalAltResult>>
index_of(array: &mut Array, value: Dynamic, start: i64) -> Result<i64, Box<EvalAltResult>>
index_of(string: &str, character: char, start: i64) -> i64
index_of(string: &str, find_string: &str, start: i64) -> i64
index_of(array: &mut Array, filter: FnPtr, start: i64) -> Result<i64, Box<EvalAltResult>>
```

Find the first element in the array that equals a particular `value` and return its index.
If no element equals `value`, `-1` is returned.

The operator `==` is used to compare elements with `value` and must be defined,
otherwise `false` is assumed.

#### Example

```rhai
let x = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 5];

print(x.index_of(4));       // prints 3 (first index)

print(x.index_of(9));       // prints -1

print(x.index_of("foo"));   // prints -1: strings do not equal numbers
```

</div>
</div>




<div class='doc-block'>

## insert

<div class='doc-content'>

```rust,ignore
insert(blob: &mut Blob, index: i64, value: i64)
insert(array: &mut Array, index: i64, item: Dynamic)
```

Add a byte `value` to the BLOB at a particular `index` position.

* If `index` < 0, position counts from the end of the BLOB (`-1` is the last byte).
* If `index` < -length of BLOB, the byte value is added to the beginning of the BLOB.
* If `index` ≥ length of BLOB, the byte value is appended to the end of the BLOB.

Only the lower 8 bits of the `value` are used; all other bits are ignored.

#### Example

```rhai
let b = blob(5, 0x42);

b.insert(2, 0x18);

print(b);       // prints "[4242184242]"
```

</div>
</div>




<div class='doc-block'>

## int

<div class='doc-content'>

```rust,ignore
int(x: f64) -> f64
```

Return the integral part of the floating-point number.

</div>
</div>




<div class='doc-block'>

## int.bits

<div class='doc-content'>

```rust,ignore
get$bits(value: i64) -> Result<BitRange, Box<EvalAltResult>>
```

Return an iterator over all the bits in the number.

#### Example

```rhai
let x = 123456;

for bit in x.bits {
print(bit);
}
```

</div>
</div>




<div class='doc-block'>

## int.is_even

<div class='doc-content'>

```rust,ignore
get$is_even(x: i64) -> bool
```

Return true if the number is even.

</div>
</div>




<div class='doc-block'>

## int.is_odd

<div class='doc-content'>

```rust,ignore
get$is_odd(x: i64) -> bool
```

Return true if the number is odd.

</div>
</div>




<div class='doc-block'>

## int.is_zero

<div class='doc-content'>

```rust,ignore
get$is_zero(x: i64) -> bool
```

Return true if the number is zero.

</div>
</div>




<div class='doc-block'>

## is_anonymous

<div class='doc-content'>

```rust,ignore
is_anonymous(fn_ptr: &mut FnPtr) -> bool
```

Return `true` if the function is an anonymous function.

#### Example

```rhai
let f = |x| x * 2;

print(f.is_anonymous);      // prints true
```

</div>
</div>




<div class='doc-block'>

## is_empty

<div class='doc-content'>

```rust,ignore
is_empty(blob: &mut Blob) -> bool
is_empty(array: &mut Array) -> bool
is_empty(string: &str) -> bool
is_empty(map: &mut Map) -> bool
is_empty(range: &mut Range<i64>) -> bool
is_empty(range: &mut RangeInclusive<i64>) -> bool
```

Return true if the BLOB is empty.

</div>
</div>




<div class='doc-block'>

## is_even

<div class='doc-content'>

```rust,ignore
is_even(x: i32) -> bool
is_even(x: u16) -> bool
is_even(x: i16) -> bool
is_even(x: i128) -> bool
is_even(x: i8) -> bool
is_even(x: u32) -> bool
is_even(x: u8) -> bool
is_even(x: u128) -> bool
is_even(x: i64) -> bool
is_even(x: u64) -> bool
```

Return true if the number is even.

</div>
</div>




<div class='doc-block'>

## is_exclusive

<div class='doc-content'>

```rust,ignore
is_exclusive(range: &mut RangeInclusive<i64>) -> bool
is_exclusive(range: &mut Range<i64>) -> bool
```

Return `true` if the range is exclusive.

</div>
</div>




<div class='doc-block'>

## is_finite

<div class='doc-content'>

```rust,ignore
is_finite(x: f64) -> bool
```

Return `true` if the floating-point number is finite.

</div>
</div>




<div class='doc-block'>

## is_inclusive

<div class='doc-content'>

```rust,ignore
is_inclusive(range: &mut RangeInclusive<i64>) -> bool
is_inclusive(range: &mut Range<i64>) -> bool
```

Return `true` if the range is inclusive.

</div>
</div>




<div class='doc-block'>

## is_infinite

<div class='doc-content'>

```rust,ignore
is_infinite(x: f64) -> bool
```

Return `true` if the floating-point number is infinite.

</div>
</div>




<div class='doc-block'>

## is_nan

<div class='doc-content'>

```rust,ignore
is_nan(x: f64) -> bool
```

Return `true` if the floating-point number is `NaN` (Not A Number).

</div>
</div>




<div class='doc-block'>

## is_odd

<div class='doc-content'>

```rust,ignore
is_odd(x: i8) -> bool
is_odd(x: u128) -> bool
is_odd(x: u8) -> bool
is_odd(x: u32) -> bool
is_odd(x: i64) -> bool
is_odd(x: u64) -> bool
is_odd(x: i32) -> bool
is_odd(x: i16) -> bool
is_odd(x: u16) -> bool
is_odd(x: i128) -> bool
```

Return true if the number is odd.

</div>
</div>




<div class='doc-block'>

## is_zero

<div class='doc-content'>

```rust,ignore
is_zero(x: i64) -> bool
is_zero(x: f64) -> bool
is_zero(x: u64) -> bool
is_zero(x: f32) -> bool
is_zero(x: i8) -> bool
is_zero(x: u8) -> bool
is_zero(x: u128) -> bool
is_zero(x: u32) -> bool
is_zero(x: i16) -> bool
is_zero(x: u16) -> bool
is_zero(x: i128) -> bool
is_zero(x: i32) -> bool
```

Return true if the number is zero.

</div>
</div>




<div class='doc-block'>

## keys

<div class='doc-content'>

```rust,ignore
keys(map: &mut Map) -> Array
```

Return an array with all the property names in the object map.

#### Example

```rhai
let m = #{a:1, b:2, c:3};

print(m.keys());        // prints ["a", "b", "c"]
```

</div>
</div>




<div class='doc-block'>

## len

<div class='doc-content'>

```rust,ignore
len(string: &str) -> i64
len(array: &mut Array) -> i64
len(map: &mut Map) -> i64
len(blob: &mut Blob) -> i64
```

Return the length of the string, in number of characters.

#### Example

```rhai
let text = "朝には紅顔ありて夕べには白骨となる";

print(text.len);        // prints 17
```

</div>
</div>




<div class='doc-block'>

## ln

<div class='doc-content'>

```rust,ignore
ln(x: f64) -> f64
```

Return the natural log of the floating-point number.

</div>
</div>




<div class='doc-block'>

## log

<div class='doc-content'>

```rust,ignore
log(x: f64) -> f64
log(x: f64, base: f64) -> f64
```

Return the log of the floating-point number with base 10.

</div>
</div>




<div class='doc-block'>

## make_lower

<div class='doc-content'>

```rust,ignore
make_lower(character: &mut char)
make_lower(string: &mut ImmutableString)
```

Convert the character to lower-case.

#### Example

```rhai
let ch = 'A';

ch.make_lower();

print(ch);          // prints 'a'
```

</div>
</div>




<div class='doc-block'>

## make_upper

<div class='doc-content'>

```rust,ignore
make_upper(character: &mut char)
make_upper(string: &mut ImmutableString)
```

Convert the character to upper-case.

#### Example

```rhai
let ch = 'a';

ch.make_upper();

print(ch);          // prints 'A'
```

</div>
</div>




<div class='doc-block'>

## map

<div class='doc-content'>

```rust,ignore
map(array: &mut Array, map: FnPtr) -> Result<Array, Box<EvalAltResult>>
map(array: &mut Array, mapper: &str) -> Result<Array, Box<EvalAltResult>>
```

Iterate through all the elements in the array, applying a `mapper` function to each element
in turn, and return the results as a new array.

#### No Function Parameter

Array element (mutable) is bound to `this`.

This method is marked _pure_; the `mapper` function should not mutate array elements.

#### Function Parameters

* `element`: copy of array element
* `index` _(optional)_: current index in the array

#### Example

```rhai
let x = [1, 2, 3, 4, 5];

let y = x.map(|v| v * v);

print(y);       // prints "[1, 4, 9, 16, 25]"

let y = x.map(|v, i| v * i);

print(y);       // prints "[0, 2, 6, 12, 20]"
```

</div>
</div>




<div class='doc-block'>

## max

<div class='doc-content'>

```rust,ignore
max(x: f64, y: f64) -> f64
max(x: i32, y: i32) -> i32
max(x: u128, y: u128) -> u128
max(x: i8, y: i8) -> i8
max(char1: char, char2: char) -> char
max(x: f32, y: i64) -> f32
max(x: i16, y: i16) -> i16
max(x: f32, y: f32) -> f32
max(x: u8, y: u8) -> u8
max(x: u64, y: u64) -> u64
max(x: f64, y: f32) -> f64
max(x: u32, y: u32) -> u32
max(string1: ImmutableString, string2: ImmutableString) -> ImmutableString
max(x: f32, y: f64) -> f64
max(x: i64, y: f64) -> f64
max(x: i64, y: f32) -> f32
max(x: i64, y: i64) -> i64
max(x: u16, y: u16) -> u16
max(x: f64, y: i64) -> f64
max(x: i128, y: i128) -> i128
```

Return the character that is lexically greater than the other character.

#### Example

```rhai
max('h', 'w');      // returns 'w'
```

</div>
</div>




<div class='doc-block'>

## min

<div class='doc-content'>

```rust,ignore
min(string1: ImmutableString, string2: ImmutableString) -> ImmutableString
min(x: f32, y: i64) -> f32
min(x: f64, y: f32) -> f64
min(x: u32, y: u32) -> u32
min(x: u8, y: u8) -> u8
min(x: u64, y: u64) -> u64
min(x: i16, y: i16) -> i16
min(x: f32, y: f32) -> f32
min(x: i8, y: i8) -> i8
min(char1: char, char2: char) -> char
min(x: i32, y: i32) -> i32
min(x: u128, y: u128) -> u128
min(x: f64, y: f64) -> f64
min(x: f64, y: i64) -> f64
min(x: u16, y: u16) -> u16
min(x: i128, y: i128) -> i128
min(x: i64, y: f32) -> f32
min(x: i64, y: i64) -> i64
min(x: f32, y: f64) -> f64
min(x: i64, y: f64) -> f64
```

Return the string that is lexically smaller than the other string.

#### Example

```rhai
min("hello", "world");      // returns "hello"
```

</div>
</div>




<div class='doc-block'>

## mixin

<div class='doc-content'>

```rust,ignore
mixin(map: &mut Map, map2: Map)
```

Add all property values of another object map into the object map.
Existing property values of the same names are replaced.

#### Example

```rhai
let m = #{a:1, b:2, c:3};
let n = #{a: 42, d:0};

m.mixin(n);

print(m);       // prints "#{a:42, b:2, c:3, d:0}"
```

</div>
</div>




<div class='doc-block'>

## name

<div class='doc-content'>

```rust,ignore
name(fn_ptr: &mut FnPtr) -> ImmutableString
```

Return the name of the function.

#### Example

```rhai
fn double(x) { x * 2 }

let f = Fn("double");

print(f.name);      // prints "double"
```

</div>
</div>




<div class='doc-block'>

## pad

<div class='doc-content'>

```rust,ignore
pad(string: &mut ImmutableString, len: i64, padding: &str) -> Result<(), Box<EvalAltResult>>
pad(blob: &mut Blob, len: i64, value: i64) -> Result<(), Box<EvalAltResult>>
pad(array: &mut Array, len: i64, item: Dynamic) -> Result<(), Box<EvalAltResult>>
pad(string: &mut ImmutableString, len: i64, character: char) -> Result<(), Box<EvalAltResult>>
```

Pad the string to at least the specified number of characters with the specified string.

If `len` ≤ length of string, no padding is done.

#### Example

```rhai
let text = "hello";

text.pad(10, "(!)");

print(text);        // prints "hello(!)(!)"

text.pad(8, '***');

print(text);        // prints "hello(!)(!)"
```

</div>
</div>




<div class='doc-block'>

## parse_be_float

<div class='doc-content'>

```rust,ignore
parse_be_float(blob: &mut Blob, range: Range<i64>) -> f64
parse_be_float(blob: &mut Blob, range: RangeInclusive<i64>) -> f64
parse_be_float(blob: &mut Blob, start: i64, len: i64) -> f64
```

Parse the bytes within an exclusive `range` in the BLOB as a `FLOAT`
in big-endian byte order.

* If number of bytes in `range` < number of bytes for `FLOAT`, zeros are padded.
* If number of bytes in `range` > number of bytes for `FLOAT`, extra bytes are ignored.

</div>
</div>




<div class='doc-block'>

## parse_be_int

<div class='doc-content'>

```rust,ignore
parse_be_int(blob: &mut Blob, range: Range<i64>) -> i64
parse_be_int(blob: &mut Blob, range: RangeInclusive<i64>) -> i64
parse_be_int(blob: &mut Blob, start: i64, len: i64) -> i64
```

Parse the bytes within an exclusive `range` in the BLOB as an `INT`
in big-endian byte order.

* If number of bytes in `range` < number of bytes for `INT`, zeros are padded.
* If number of bytes in `range` > number of bytes for `INT`, extra bytes are ignored.

```rhai
let b = blob();

b += 1; b += 2; b += 3; b += 4; b += 5;

let x = b.parse_be_int(1..3);   // parse two bytes

print(x.to_hex());              // prints "02030000...00"
```

</div>
</div>




<div class='doc-block'>

## parse_float

<div class='doc-content'>

```rust,ignore
parse_float(string: &str) -> Result<f64, Box<EvalAltResult>>
```

Parse a string into a floating-point number.

#### Example

```rhai
let x = parse_int("123.456");

print(x);       // prints 123.456
```

</div>
</div>




<div class='doc-block'>

## parse_int

<div class='doc-content'>

```rust,ignore
parse_int(string: &str) -> Result<i64, Box<EvalAltResult>>
parse_int(string: &str, radix: i64) -> Result<i64, Box<EvalAltResult>>
```

Parse a string into an integer number.

#### Example

```rhai
let x = parse_int("123");

print(x);       // prints 123
```

</div>
</div>




<div class='doc-block'>

## parse_json

<div class='doc-content'>

```rust,ignore
parse_json(json: &str) -> Result<Dynamic, Box<EvalAltResult>>
```

Parse a JSON string into a value.

#### Example

```rhai
let m = parse_json(`{"a":1, "b":2, "c":3}`);

print(m);       // prints #{"a":1, "b":2, "c":3}
```

</div>
</div>




<div class='doc-block'>

## parse_le_float

<div class='doc-content'>

```rust,ignore
parse_le_float(blob: &mut Blob, range: RangeInclusive<i64>) -> f64
parse_le_float(blob: &mut Blob, range: Range<i64>) -> f64
parse_le_float(blob: &mut Blob, start: i64, len: i64) -> f64
```

Parse the bytes within an inclusive `range` in the BLOB as a `FLOAT`
in little-endian byte order.

* If number of bytes in `range` < number of bytes for `FLOAT`, zeros are padded.
* If number of bytes in `range` > number of bytes for `FLOAT`, extra bytes are ignored.

</div>
</div>




<div class='doc-block'>

## parse_le_int

<div class='doc-content'>

```rust,ignore
parse_le_int(blob: &mut Blob, range: RangeInclusive<i64>) -> i64
parse_le_int(blob: &mut Blob, range: Range<i64>) -> i64
parse_le_int(blob: &mut Blob, start: i64, len: i64) -> i64
```

Parse the bytes within an inclusive `range` in the BLOB as an `INT`
in little-endian byte order.

* If number of bytes in `range` < number of bytes for `INT`, zeros are padded.
* If number of bytes in `range` > number of bytes for `INT`, extra bytes are ignored.

```rhai
let b = blob();

b += 1; b += 2; b += 3; b += 4; b += 5;

let x = b.parse_le_int(1..=3);  // parse three bytes

print(x.to_hex());              // prints "040302"
```

</div>
</div>




<div class='doc-block'>

## pop

<div class='doc-content'>

```rust,ignore
pop(blob: &mut Blob) -> i64
pop(array: &mut Array) -> Dynamic
pop(string: &mut ImmutableString) -> Dynamic
pop(string: &mut ImmutableString, len: i64) -> ImmutableString
```

Remove the last byte from the BLOB and return it.

If the BLOB is empty, zero is returned.

#### Example

```rhai
let b = blob();

b += 1; b += 2; b += 3; b += 4; b += 5;

print(b.pop());         // prints 5

print(b);               // prints "[01020304]"
```

</div>
</div>




<div class='doc-block'>

## print

<div class='doc-content'>

```rust,ignore
print() -> ImmutableString
print(value: bool) -> ImmutableString
print(item: &mut Dynamic) -> ImmutableString
print(character: char) -> ImmutableString
print(number: f64) -> ImmutableString
print(unit: ()) -> ImmutableString
print(map: &mut Map) -> ImmutableString
print(array: &mut Array) -> ImmutableString
print(string: ImmutableString) -> ImmutableString
print(number: f32) -> ImmutableString
```

Return the empty string.

</div>
</div>




<div class='doc-block'>

## push

<div class='doc-content'>

```rust,ignore
push(blob: &mut Blob, value: i64)
push(array: &mut Array, item: Dynamic)
```

Add a new byte `value` to the end of the BLOB.

Only the lower 8 bits of the `value` are used; all other bits are ignored.

#### Example

```rhai
let b = blob();

b.push(0x42);

print(b);       // prints "[42]"
```

</div>
</div>




<div class='doc-block'>

## range

<div class='doc-content'>

```rust,ignore
range(range: std::ops::Range<i64>, step: i64) -> Result<StepRange<i64>, Box<EvalAltResult>>
range(range: std::ops::Range<u128>, step: u128) -> Result<StepRange<u128>, Box<EvalAltResult>>
range(from: u8, to: u8) -> Range<u8>
range(from: u64, to: u64) -> Range<u64>
range(range: std::ops::Range<u32>, step: u32) -> Result<StepRange<u32>, Box<EvalAltResult>>
range(from: u32, to: u32) -> Range<u32>
range(range: std::ops::Range<i16>, step: i16) -> Result<StepRange<i16>, Box<EvalAltResult>>
range(from: i16, to: i16) -> Range<i16>
range(range: std::ops::Range<i8>, step: i8) -> Result<StepRange<i8>, Box<EvalAltResult>>
range(from: i8, to: i8) -> Range<i8>
range(range: std::ops::Range<u16>, step: u16) -> Result<StepRange<u16>, Box<EvalAltResult>>
range(range: std::ops::Range<u64>, step: u64) -> Result<StepRange<u64>, Box<EvalAltResult>>
range(from: i32, to: i32) -> Range<i32>
range(from: u128, to: u128) -> Range<u128>
range(range: std::ops::Range<u8>, step: u8) -> Result<StepRange<u8>, Box<EvalAltResult>>
range(from: u16, to: u16) -> Range<u16>
range(from: i128, to: i128) -> Range<i128>
range(range: std::ops::Range<i32>, step: i32) -> Result<StepRange<i32>, Box<EvalAltResult>>
range(range: std::ops::Range<FLOAT>, step: f64) -> Result<StepRange<FLOAT>, Box<EvalAltResult>>
range(range: std::ops::Range<i128>, step: i128) -> Result<StepRange<i128>, Box<EvalAltResult>>
range(from: i64, to: i64) -> Range<i64>
range(from: u8, to: u8, step: u8) -> Result<StepRange<u8>, Box<EvalAltResult>>
range(from: i64, to: i64, step: i64) -> Result<StepRange<i64>, Box<EvalAltResult>>
range(from: u64, to: u64, step: u64) -> Result<StepRange<u64>, Box<EvalAltResult>>
range(from: f64, to: f64, step: f64) -> Result<StepRange<FLOAT>, Box<EvalAltResult>>
range(from: u32, to: u32, step: u32) -> Result<StepRange<u32>, Box<EvalAltResult>>
range(from: i8, to: i8, step: i8) -> Result<StepRange<i8>, Box<EvalAltResult>>
range(from: i32, to: i32, step: i32) -> Result<StepRange<i32>, Box<EvalAltResult>>
range(from: u128, to: u128, step: u128) -> Result<StepRange<u128>, Box<EvalAltResult>>
range(from: i16, to: i16, step: i16) -> Result<StepRange<i16>, Box<EvalAltResult>>
range(from: u16, to: u16, step: u16) -> Result<StepRange<u16>, Box<EvalAltResult>>
range(from: i128, to: i128, step: i128) -> Result<StepRange<i128>, Box<EvalAltResult>>
```

Return an iterator over an exclusive range, each iteration increasing by `step`.

If `range` is reversed and `step` < 0, iteration goes backwards.

Otherwise, if `range` is empty, an empty iterator is returned.

#### Example

```rhai
// prints all values from 8 to 17 in steps of 3
for n in range(8..18, 3) {
print(n);
}

// prints all values down from 18 to 9 in steps of -3
for n in range(18..8, -3) {
print(n);
}
```

</div>
</div>




<div class='doc-block'>

## reduce

<div class='doc-content'>

```rust,ignore
reduce(array: &mut Array, reducer: &str) -> Result<Dynamic, Box<EvalAltResult>>
reduce(array: &mut Array, reducer: FnPtr) -> Result<Dynamic, Box<EvalAltResult>>
reduce(array: &mut Array, reducer: FnPtr, initial: Dynamic) -> Result<Dynamic, Box<EvalAltResult>>
reduce(array: &mut Array, reducer: &str, initial: Dynamic) -> Result<Dynamic, Box<EvalAltResult>>
```

Reduce an array by iterating through all elements while applying a function named by `reducer`.

#### Deprecated API

This method is deprecated and will be removed from the next major version.
Use `array.reduce(Fn("fn_name"))` instead.

#### Function Parameters

A function with the same name as the value of `reducer` must exist taking these parameters:

* `result`: accumulated result, initially `()`
* `element`: copy of array element
* `index` _(optional)_: current index in the array

#### Example

```rhai
fn process(r, x) {
x + (r ?? 0)
}
fn process_extra(r, x, i) {
x + i + (r ?? 0)
}

let x = [1, 2, 3, 4, 5];

let y = x.reduce("process");

print(y);       // prints 15

let y = x.reduce("process_extra");

print(y);       // prints 25
```

</div>
</div>




<div class='doc-block'>

## reduce_rev

<div class='doc-content'>

```rust,ignore
reduce_rev(array: &mut Array, reducer: &str) -> Result<Dynamic, Box<EvalAltResult>>
reduce_rev(array: &mut Array, reducer: FnPtr) -> Result<Dynamic, Box<EvalAltResult>>
reduce_rev(array: &mut Array, reducer: &str, initial: Dynamic) -> Result<Dynamic, Box<EvalAltResult>>
reduce_rev(array: &mut Array, reducer: FnPtr, initial: Dynamic) -> Result<Dynamic, Box<EvalAltResult>>
```

Reduce an array by iterating through all elements, in _reverse_ order,
while applying a function named by `reducer`.

#### Deprecated API

This method is deprecated and will be removed from the next major version.
Use `array.reduce_rev(Fn("fn_name"))` instead.

#### Function Parameters

A function with the same name as the value of `reducer` must exist taking these parameters:

* `result`: accumulated result, initially `()`
* `element`: copy of array element
* `index` _(optional)_: current index in the array

#### Example

```rhai
fn process(r, x) {
x + (r ?? 0)
}
fn process_extra(r, x, i) {
x + i + (r ?? 0)
}

let x = [1, 2, 3, 4, 5];

let y = x.reduce_rev("process");

print(y);       // prints 15

let y = x.reduce_rev("process_extra");

print(y);       // prints 25
```

</div>
</div>




<div class='doc-block'>

## remove

<div class='doc-content'>

```rust,ignore
remove(string: &mut ImmutableString, sub_string: &str)
remove(string: &mut ImmutableString, character: char)
remove(map: &mut Map, property: &str) -> Dynamic
remove(blob: &mut Blob, index: i64) -> i64
remove(array: &mut Array, index: i64) -> Dynamic
```

Remove all occurrences of a sub-string from the string.

#### Example

```rhai
let text = "hello, world! hello, foobar!";

text.remove("hello");

print(text);        // prints ", world! , foobar!"
```

</div>
</div>




<div class='doc-block'>

## replace

<div class='doc-content'>

```rust,ignore
replace(string: &mut ImmutableString, find_character: char, substitute_string: &str)
replace(string: &mut ImmutableString, find_character: char, substitute_character: char)
replace(string: &mut ImmutableString, find_string: &str, substitute_character: char)
replace(string: &mut ImmutableString, find_string: &str, substitute_string: &str)
```

Replace all occurrences of the specified character in the string with another string.

#### Example

```rhai
let text = "hello, world! hello, foobar!";

text.replace('l', "(^)");

print(text);        // prints "he(^)(^)o, wor(^)d! he(^)(^)o, foobar!"
```

</div>
</div>




<div class='doc-block'>

## retain

<div class='doc-content'>

```rust,ignore
retain(array: &mut Array, filter: &str) -> Result<Array, Box<EvalAltResult>>
retain(array: &mut Array, range: Range<i64>) -> Array
retain(blob: &mut Blob, range: RangeInclusive<i64>) -> Blob
retain(map: &mut Map, filter: FnPtr) -> Result<Map, Box<EvalAltResult>>
retain(array: &mut Array, range: RangeInclusive<i64>) -> Array
retain(array: &mut Array, filter: FnPtr) -> Result<Array, Box<EvalAltResult>>
retain(blob: &mut Blob, range: Range<i64>) -> Blob
retain(array: &mut Array, start: i64, len: i64) -> Array
retain(blob: &mut Blob, start: i64, len: i64) -> Blob
```

Remove all elements in the array that do not return `true` when applied a function named by
`filter` and return them as a new array.

#### Deprecated API

This method is deprecated and will be removed from the next major version.
Use `array.retain(Fn("fn_name"))` instead.

#### Function Parameters

A function with the same name as the value of `filter` must exist taking these parameters:

* `element`: copy of array element
* `index` _(optional)_: current index in the array

#### Example

```rhai
fn large(x) { x >= 3 }

fn screen(x, i) { x + i <= 5 }

let x = [1, 2, 3, 4, 5];

let y = x.retain("large");

print(x);       // prints "[3, 4, 5]"

print(y);       // prints "[1, 2]"

let z = x.retain("screen");

print(x);       // prints "[3, 4]"

print(z);       // prints "[5]"
```

</div>
</div>




<div class='doc-block'>

## reverse

<div class='doc-content'>

```rust,ignore
reverse(blob: &mut Blob)
reverse(array: &mut Array)
```

Reverse the BLOB.

#### Example

```rhai
let b = blob();

b += 1; b += 2; b += 3; b += 4; b += 5;

print(b);           // prints "[0102030405]"

b.reverse();

print(b);           // prints "[0504030201]"
```

</div>
</div>




<div class='doc-block'>

## round

<div class='doc-content'>

```rust,ignore
round(x: f64) -> f64
```

Return the nearest whole number closest to the floating-point number.
Rounds away from zero.

</div>
</div>




<div class='doc-block'>

## set

<div class='doc-content'>

```rust,ignore
set(map: &mut Map, property: &str, value: Dynamic)
set(string: &mut ImmutableString, index: i64, character: char)
set(array: &mut Array, index: i64, value: Dynamic)
set(blob: &mut Blob, index: i64, value: i64)
```

Set the value of the `property` in the object map to a new `value`.

If `property` does not exist in the object map, it is added.

#### Example

```rhai
let m = #{a: 1, b: 2, c: 3};

m.set("b", 42)'

print(m);           // prints "#{a: 1, b: 42, c: 3}"

x.set("x", 0);

print(m);           // prints "#{a: 1, b: 42, c: 3, x: 0}"
```

</div>
</div>




<div class='doc-block'>

## set_bit

<div class='doc-content'>

```rust,ignore
set_bit(value: &mut i64, bit: i64, new_value: bool) -> Result<(), Box<EvalAltResult>>
```

Set the specified `bit` in the number if the new value is `true`.
Clear the `bit` if the new value is `false`.

If `bit` < 0, position counts from the MSB (Most Significant Bit).

#### Example

```rhai
let x = 123456;

x.set_bit(5, true);

print(x);               // prints 123488

x.set_bit(6, false);

print(x);               // prints 123424

x.set_bit(-48, false);

print(x);               // prints 57888 on 64-bit
```

</div>
</div>




<div class='doc-block'>

## set_bits

<div class='doc-content'>

```rust,ignore
set_bits(value: &mut i64, range: RangeInclusive<i64>, new_value: i64) -> Result<(), Box<EvalAltResult>>
set_bits(value: &mut i64, range: Range<i64>, new_value: i64) -> Result<(), Box<EvalAltResult>>
set_bits(value: &mut i64, bit: i64, bits: i64, new_value: i64) -> Result<(), Box<EvalAltResult>>
```

Replace an inclusive range of bits in the number with a new value.

#### Example

```rhai
let x = 123456;

x.set_bits(5..=9, 42);

print(x);           // print 123200
```

</div>
</div>




<div class='doc-block'>

## set_tag

<div class='doc-content'>

```rust,ignore
set_tag(value: &mut Dynamic, tag: i64) -> Result<(), Box<EvalAltResult>>
```

Set the _tag_ of a `Dynamic` value.

#### Example

```rhai
let x = "hello, world!";

x.tag = 42;

print(x.tag);           // prints 42
```

</div>
</div>




<div class='doc-block'>

## shift

<div class='doc-content'>

```rust,ignore
shift(blob: &mut Blob) -> i64
shift(array: &mut Array) -> Dynamic
```

Remove the first byte from the BLOB and return it.

If the BLOB is empty, zero is returned.

#### Example

```rhai
let b = blob();

b += 1; b += 2; b += 3; b += 4; b += 5;

print(b.shift());       // prints 1

print(b);               // prints "[02030405]"
```

</div>
</div>




<div class='doc-block'>

## sign

<div class='doc-content'>

```rust,ignore
sign(x: i16) -> i64
sign(x: i128) -> i64
sign(x: i32) -> i64
sign(x: i64) -> i64
sign(x: f64) -> Result<i64, Box<EvalAltResult>>
sign(x: i8) -> i64
sign(x: f32) -> Result<i64, Box<EvalAltResult>>
```

Return the sign (as an integer) of the number according to the following:

* `0` if the number is zero
* `1` if the number is positive
* `-1` if the number is negative

</div>
</div>




<div class='doc-block'>

## sin

<div class='doc-content'>

```rust,ignore
sin(x: f64) -> f64
```

Return the sine of the floating-point number in radians.

</div>
</div>




<div class='doc-block'>

## sinh

<div class='doc-content'>

```rust,ignore
sinh(x: f64) -> f64
```

Return the hyperbolic sine of the floating-point number in radians.

</div>
</div>




<div class='doc-block'>

## sleep

<div class='doc-content'>

```rust,ignore
sleep(seconds: f64)
sleep(seconds: i64)
```

Block the current thread for a particular number of `seconds`.

#### Example

```rhai
// Do nothing for 10 seconds!
sleep(10.0);
```

</div>
</div>




<div class='doc-block'>

## some

<div class='doc-content'>

```rust,ignore
some(array: &mut Array, filter: &str) -> Result<bool, Box<EvalAltResult>>
some(array: &mut Array, filter: FnPtr) -> Result<bool, Box<EvalAltResult>>
```

Return `true` if any element in the array that returns `true` when applied a function named
by `filter`.

#### Deprecated API

This method is deprecated and will be removed from the next major version.
Use `array.some(Fn("fn_name"))` instead.

#### Function Parameters

A function with the same name as the value of `filter` must exist taking these parameters:

* `element`: copy of array element
* `index` _(optional)_: current index in the array

#### Example

```rhai
fn large(x) { x > 3 }

fn huge(x) { x > 10 }

fn screen(x, i) { i > x }

let x = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 5];

print(x.some("large"));     // prints true

print(x.some("huge"));      // prints false

print(x.some("screen"));    // prints true
```

</div>
</div>




<div class='doc-block'>

## sort

<div class='doc-content'>

```rust,ignore
sort(array: &mut Array) -> Result<(), Box<EvalAltResult>>
sort(array: &mut Array, comparer: FnPtr)
sort(array: &mut Array, comparer: &str) -> Result<(), Box<EvalAltResult>>
```

Sort the array.

All elements in the array must be of the same data type.

#### Supported Data Types

* integer numbers
* floating-point numbers
* decimal numbers
* characters
* strings
* booleans
* `()`

#### Example

```rhai
let x = [1, 3, 5, 7, 9, 2, 4, 6, 8, 10];

x.sort();

print(x);       // prints "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]"
```

</div>
</div>




<div class='doc-block'>

## splice

<div class='doc-content'>

```rust,ignore
splice(blob: &mut Blob, range: Range<i64>, replace: Blob)
splice(array: &mut Array, range: Range<i64>, replace: Array)
splice(blob: &mut Blob, range: RangeInclusive<i64>, replace: Blob)
splice(array: &mut Array, range: RangeInclusive<i64>, replace: Array)
splice(blob: &mut Blob, start: i64, len: i64, replace: Blob)
splice(array: &mut Array, start: i64, len: i64, replace: Array)
```

Replace an exclusive `range` of the BLOB with another BLOB.

#### Example

```rhai
let b1 = blob(10, 0x42);
let b2 = blob(5, 0x18);

b1.splice(1..4, b2);

print(b1);      // prints "[4218181818184242 42424242]"
```

</div>
</div>




<div class='doc-block'>

## split

<div class='doc-content'>

```rust,ignore
split(string: &str) -> Array
split(array: &mut Array, index: i64) -> Array
split(string: &mut ImmutableString, index: i64) -> Array
split(string: &str, delimiter: char) -> Array
split(blob: &mut Blob, index: i64) -> Blob
split(string: &str, delimiter: &str) -> Array
split(string: &str, delimiter: char, segments: i64) -> Array
split(string: &str, delimiter: &str, segments: i64) -> Array
```

Split the string into segments based on whitespaces, returning an array of the segments.

#### Example

```rhai
let text = "hello, world! hello, foo!";

print(text.split());        // prints ["hello,", "world!", "hello,", "foo!"]
```

</div>
</div>




<div class='doc-block'>

## split_rev

<div class='doc-content'>

```rust,ignore
split_rev(string: &str, delimiter: &str) -> Array
split_rev(string: &str, delimiter: char) -> Array
split_rev(string: &str, delimiter: char, segments: i64) -> Array
split_rev(string: &str, delimiter: &str, segments: i64) -> Array
```

Split the string into segments based on a `delimiter` string, returning an array of the
segments in _reverse_ order.

#### Example

```rhai
let text = "hello, world! hello, foo!";

print(text.split_rev("ll"));    // prints ["o, foo!", "o, world! he", "he"]
```

</div>
</div>




<div class='doc-block'>

## sqrt

<div class='doc-content'>

```rust,ignore
sqrt(x: f64) -> f64
```

Return the square root of the floating-point number.

</div>
</div>




<div class='doc-block'>

## start

<div class='doc-content'>

```rust,ignore
start(range: &mut RangeInclusive<i64>) -> i64
start(range: &mut Range<i64>) -> i64
```

Return the start of the inclusive range.

</div>
</div>




<div class='doc-block'>

## starts_with

<div class='doc-content'>

```rust,ignore
starts_with(string: &str, match_string: &str) -> bool
```

Return `true` if the string starts with a specified string.

#### Example

```rhai
let text = "hello, world!";

print(text.starts_with("hello"));   // prints true

print(text.starts_with("world"));   // prints false
```

</div>
</div>




<div class='doc-block'>

## sub_string

<div class='doc-content'>

```rust,ignore
sub_string(string: &str, start: i64) -> ImmutableString
sub_string(string: &str, range: RangeInclusive<i64>) -> ImmutableString
sub_string(string: &str, range: Range<i64>) -> ImmutableString
sub_string(string: &str, start: i64, len: i64) -> ImmutableString
```

Copy a portion of the string beginning at the `start` position till the end and return it as
a new string.

* If `start` < 0, position counts from the end of the string (`-1` is the last character).
* If `start` < -length of string, the entire string is copied and returned.
* If `start` ≥ length of string, an empty string is returned.

#### Example

```rhai
let text = "hello, world!";

print(text.sub_string(5));      // prints ", world!"

print(text.sub_string(-5));      // prints "orld!"
```

</div>
</div>




<div class='doc-block'>

## tag

<div class='doc-content'>

```rust,ignore
tag(value: &mut Dynamic) -> i64
```

Return the _tag_ of a `Dynamic` value.

#### Example

```rhai
let x = "hello, world!";

x.tag = 42;

print(x.tag);           // prints 42
```

</div>
</div>




<div class='doc-block'>

## take

<div class='doc-content'>

```rust,ignore
take(value: &mut Dynamic) -> Result<Dynamic, Box<EvalAltResult>>
```

Take ownership of the data in a `Dynamic` value and return it.
The data is _NOT_ cloned.

The original value is replaced with `()`.

#### Example

```rhai
let x = 42;

print(take(x));         // prints 42

print(x);               // prints ()
```

</div>
</div>




<div class='doc-block'>

## tan

<div class='doc-content'>

```rust,ignore
tan(x: f64) -> f64
```

Return the tangent of the floating-point number in radians.

</div>
</div>




<div class='doc-block'>

## tanh

<div class='doc-content'>

```rust,ignore
tanh(x: f64) -> f64
```

Return the hyperbolic tangent of the floating-point number in radians.

</div>
</div>




<div class='doc-block'>

## timestamp

<div class='doc-content'>

```rust,ignore
timestamp() -> Instant
```

Create a timestamp containing the current system time.

#### Example

```rhai
let now = timestamp();

sleep(10.0);            // sleep for 10 seconds

print(now.elapsed);     // prints 10.???
```

</div>
</div>




<div class='doc-block'>

## to_array

<div class='doc-content'>

```rust,ignore
to_array(blob: &mut Blob) -> Array
```

Convert the BLOB into an array of integers.

#### Example

```rhai
let b = blob(5, 0x42);

let x = b.to_array();

print(x);       // prints "[66, 66, 66, 66, 66]"
```

</div>
</div>




<div class='doc-block'>

## to_binary

<div class='doc-content'>

```rust,ignore
to_binary(value: i8) -> ImmutableString
to_binary(value: u8) -> ImmutableString
to_binary(value: u128) -> ImmutableString
to_binary(value: u32) -> ImmutableString
to_binary(value: i64) -> ImmutableString
to_binary(value: u64) -> ImmutableString
to_binary(value: i32) -> ImmutableString
to_binary(value: i16) -> ImmutableString
to_binary(value: u16) -> ImmutableString
to_binary(value: i128) -> ImmutableString
```

Convert the `value` into a string in binary format.

</div>
</div>




<div class='doc-block'>

## to_blob

<div class='doc-content'>

```rust,ignore
to_blob(string: &str) -> Blob
```

Convert the string into an UTF-8 encoded byte-stream as a BLOB.

#### Example

```rhai
let text = "朝には紅顔ありて夕べには白骨となる";

let bytes = text.to_blob();

print(bytes.len());     // prints 51
```

</div>
</div>




<div class='doc-block'>

## to_chars

<div class='doc-content'>

```rust,ignore
to_chars(string: &str) -> Array
```

Return an array containing all the characters of the string.

#### Example

```rhai
let text = "hello";

print(text.to_chars());     // prints "['h', 'e', 'l', 'l', 'o']"
```

</div>
</div>




<div class='doc-block'>

## to_debug

<div class='doc-content'>

```rust,ignore
to_debug(number: f32) -> ImmutableString
to_debug(array: &mut Array) -> ImmutableString
to_debug(string: &str) -> ImmutableString
to_debug(map: &mut Map) -> ImmutableString
to_debug(unit: ()) -> ImmutableString
to_debug(number: f64) -> ImmutableString
to_debug(character: char) -> ImmutableString
to_debug(item: &mut Dynamic) -> ImmutableString
to_debug(f: &mut FnPtr) -> ImmutableString
to_debug(value: bool) -> ImmutableString
```

Convert the value of `number` into a string.

</div>
</div>




<div class='doc-block'>

## to_degrees

<div class='doc-content'>

```rust,ignore
to_degrees(x: f64) -> f64
```

Convert radians to degrees.

</div>
</div>




<div class='doc-block'>

## to_float

<div class='doc-content'>

```rust,ignore
to_float(_)
to_float(_)
to_float(_)
to_float(_)
to_float(_)
to_float(_)
to_float(x: f32) -> f64
to_float(_)
to_float(_)
to_float(_)
to_float(_)
to_float(_)
```

Convert the 32-bit floating-point number to 64-bit.

</div>
</div>




<div class='doc-block'>

## to_hex

<div class='doc-content'>

```rust,ignore
to_hex(value: i32) -> ImmutableString
to_hex(value: i128) -> ImmutableString
to_hex(value: u16) -> ImmutableString
to_hex(value: i16) -> ImmutableString
to_hex(value: u32) -> ImmutableString
to_hex(value: u8) -> ImmutableString
to_hex(value: u128) -> ImmutableString
to_hex(value: i8) -> ImmutableString
to_hex(value: u64) -> ImmutableString
to_hex(value: i64) -> ImmutableString
```

Convert the `value` into a string in hex format.

</div>
</div>




<div class='doc-block'>

## to_int

<div class='doc-content'>

```rust,ignore
to_int(_)
to_int(x: f64) -> Result<i64, Box<EvalAltResult>>
to_int(_)
to_int(_)
to_int(_)
to_int(_)
to_int(x: f32) -> Result<i64, Box<EvalAltResult>>
to_int(_)
to_int(_)
to_int(_)
to_int(_)
to_int(_)
to_int(_)
```

Convert the floating-point number into an integer.

</div>
</div>




<div class='doc-block'>

## to_json

<div class='doc-content'>

```rust,ignore
to_json(map: &mut Map) -> String
```

Return the JSON representation of the object map.

#### Data types

Only the following data types should be kept inside the object map:
`INT`, `FLOAT`, `ImmutableString`, `char`, `bool`, `()`, `Array`, `Map`.

#### Errors

Data types not supported by JSON serialize into formats that may
invalidate the result.

#### Example

```rhai
let m = #{a:1, b:2, c:3};

print(m.to_json());     // prints {"a":1, "b":2, "c":3}
```

</div>
</div>




<div class='doc-block'>

## to_lower

<div class='doc-content'>

```rust,ignore
to_lower(character: char) -> char
to_lower(string: ImmutableString) -> ImmutableString
```

Convert the character to lower-case and return it as a new character.

#### Example

```rhai
let ch = 'A';

print(ch.to_lower());       // prints 'a'

print(ch);                  // prints 'A'
```

</div>
</div>




<div class='doc-block'>

## to_octal

<div class='doc-content'>

```rust,ignore
to_octal(value: i8) -> ImmutableString
to_octal(value: u128) -> ImmutableString
to_octal(value: u8) -> ImmutableString
to_octal(value: u32) -> ImmutableString
to_octal(value: i64) -> ImmutableString
to_octal(value: u64) -> ImmutableString
to_octal(value: i32) -> ImmutableString
to_octal(value: i16) -> ImmutableString
to_octal(value: u16) -> ImmutableString
to_octal(value: i128) -> ImmutableString
```

Convert the `value` into a string in octal format.

</div>
</div>




<div class='doc-block'>

## to_radians

<div class='doc-content'>

```rust,ignore
to_radians(x: f64) -> f64
```

Convert degrees to radians.

</div>
</div>




<div class='doc-block'>

## to_string

<div class='doc-content'>

```rust,ignore
to_string(unit: ()) -> ImmutableString
to_string(number: f64) -> ImmutableString
to_string(number: f32) -> ImmutableString
to_string(string: ImmutableString) -> ImmutableString
to_string(array: &mut Array) -> ImmutableString
to_string(map: &mut Map) -> ImmutableString
to_string(value: bool) -> ImmutableString
to_string(character: char) -> ImmutableString
to_string(item: &mut Dynamic) -> ImmutableString
```

Return the empty string.

</div>
</div>




<div class='doc-block'>

## to_upper

<div class='doc-content'>

```rust,ignore
to_upper(string: ImmutableString) -> ImmutableString
to_upper(character: char) -> char
```

Convert the string to all upper-case and return it as a new string.

#### Example

```rhai
let text = "hello, world!"

print(text.to_upper());     // prints "HELLO, WORLD!"

print(text);                // prints "hello, world!"
```

</div>
</div>




<div class='doc-block'>

## trim

<div class='doc-content'>

```rust,ignore
trim(string: &mut ImmutableString)
```

Remove whitespace characters from both ends of the string.

#### Example

```rhai
let text = "   hello     ";

text.trim();

print(text);    // prints "hello"
```

</div>
</div>




<div class='doc-block'>

## truncate

<div class='doc-content'>

```rust,ignore
truncate(blob: &mut Blob, len: i64)
truncate(string: &mut ImmutableString, len: i64)
truncate(array: &mut Array, len: i64)
```

Cut off the BLOB at the specified length.

* If `len` ≤ 0, the BLOB is cleared.
* If `len` ≥ length of BLOB, the BLOB is not truncated.

#### Example

```rhai
let b = blob();

b += 1; b += 2; b += 3; b += 4; b += 5;

b.truncate(3);

print(b);           // prints "[010203]"

b.truncate(10);

print(b);           // prints "[010203]"
```

</div>
</div>




<div class='doc-block'>

## values

<div class='doc-content'>

```rust,ignore
values(map: &mut Map) -> Array
```

Return an array with all the property values in the object map.

#### Example

```rhai
let m = #{a:1, b:2, c:3};

print(m.values());      // prints "[1, 2, 3]""
```

</div>
</div>




<div class='doc-block'>

## write_ascii

<div class='doc-content'>

```rust,ignore
write_ascii(blob: &mut Blob, range: Range<i64>, string: &str)
write_ascii(blob: &mut Blob, range: RangeInclusive<i64>, string: &str)
write_ascii(blob: &mut Blob, start: i64, len: i64, string: &str)
```

Write an ASCII string to the bytes within an exclusive `range` in the BLOB.

Each ASCII character encodes to one single byte in the BLOB.
Non-ASCII characters are ignored.

* If number of bytes in `range` < length of `string`, extra bytes in `string` are not written.
* If number of bytes in `range` > length of `string`, extra bytes in `range` are not modified.

```rhai
let b = blob(8);

b.write_ascii(1..5, "hello, world!");

print(b);       // prints "[0068656c6c000000]"
```

</div>
</div>




<div class='doc-block'>

## write_be

<div class='doc-content'>

```rust,ignore
write_be(blob: &mut Blob, range: RangeInclusive<i64>, value: i64)
write_be(blob: &mut Blob, range: RangeInclusive<i64>, value: f64)
write_be(blob: &mut Blob, range: Range<i64>, value: f64)
write_be(blob: &mut Blob, range: Range<i64>, value: i64)
write_be(blob: &mut Blob, start: i64, len: i64, value: i64)
write_be(blob: &mut Blob, start: i64, len: i64, value: f64)
```

Write an `INT` value to the bytes within an inclusive `range` in the BLOB
in big-endian byte order.

* If number of bytes in `range` < number of bytes for `INT`, extra bytes in `INT` are not written.
* If number of bytes in `range` > number of bytes for `INT`, extra bytes in `range` are not modified.

```rhai
let b = blob(8, 0x42);

b.write_be_int(1..=3, 0x99);

print(b);       // prints "[4200000042424242]"
```

</div>
</div>




<div class='doc-block'>

## write_le

<div class='doc-content'>

```rust,ignore
write_le(blob: &mut Blob, range: Range<i64>, value: i64)
write_le(blob: &mut Blob, range: Range<i64>, value: f64)
write_le(blob: &mut Blob, range: RangeInclusive<i64>, value: i64)
write_le(blob: &mut Blob, range: RangeInclusive<i64>, value: f64)
write_le(blob: &mut Blob, start: i64, len: i64, value: i64)
write_le(blob: &mut Blob, start: i64, len: i64, value: f64)
```

Write an `INT` value to the bytes within an exclusive `range` in the BLOB
in little-endian byte order.

* If number of bytes in `range` < number of bytes for `INT`, extra bytes in `INT` are not written.
* If number of bytes in `range` > number of bytes for `INT`, extra bytes in `range` are not modified.

```rhai
let b = blob(8);

b.write_le_int(1..3, 0x12345678);

print(b);       // prints "[0078560000000000]"
```

</div>
</div>




<div class='doc-block'>

## write_utf8

<div class='doc-content'>

```rust,ignore
write_utf8(blob: &mut Blob, range: Range<i64>, string: &str)
write_utf8(blob: &mut Blob, range: RangeInclusive<i64>, string: &str)
write_utf8(blob: &mut Blob, start: i64, len: i64, string: &str)
```

Write a string to the bytes within an exclusive `range` in the BLOB in UTF-8 encoding.

* If number of bytes in `range` < length of `string`, extra bytes in `string` are not written.
* If number of bytes in `range` > length of `string`, extra bytes in `range` are not modified.

```rhai
let b = blob(8);

b.write_utf8(1..5, "朝には紅顔ありて夕べには白骨となる");

print(b);       // prints "[00e69c9de3000000]"
```

</div>
</div>




<div class='doc-block'>

## zip

<div class='doc-content'>

```rust,ignore
zip(array1: &mut Array, array2: Array, map: FnPtr) -> Result<Array, Box<EvalAltResult>>
```

Iterate through all elements in two arrays, applying a `mapper` function to them,
and return a new array containing the results.

#### Function Parameters

* `array1`: First array
* `array2`: Second array
* `index` _(optional)_: current index in the array

#### Example

```rhai
let x = [1, 2, 3, 4, 5];
let y = [9, 8, 7, 6];

let z = x.zip(y, |a, b| a + b);

print(z);       // prints [10, 10, 10, 10]

let z = x.zip(y, |a, b, i| a + b + i);

print(z);       // prints [10, 11, 12, 13]
```

</div>
</div>




</div>