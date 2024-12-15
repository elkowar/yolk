# global

```Namespace: global```

<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>op</code> !&#x3D; </h2>

```rust,ignore
op f32 != int -> bool
op Array != Array -> bool
op u16 != u16 -> bool
op f32 != f32 -> bool
op u128 != u128 -> bool
op i128 != i128 -> bool
op u32 != u32 -> bool
op i8 != i8 -> bool
op i16 != i16 -> bool
op i32 != i32 -> bool
op int != f32 -> bool
op u64 != u64 -> bool
op u8 != u8 -> bool
op Map != Map -> bool
op Instant != Instant -> bool
```

<div>
<div class="tab">
<button group="!&#x3D;" id="link-!&#x3D;-Description"  class="tablinks active" 
    onclick="openTab(event, '!&#x3D;', 'Description')">
Description
</button>
<button group="!&#x3D;" id="link-!&#x3D;-Example"  class="tablinks" 
    onclick="openTab(event, '!&#x3D;', 'Example')">
Example
</button>
</div>

<div group="!&#x3D;" id="!&#x3D;-Description" class="tabcontent"  style="display: block;" >
Return `true` if two arrays are not-equal (i.e. any element not equal or not in the same order).

The operator `==` is used to compare elements and must be defined,
otherwise `false` is assumed.
</div>
<div group="!&#x3D;" id="!&#x3D;-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = [1, 2, 3, 4, 5];
let y = [1, 2, 3, 4, 5];
let z = [1, 2, 3, 4];

print(x != y);      // prints false

print(x != z);      // prints true
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> + </h2>

```rust,ignore
fn +(x: i16) -> i16
fn +(x: i8) -> i8
fn +(x: i32) -> i32
fn +(x: i128) -> i128
fn +(x: f32) -> f32
fn +(x: float) -> float
fn +(x: int) -> int
fn +(x: i8, y: i8) -> i8
fn +(x: i16, y: i16) -> i16
fn +(character: char, string: String) -> String
fn +(x: i128, y: i128) -> i128
fn +(x: u32, y: u32) -> u32
fn +(x: u16, y: u16) -> u16
fn +(x: f32, y: f32) -> f32
fn +(item: ?, string: String) -> String
fn +(string1: String, string2: String) -> String
fn +(x: u128, y: u128) -> u128
fn +(x: f32, y: int) -> f32
fn +(string: String, character: char) -> String
fn +(item: ?, string: String) -> String
fn +(string: String, utf8: Blob) -> String
fn +(utf8: Blob, string: String) -> String
fn +(array1: Array, array2: Array) -> Array
fn +(string: String, mut item: ?) -> String
fn +(timestamp: Instant, seconds: float) -> Instant
fn +(map1: Map, map2: Map) -> Map
fn +(x: u8, y: u8) -> u8
fn +(timestamp: Instant, seconds: int) -> Instant
fn +(x: i32, y: i32) -> i32
fn +(x: int, y: f32) -> f32
fn +(string: String, item: ?) -> String
fn +(x: u64, y: u64) -> u64
```

<div>
<div class="tab">
<button group="+" id="link-+-Description"  class="tablinks active" 
    onclick="openTab(event, '+', 'Description')">
Description
</button>
<button group="+" id="link-+-Example"  class="tablinks" 
    onclick="openTab(event, '+', 'Example')">
Example
</button>
</div>

<div group="+" id="+-Description" class="tabcontent"  style="display: block;" >
Combine two arrays into a new array and return it.
</div>
<div group="+" id="+-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = [1, 2, 3];
let y = [true, 'x'];

print(x + y);   // prints "[1, 2, 3, true, 'x']"

print(x);       // prints "[1, 2, 3"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> +&#x3D; </h2>

```rust,ignore
fn +=(string: String, item: ?)
fn +=(timestamp: Instant, seconds: int)
fn +=(map: Map, map2: Map)
fn +=(string: String, mut item: ?)
fn +=(timestamp: Instant, seconds: float)
fn +=(string: String, character: char)
fn +=(string: String, utf8: Blob)
fn +=(string1: String, string2: String)
```

<div>
<div class="tab">
<button group="+&#x3D;" id="link-+&#x3D;-Description"  class="tablinks active" 
    onclick="openTab(event, '+&#x3D;', 'Description')">
Description
</button>
</div>

<div group="+&#x3D;" id="+&#x3D;-Description" class="tabcontent"  style="display: block;" >
Add the specified number of `seconds` to the timestamp.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> - </h2>

```rust,ignore
fn -(x: i8) -> i8
fn -(x: i16) -> i16
fn -(x: int) -> int
fn -(x: float) -> float
fn -(x: i128) -> i128
fn -(x: f32) -> f32
fn -(x: i32) -> i32
fn -(x: u8, y: u8) -> u8
fn -(timestamp: Instant, seconds: int) -> Instant
fn -(x: i32, y: i32) -> i32
fn -(x: int, y: f32) -> f32
fn -(x: u64, y: u64) -> u64
fn -(timestamp: Instant, seconds: float) -> Instant
fn -(timestamp1: Instant, timestamp2: Instant) -> ?
fn -(x: u16, y: u16) -> u16
fn -(x: f32, y: f32) -> f32
fn -(x: u128, y: u128) -> u128
fn -(x: f32, y: int) -> f32
fn -(x: i8, y: i8) -> i8
fn -(x: i16, y: i16) -> i16
fn -(x: i128, y: i128) -> i128
fn -(x: u32, y: u32) -> u32
```

<div>
<div class="tab">
<button group="-" id="link---Description"  class="tablinks active" 
    onclick="openTab(event, '-', 'Description')">
Description
</button>
</div>

<div group="-" id="--Description" class="tabcontent"  style="display: block;" >
Subtract the specified number of `seconds` from the timestamp and return it as a new timestamp.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> -&#x3D; </h2>

```rust,ignore
fn -=(timestamp: Instant, seconds: int)
fn -=(timestamp: Instant, seconds: float)
```

<div>
<div class="tab">
<button group="-&#x3D;" id="link--&#x3D;-Description"  class="tablinks active" 
    onclick="openTab(event, '-&#x3D;', 'Description')">
Description
</button>
</div>

<div group="-&#x3D;" id="-&#x3D;-Description" class="tabcontent"  style="display: block;" >
Subtract the specified number of `seconds` from the timestamp.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>op</code> &lt; </h2>

```rust,ignore
op u8 < u8 -> bool
op i32 < i32 -> bool
op int < f32 -> bool
op u64 < u64 -> bool
op Instant < Instant -> bool
op u128 < u128 -> bool
op u16 < u16 -> bool
op f32 < f32 -> bool
op f32 < int -> bool
op i16 < i16 -> bool
op i8 < i8 -> bool
op u32 < u32 -> bool
op i128 < i128 -> bool
```

<div>
<div class="tab">
<button group="&lt;" id="link-&lt;-Description"  class="tablinks active" 
    onclick="openTab(event, '&lt;', 'Description')">
Description
</button>
</div>

<div group="&lt;" id="&lt;-Description" class="tabcontent"  style="display: block;" >
Return `true` if the first timestamp is earlier than the second.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>op</code> &lt;&#x3D; </h2>

```rust,ignore
op Instant <= Instant -> bool
op u64 <= u64 -> bool
op i32 <= i32 -> bool
op int <= f32 -> bool
op u8 <= u8 -> bool
op i128 <= i128 -> bool
op u32 <= u32 -> bool
op i8 <= i8 -> bool
op i16 <= i16 -> bool
op f32 <= int -> bool
op u16 <= u16 -> bool
op f32 <= f32 -> bool
op u128 <= u128 -> bool
```

<div>
<div class="tab">
<button group="&lt;&#x3D;" id="link-&lt;&#x3D;-Description"  class="tablinks active" 
    onclick="openTab(event, '&lt;&#x3D;', 'Description')">
Description
</button>
</div>

<div group="&lt;&#x3D;" id="&lt;&#x3D;-Description" class="tabcontent"  style="display: block;" >
Return `true` if the first timestamp is earlier than or equals to the second.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>op</code> &#x3D;&#x3D; </h2>

```rust,ignore
op i128 == i128 -> bool
op u32 == u32 -> bool
op i8 == i8 -> bool
op i16 == i16 -> bool
op f32 == int -> bool
op Array == Array -> bool
op u16 == u16 -> bool
op f32 == f32 -> bool
op u128 == u128 -> bool
op Map == Map -> bool
op Instant == Instant -> bool
op u64 == u64 -> bool
op i32 == i32 -> bool
op int == f32 -> bool
op u8 == u8 -> bool
```

<div>
<div class="tab">
<button group="&#x3D;&#x3D;" id="link-&#x3D;&#x3D;-Description"  class="tablinks active" 
    onclick="openTab(event, '&#x3D;&#x3D;', 'Description')">
Description
</button>
<button group="&#x3D;&#x3D;" id="link-&#x3D;&#x3D;-Example"  class="tablinks" 
    onclick="openTab(event, '&#x3D;&#x3D;', 'Example')">
Example
</button>
</div>

<div group="&#x3D;&#x3D;" id="&#x3D;&#x3D;-Description" class="tabcontent"  style="display: block;" >
Return `true` if two arrays are equal (i.e. all elements are equal and in the same order).

The operator `==` is used to compare elements and must be defined,
otherwise `false` is assumed.
</div>
<div group="&#x3D;&#x3D;" id="&#x3D;&#x3D;-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = [1, 2, 3, 4, 5];
let y = [1, 2, 3, 4, 5];
let z = [1, 2, 3, 4];

print(x == y);      // prints true

print(x == z);      // prints false
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>op</code> &gt; </h2>

```rust,ignore
op u64 > u64 -> bool
op i32 > i32 -> bool
op int > f32 -> bool
op u8 > u8 -> bool
op Instant > Instant -> bool
op f32 > int -> bool
op u16 > u16 -> bool
op f32 > f32 -> bool
op u128 > u128 -> bool
op i128 > i128 -> bool
op u32 > u32 -> bool
op i8 > i8 -> bool
op i16 > i16 -> bool
```

<div>
<div class="tab">
<button group="&gt;" id="link-&gt;-Description"  class="tablinks active" 
    onclick="openTab(event, '&gt;', 'Description')">
Description
</button>
</div>

<div group="&gt;" id="&gt;-Description" class="tabcontent"  style="display: block;" >
Return `true` if the first timestamp is later than the second.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>op</code> &gt;&#x3D; </h2>

```rust,ignore
op f32 >= int -> bool
op u128 >= u128 -> bool
op u16 >= u16 -> bool
op f32 >= f32 -> bool
op u32 >= u32 -> bool
op i128 >= i128 -> bool
op i16 >= i16 -> bool
op i8 >= i8 -> bool
op u64 >= u64 -> bool
op i32 >= i32 -> bool
op int >= f32 -> bool
op u8 >= u8 -> bool
op Instant >= Instant -> bool
```

<div>
<div class="tab">
<button group="&gt;&#x3D;" id="link-&gt;&#x3D;-Description"  class="tablinks active" 
    onclick="openTab(event, '&gt;&#x3D;', 'Description')">
Description
</button>
</div>

<div group="&gt;&#x3D;" id="&gt;&#x3D;-Description" class="tabcontent"  style="display: block;" >
Return `true` if the first timestamp is later than or equals to the second.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> ?.tag </h2>

```rust,ignore
get ?.tag -> int
set ?.tag = int
```

<div>
<div class="tab">
<button group="?.tag" id="link-?.tag-Description"  class="tablinks active" 
    onclick="openTab(event, '?.tag', 'Description')">
Description
</button>
<button group="?.tag" id="link-?.tag-Example"  class="tablinks" 
    onclick="openTab(event, '?.tag', 'Example')">
Example
</button>
</div>

<div group="?.tag" id="?.tag-Description" class="tabcontent"  style="display: block;" >
Return the _tag_ of a `Dynamic` value.
</div>
<div group="?.tag" id="?.tag-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = "hello, world!";

x.tag = 42;

print(x.tag);           // prints 42
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> Array.is_empty </h2>

```rust,ignore
get Array.is_empty -> bool
```

<div>
<div class="tab">
<button group="Array.is_empty" id="link-Array.is_empty-Description"  class="tablinks active" 
    onclick="openTab(event, 'Array.is_empty', 'Description')">
Description
</button>
</div>

<div group="Array.is_empty" id="Array.is_empty-Description" class="tabcontent"  style="display: block;" >
Return true if the array is empty.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> Array.len </h2>

```rust,ignore
get Array.len -> int
```

<div>
<div class="tab">
<button group="Array.len" id="link-Array.len-Description"  class="tablinks active" 
    onclick="openTab(event, 'Array.len', 'Description')">
Description
</button>
</div>

<div group="Array.len" id="Array.len-Description" class="tabcontent"  style="display: block;" >
Number of elements in the array.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> Blob.is_empty </h2>

```rust,ignore
get Blob.is_empty -> bool
```

<div>
<div class="tab">
<button group="Blob.is_empty" id="link-Blob.is_empty-Description"  class="tablinks active" 
    onclick="openTab(event, 'Blob.is_empty', 'Description')">
Description
</button>
</div>

<div group="Blob.is_empty" id="Blob.is_empty-Description" class="tabcontent"  style="display: block;" >
Return true if the BLOB is empty.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> Blob.len </h2>

```rust,ignore
get Blob.len -> int
```

<div>
<div class="tab">
<button group="Blob.len" id="link-Blob.len-Description"  class="tablinks active" 
    onclick="openTab(event, 'Blob.len', 'Description')">
Description
</button>
<button group="Blob.len" id="link-Blob.len-Example"  class="tablinks" 
    onclick="openTab(event, 'Blob.len', 'Example')">
Example
</button>
</div>

<div group="Blob.len" id="Blob.len-Description" class="tabcontent"  style="display: block;" >
Return the length of the BLOB.
</div>
<div group="Blob.len" id="Blob.len-Example" class="tabcontent"  style="display: none;" >

```rhai
let b = blob(10, 0x42);

print(b);           // prints "[4242424242424242 4242]"

print(b.len());     // prints 10
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> E </h2>

```rust,ignore
fn E() -> float
```

<div>
<div class="tab">
<button group="E" id="link-E-Description"  class="tablinks active" 
    onclick="openTab(event, 'E', 'Description')">
Description
</button>
</div>

<div group="E" id="E-Description" class="tabcontent"  style="display: block;" >
Return the natural number _e_.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> FnPtr.is_anonymous </h2>

```rust,ignore
get FnPtr.is_anonymous -> bool
```

<div>
<div class="tab">
<button group="FnPtr.is_anonymous" id="link-FnPtr.is_anonymous-Description"  class="tablinks active" 
    onclick="openTab(event, 'FnPtr.is_anonymous', 'Description')">
Description
</button>
<button group="FnPtr.is_anonymous" id="link-FnPtr.is_anonymous-Example"  class="tablinks" 
    onclick="openTab(event, 'FnPtr.is_anonymous', 'Example')">
Example
</button>
</div>

<div group="FnPtr.is_anonymous" id="FnPtr.is_anonymous-Description" class="tabcontent"  style="display: block;" >
Return `true` if the function is an anonymous function.
</div>
<div group="FnPtr.is_anonymous" id="FnPtr.is_anonymous-Example" class="tabcontent"  style="display: none;" >

```rhai
let f = |x| x * 2;

print(f.is_anonymous);      // prints true
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> FnPtr.name </h2>

```rust,ignore
get FnPtr.name -> String
```

<div>
<div class="tab">
<button group="FnPtr.name" id="link-FnPtr.name-Description"  class="tablinks active" 
    onclick="openTab(event, 'FnPtr.name', 'Description')">
Description
</button>
<button group="FnPtr.name" id="link-FnPtr.name-Example"  class="tablinks" 
    onclick="openTab(event, 'FnPtr.name', 'Example')">
Example
</button>
</div>

<div group="FnPtr.name" id="FnPtr.name-Description" class="tabcontent"  style="display: block;" >
Return the name of the function.
</div>
<div group="FnPtr.name" id="FnPtr.name-Example" class="tabcontent"  style="display: none;" >

```rhai
fn double(x) { x * 2 }

let f = Fn("double");

print(f.name);      // prints "double"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> Instant.elapsed </h2>

```rust,ignore
get Instant.elapsed -> ?
```

<div>
<div class="tab">
<button group="Instant.elapsed" id="link-Instant.elapsed-Description"  class="tablinks active" 
    onclick="openTab(event, 'Instant.elapsed', 'Description')">
Description
</button>
<button group="Instant.elapsed" id="link-Instant.elapsed-Example"  class="tablinks" 
    onclick="openTab(event, 'Instant.elapsed', 'Example')">
Example
</button>
</div>

<div group="Instant.elapsed" id="Instant.elapsed-Description" class="tabcontent"  style="display: block;" >
Return the number of seconds between the current system time and the timestamp.
</div>
<div group="Instant.elapsed" id="Instant.elapsed-Example" class="tabcontent"  style="display: none;" >

```rhai
let now = timestamp();

sleep(10.0);            // sleep for 10 seconds

print(now.elapsed);     // prints 10.???
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> PI </h2>

```rust,ignore
fn PI() -> float
```

<div>
<div class="tab">
<button group="PI" id="link-PI-Description"  class="tablinks active" 
    onclick="openTab(event, 'PI', 'Description')">
Description
</button>
</div>

<div group="PI" id="PI-Description" class="tabcontent"  style="display: block;" >
Return the number π.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> Range&lt;int&gt;.end </h2>

```rust,ignore
get Range<int>.end -> int
```

<div>
<div class="tab">
<button group="Range&lt;int&gt;.end" id="link-Range&lt;int&gt;.end-Description"  class="tablinks active" 
    onclick="openTab(event, 'Range&lt;int&gt;.end', 'Description')">
Description
</button>
</div>

<div group="Range&lt;int&gt;.end" id="Range&lt;int&gt;.end-Description" class="tabcontent"  style="display: block;" >
Return the end of the exclusive range.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> Range&lt;int&gt;.is_empty </h2>

```rust,ignore
get Range<int>.is_empty -> bool
```

<div>
<div class="tab">
<button group="Range&lt;int&gt;.is_empty" id="link-Range&lt;int&gt;.is_empty-Description"  class="tablinks active" 
    onclick="openTab(event, 'Range&lt;int&gt;.is_empty', 'Description')">
Description
</button>
</div>

<div group="Range&lt;int&gt;.is_empty" id="Range&lt;int&gt;.is_empty-Description" class="tabcontent"  style="display: block;" >
Return true if the range contains no items.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> Range&lt;int&gt;.is_exclusive </h2>

```rust,ignore
get Range<int>.is_exclusive -> bool
```

<div>
<div class="tab">
<button group="Range&lt;int&gt;.is_exclusive" id="link-Range&lt;int&gt;.is_exclusive-Description"  class="tablinks active" 
    onclick="openTab(event, 'Range&lt;int&gt;.is_exclusive', 'Description')">
Description
</button>
</div>

<div group="Range&lt;int&gt;.is_exclusive" id="Range&lt;int&gt;.is_exclusive-Description" class="tabcontent"  style="display: block;" >
Return `true` if the range is exclusive.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> Range&lt;int&gt;.is_inclusive </h2>

```rust,ignore
get Range<int>.is_inclusive -> bool
```

<div>
<div class="tab">
<button group="Range&lt;int&gt;.is_inclusive" id="link-Range&lt;int&gt;.is_inclusive-Description"  class="tablinks active" 
    onclick="openTab(event, 'Range&lt;int&gt;.is_inclusive', 'Description')">
Description
</button>
</div>

<div group="Range&lt;int&gt;.is_inclusive" id="Range&lt;int&gt;.is_inclusive-Description" class="tabcontent"  style="display: block;" >
Return `true` if the range is inclusive.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> Range&lt;int&gt;.start </h2>

```rust,ignore
get Range<int>.start -> int
```

<div>
<div class="tab">
<button group="Range&lt;int&gt;.start" id="link-Range&lt;int&gt;.start-Description"  class="tablinks active" 
    onclick="openTab(event, 'Range&lt;int&gt;.start', 'Description')">
Description
</button>
</div>

<div group="Range&lt;int&gt;.start" id="Range&lt;int&gt;.start-Description" class="tabcontent"  style="display: block;" >
Return the start of the exclusive range.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> RangeInclusive&lt;int&gt;.end </h2>

```rust,ignore
get RangeInclusive<int>.end -> int
```

<div>
<div class="tab">
<button group="RangeInclusive&lt;int&gt;.end" id="link-RangeInclusive&lt;int&gt;.end-Description"  class="tablinks active" 
    onclick="openTab(event, 'RangeInclusive&lt;int&gt;.end', 'Description')">
Description
</button>
</div>

<div group="RangeInclusive&lt;int&gt;.end" id="RangeInclusive&lt;int&gt;.end-Description" class="tabcontent"  style="display: block;" >
Return the end of the inclusive range.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> RangeInclusive&lt;int&gt;.is_empty </h2>

```rust,ignore
get RangeInclusive<int>.is_empty -> bool
```

<div>
<div class="tab">
<button group="RangeInclusive&lt;int&gt;.is_empty" id="link-RangeInclusive&lt;int&gt;.is_empty-Description"  class="tablinks active" 
    onclick="openTab(event, 'RangeInclusive&lt;int&gt;.is_empty', 'Description')">
Description
</button>
</div>

<div group="RangeInclusive&lt;int&gt;.is_empty" id="RangeInclusive&lt;int&gt;.is_empty-Description" class="tabcontent"  style="display: block;" >
Return true if the range contains no items.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> RangeInclusive&lt;int&gt;.is_exclusive </h2>

```rust,ignore
get RangeInclusive<int>.is_exclusive -> bool
```

<div>
<div class="tab">
<button group="RangeInclusive&lt;int&gt;.is_exclusive" id="link-RangeInclusive&lt;int&gt;.is_exclusive-Description"  class="tablinks active" 
    onclick="openTab(event, 'RangeInclusive&lt;int&gt;.is_exclusive', 'Description')">
Description
</button>
</div>

<div group="RangeInclusive&lt;int&gt;.is_exclusive" id="RangeInclusive&lt;int&gt;.is_exclusive-Description" class="tabcontent"  style="display: block;" >
Return `true` if the range is exclusive.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> RangeInclusive&lt;int&gt;.is_inclusive </h2>

```rust,ignore
get RangeInclusive<int>.is_inclusive -> bool
```

<div>
<div class="tab">
<button group="RangeInclusive&lt;int&gt;.is_inclusive" id="link-RangeInclusive&lt;int&gt;.is_inclusive-Description"  class="tablinks active" 
    onclick="openTab(event, 'RangeInclusive&lt;int&gt;.is_inclusive', 'Description')">
Description
</button>
</div>

<div group="RangeInclusive&lt;int&gt;.is_inclusive" id="RangeInclusive&lt;int&gt;.is_inclusive-Description" class="tabcontent"  style="display: block;" >
Return `true` if the range is inclusive.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> RangeInclusive&lt;int&gt;.start </h2>

```rust,ignore
get RangeInclusive<int>.start -> int
```

<div>
<div class="tab">
<button group="RangeInclusive&lt;int&gt;.start" id="link-RangeInclusive&lt;int&gt;.start-Description"  class="tablinks active" 
    onclick="openTab(event, 'RangeInclusive&lt;int&gt;.start', 'Description')">
Description
</button>
</div>

<div group="RangeInclusive&lt;int&gt;.start" id="RangeInclusive&lt;int&gt;.start-Description" class="tabcontent"  style="display: block;" >
Return the start of the inclusive range.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> String.bytes </h2>

```rust,ignore
get String.bytes -> int
```

<div>
<div class="tab">
<button group="String.bytes" id="link-String.bytes-Description"  class="tablinks active" 
    onclick="openTab(event, 'String.bytes', 'Description')">
Description
</button>
<button group="String.bytes" id="link-String.bytes-Example"  class="tablinks" 
    onclick="openTab(event, 'String.bytes', 'Example')">
Example
</button>
</div>

<div group="String.bytes" id="String.bytes-Description" class="tabcontent"  style="display: block;" >
Return the length of the string, in number of bytes used to store it in UTF-8 encoding.
</div>
<div group="String.bytes" id="String.bytes-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "朝には紅顔ありて夕べには白骨となる";

print(text.bytes);      // prints 51
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> String.chars </h2>

```rust,ignore
get String.chars -> CharsStream
```

<div>
<div class="tab">
<button group="String.chars" id="link-String.chars-Description"  class="tablinks active" 
    onclick="openTab(event, 'String.chars', 'Description')">
Description
</button>
<button group="String.chars" id="link-String.chars-Example"  class="tablinks" 
    onclick="openTab(event, 'String.chars', 'Example')">
Example
</button>
</div>

<div group="String.chars" id="String.chars-Description" class="tabcontent"  style="display: block;" >
Return an iterator over all the characters in the string.
</div>
<div group="String.chars" id="String.chars-Example" class="tabcontent"  style="display: none;" >

```rhai
for ch in "hello, world!".chars {"
    print(ch);
}
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> String.is_empty </h2>

```rust,ignore
get String.is_empty -> bool
```

<div>
<div class="tab">
<button group="String.is_empty" id="link-String.is_empty-Description"  class="tablinks active" 
    onclick="openTab(event, 'String.is_empty', 'Description')">
Description
</button>
</div>

<div group="String.is_empty" id="String.is_empty-Description" class="tabcontent"  style="display: block;" >
Return true if the string is empty.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> String.len </h2>

```rust,ignore
get String.len -> int
```

<div>
<div class="tab">
<button group="String.len" id="link-String.len-Description"  class="tablinks active" 
    onclick="openTab(event, 'String.len', 'Description')">
Description
</button>
<button group="String.len" id="link-String.len-Example"  class="tablinks" 
    onclick="openTab(event, 'String.len', 'Example')">
Example
</button>
</div>

<div group="String.len" id="String.len-Description" class="tabcontent"  style="display: block;" >
Return the length of the string, in number of characters.
</div>
<div group="String.len" id="String.len-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "朝には紅顔ありて夕べには白骨となる";

print(text.len);        // prints 17
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code></code> SystemInfo </h2>

```rust,ignore

```

<div>
<div class="tab">
</div>


</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code></code> SystemInfoPaths </h2>

```rust,ignore

```

<div>
<div class="tab">
</div>


</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> abs </h2>

```rust,ignore
fn abs(x: int) -> int
fn abs(x: float) -> float
fn abs(x: i32) -> i32
fn abs(x: i128) -> i128
fn abs(x: f32) -> f32
fn abs(x: i8) -> i8
fn abs(x: i16) -> i16
```

<div>
<div class="tab">
<button group="abs" id="link-abs-Description"  class="tablinks active" 
    onclick="openTab(event, 'abs', 'Description')">
Description
</button>
</div>

<div group="abs" id="abs-Description" class="tabcontent"  style="display: block;" >
Return the absolute value of the number.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> acos </h2>

```rust,ignore
fn acos(x: float) -> float
```

<div>
<div class="tab">
<button group="acos" id="link-acos-Description"  class="tablinks active" 
    onclick="openTab(event, 'acos', 'Description')">
Description
</button>
</div>

<div group="acos" id="acos-Description" class="tabcontent"  style="display: block;" >
Return the arc-cosine of the floating-point number, in radians.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> acosh </h2>

```rust,ignore
fn acosh(x: float) -> float
```

<div>
<div class="tab">
<button group="acosh" id="link-acosh-Description"  class="tablinks active" 
    onclick="openTab(event, 'acosh', 'Description')">
Description
</button>
</div>

<div group="acosh" id="acosh-Description" class="tabcontent"  style="display: block;" >
Return the arc-hyperbolic-cosine of the floating-point number, in radians.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> all </h2>

```rust,ignore
fn all(array: Array, filter: FnPtr) -> bool
fn all(array: Array, filter: String) -> bool
```

<div>
<div class="tab">
<button group="all" id="link-all-Description"  class="tablinks active" 
    onclick="openTab(event, 'all', 'Description')">
Description
</button>
<button group="all" id="link-all-No Function Parameter"  class="tablinks" 
    onclick="openTab(event, 'all', 'No Function Parameter')">
No Function Parameter
</button>
<button group="all" id="link-all-Function Parameters"  class="tablinks" 
    onclick="openTab(event, 'all', 'Function Parameters')">
Function Parameters
</button>
<button group="all" id="link-all-Example"  class="tablinks" 
    onclick="openTab(event, 'all', 'Example')">
Example
</button>
</div>

<div group="all" id="all-Description" class="tabcontent"  style="display: block;" >
Return `true` if all elements in the array return `true` when applied the `filter` function.
</div>
<div group="all" id="all-No Function Parameter" class="tabcontent"  style="display: none;" >

Array element (mutable) is bound to `this`.

This method is marked _pure_; the `filter` function should not mutate array elements.
</div>
<div group="all" id="all-Function Parameters" class="tabcontent"  style="display: none;" >

* `element`: copy of array element
* `index` _(optional)_: current index in the array
</div>
<div group="all" id="all-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 5];

print(x.all(|v| v > 3));        // prints false

print(x.all(|v| v > 1));        // prints true

print(x.all(|v, i| i > v));     // prints false
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> append </h2>

```rust,ignore
fn append(blob: Blob, character: char)
fn append(blob: Blob, value: int)
fn append(string: String, mut item: ?)
fn append(blob1: Blob, blob2: Blob)
fn append(array: Array, new_array: Array)
fn append(blob: Blob, string: String)
fn append(string: String, utf8: Blob)
```

<div>
<div class="tab">
<button group="append" id="link-append-Description"  class="tablinks active" 
    onclick="openTab(event, 'append', 'Description')">
Description
</button>
<button group="append" id="link-append-Example"  class="tablinks" 
    onclick="openTab(event, 'append', 'Example')">
Example
</button>
</div>

<div group="append" id="append-Description" class="tabcontent"  style="display: block;" >
Add a character (as UTF-8 encoded byte-stream) to the end of the BLOB
</div>
<div group="append" id="append-Example" class="tabcontent"  style="display: none;" >

```rhai
let b = blob(5, 0x42);

b.append('!');

print(b);       // prints "[424242424221]"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> as_string </h2>

```rust,ignore
fn as_string(blob: Blob) -> String
```

<div>
<div class="tab">
<button group="as_string" id="link-as_string-Description"  class="tablinks active" 
    onclick="openTab(event, 'as_string', 'Description')">
Description
</button>
<button group="as_string" id="link-as_string-Example"  class="tablinks" 
    onclick="openTab(event, 'as_string', 'Example')">
Example
</button>
</div>

<div group="as_string" id="as_string-Description" class="tabcontent"  style="display: block;" >
Convert the BLOB into a string.

The byte stream must be valid UTF-8, otherwise an error is raised.
</div>
<div group="as_string" id="as_string-Example" class="tabcontent"  style="display: none;" >

```rhai
let b = blob(5, 0x42);

let x = b.as_string();

print(x);       // prints "FFFFF"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> asin </h2>

```rust,ignore
fn asin(x: float) -> float
```

<div>
<div class="tab">
<button group="asin" id="link-asin-Description"  class="tablinks active" 
    onclick="openTab(event, 'asin', 'Description')">
Description
</button>
</div>

<div group="asin" id="asin-Description" class="tabcontent"  style="display: block;" >
Return the arc-sine of the floating-point number, in radians.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> asinh </h2>

```rust,ignore
fn asinh(x: float) -> float
```

<div>
<div class="tab">
<button group="asinh" id="link-asinh-Description"  class="tablinks active" 
    onclick="openTab(event, 'asinh', 'Description')">
Description
</button>
</div>

<div group="asinh" id="asinh-Description" class="tabcontent"  style="display: block;" >
Return the arc-hyperbolic-sine of the floating-point number, in radians.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> atan </h2>

```rust,ignore
fn atan(x: float) -> float
fn atan(x: float, y: float) -> float
```

<div>
<div class="tab">
<button group="atan" id="link-atan-Description"  class="tablinks active" 
    onclick="openTab(event, 'atan', 'Description')">
Description
</button>
</div>

<div group="atan" id="atan-Description" class="tabcontent"  style="display: block;" >
Return the arc-tangent of the floating-point number, in radians.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> atanh </h2>

```rust,ignore
fn atanh(x: float) -> float
```

<div>
<div class="tab">
<button group="atanh" id="link-atanh-Description"  class="tablinks active" 
    onclick="openTab(event, 'atanh', 'Description')">
Description
</button>
</div>

<div group="atanh" id="atanh-Description" class="tabcontent"  style="display: block;" >
Return the arc-hyperbolic-tangent of the floating-point number, in radians.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> bits </h2>

```rust,ignore
fn bits(value: int) -> BitRange
fn bits(value: int, from: int) -> BitRange
fn bits(value: int, range: RangeInclusive<int>) -> BitRange
fn bits(value: int, range: Range<int>) -> BitRange
fn bits(value: int, from: int, len: int) -> BitRange
```

<div>
<div class="tab">
<button group="bits" id="link-bits-Description"  class="tablinks active" 
    onclick="openTab(event, 'bits', 'Description')">
Description
</button>
<button group="bits" id="link-bits-Example"  class="tablinks" 
    onclick="openTab(event, 'bits', 'Example')">
Example
</button>
</div>

<div group="bits" id="bits-Description" class="tabcontent"  style="display: block;" >
Return an iterator over all the bits in the number.
</div>
<div group="bits" id="bits-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = 123456;

for bit in x.bits() {
    print(bit);
}
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> blob </h2>

```rust,ignore
fn blob() -> Blob
fn blob(len: int) -> Blob
fn blob(len: int, value: int) -> Blob
```

<div>
<div class="tab">
<button group="blob" id="link-blob-Description"  class="tablinks active" 
    onclick="openTab(event, 'blob', 'Description')">
Description
</button>
</div>

<div group="blob" id="blob-Description" class="tabcontent"  style="display: block;" >
Return a new, empty BLOB.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> bytes </h2>

```rust,ignore
fn bytes(string: String) -> int
```

<div>
<div class="tab">
<button group="bytes" id="link-bytes-Description"  class="tablinks active" 
    onclick="openTab(event, 'bytes', 'Description')">
Description
</button>
<button group="bytes" id="link-bytes-Example"  class="tablinks" 
    onclick="openTab(event, 'bytes', 'Example')">
Example
</button>
</div>

<div group="bytes" id="bytes-Description" class="tabcontent"  style="display: block;" >
Return the length of the string, in number of bytes used to store it in UTF-8 encoding.
</div>
<div group="bytes" id="bytes-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "朝には紅顔ありて夕べには白骨となる";

print(text.bytes);      // prints 51
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> ceiling </h2>

```rust,ignore
fn ceiling(x: float) -> float
```

<div>
<div class="tab">
<button group="ceiling" id="link-ceiling-Description"  class="tablinks active" 
    onclick="openTab(event, 'ceiling', 'Description')">
Description
</button>
</div>

<div group="ceiling" id="ceiling-Description" class="tabcontent"  style="display: block;" >
Return the smallest whole number larger than or equals to the floating-point number.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> chars </h2>

```rust,ignore
fn chars(string: String) -> CharsStream
fn chars(string: String, start: int) -> CharsStream
fn chars(string: String, range: Range<int>) -> CharsStream
fn chars(string: String, range: RangeInclusive<int>) -> CharsStream
fn chars(string: String, start: int, len: int) -> CharsStream
```

<div>
<div class="tab">
<button group="chars" id="link-chars-Description"  class="tablinks active" 
    onclick="openTab(event, 'chars', 'Description')">
Description
</button>
<button group="chars" id="link-chars-Example"  class="tablinks" 
    onclick="openTab(event, 'chars', 'Example')">
Example
</button>
</div>

<div group="chars" id="chars-Description" class="tabcontent"  style="display: block;" >
Return an iterator over the characters in the string.
</div>
<div group="chars" id="chars-Example" class="tabcontent"  style="display: none;" >

```rhai
for ch in "hello, world!".chars() {
    print(ch);
}
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> chop </h2>

```rust,ignore
fn chop(blob: Blob, len: int)
fn chop(array: Array, len: int)
```

<div>
<div class="tab">
<button group="chop" id="link-chop-Description"  class="tablinks active" 
    onclick="openTab(event, 'chop', 'Description')">
Description
</button>
<button group="chop" id="link-chop-Example"  class="tablinks" 
    onclick="openTab(event, 'chop', 'Example')">
Example
</button>
</div>

<div group="chop" id="chop-Description" class="tabcontent"  style="display: block;" >
Cut off the head of the BLOB, leaving a tail of the specified length.

* If `len` ≤ 0, the BLOB is cleared.
* If `len` ≥ length of BLOB, the BLOB is not modified.
</div>
<div group="chop" id="chop-Example" class="tabcontent"  style="display: none;" >

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
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> clear </h2>

```rust,ignore
fn clear(blob: Blob)
fn clear(string: String)
fn clear(array: Array)
fn clear(map: Map)
```

<div>
<div class="tab">
<button group="clear" id="link-clear-Description"  class="tablinks active" 
    onclick="openTab(event, 'clear', 'Description')">
Description
</button>
</div>

<div group="clear" id="clear-Description" class="tabcontent"  style="display: block;" >
Clear the BLOB.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> contains </h2>

```rust,ignore
fn contains(array: Array, value: ?) -> bool
fn contains(map: Map, property: String) -> bool
fn contains(range: RangeInclusive<int>, value: int) -> bool
fn contains(range: Range<int>, value: int) -> bool
fn contains(blob: Blob, value: int) -> bool
fn contains(string: String, match_string: String) -> bool
fn contains(string: String, character: char) -> bool
```

<div>
<div class="tab">
<button group="contains" id="link-contains-Description"  class="tablinks active" 
    onclick="openTab(event, 'contains', 'Description')">
Description
</button>
<button group="contains" id="link-contains-Example"  class="tablinks" 
    onclick="openTab(event, 'contains', 'Example')">
Example
</button>
</div>

<div group="contains" id="contains-Description" class="tabcontent"  style="display: block;" >
Return `true` if the array contains an element that equals `value`.

The operator `==` is used to compare elements with `value` and must be defined,
otherwise `false` is assumed.

This function also drives the `in` operator.
</div>
<div group="contains" id="contains-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = [1, 2, 3, 4, 5];

// The 'in' operator calls 'contains' in the background
if 4 in x {
    print("found!");
}
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> cos </h2>

```rust,ignore
fn cos(x: float) -> float
```

<div>
<div class="tab">
<button group="cos" id="link-cos-Description"  class="tablinks active" 
    onclick="openTab(event, 'cos', 'Description')">
Description
</button>
</div>

<div group="cos" id="cos-Description" class="tabcontent"  style="display: block;" >
Return the cosine of the floating-point number in radians.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> cosh </h2>

```rust,ignore
fn cosh(x: float) -> float
```

<div>
<div class="tab">
<button group="cosh" id="link-cosh-Description"  class="tablinks active" 
    onclick="openTab(event, 'cosh', 'Description')">
Description
</button>
</div>

<div group="cosh" id="cosh-Description" class="tabcontent"  style="display: block;" >
Return the hyperbolic cosine of the floating-point number in radians.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> crop </h2>

```rust,ignore
fn crop(string: String, range: Range<int>)
fn crop(string: String, start: int)
fn crop(string: String, range: RangeInclusive<int>)
fn crop(string: String, start: int, len: int)
```

<div>
<div class="tab">
<button group="crop" id="link-crop-Description"  class="tablinks active" 
    onclick="openTab(event, 'crop', 'Description')">
Description
</button>
<button group="crop" id="link-crop-Example"  class="tablinks" 
    onclick="openTab(event, 'crop', 'Example')">
Example
</button>
</div>

<div group="crop" id="crop-Description" class="tabcontent"  style="display: block;" >
Remove all characters from the string except those within an exclusive `range`.
</div>
<div group="crop" id="crop-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "hello, world!";

text.crop(2..8);

print(text);        // prints "llo, w"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> debug </h2>

```rust,ignore
fn debug() -> String
fn debug(number: f32) -> String
fn debug(value: bool) -> String
fn debug(string: String) -> String
fn debug(number: float) -> String
fn debug(unit: ?) -> String
fn debug(f: FnPtr) -> String
fn debug(map: Map) -> String
fn debug(array: Array) -> String
fn debug(item: ?) -> String
fn debug(character: char) -> String
```

<div>
<div class="tab">
<button group="debug" id="link-debug-Description"  class="tablinks active" 
    onclick="openTab(event, 'debug', 'Description')">
Description
</button>
</div>

<div group="debug" id="debug-Description" class="tabcontent"  style="display: block;" >
Return the empty string.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> dedup </h2>

```rust,ignore
fn dedup(array: Array)
fn dedup(array: Array, comparer: String)
fn dedup(array: Array, comparer: FnPtr)
```

<div>
<div class="tab">
<button group="dedup" id="link-dedup-Description"  class="tablinks active" 
    onclick="openTab(event, 'dedup', 'Description')">
Description
</button>
<button group="dedup" id="link-dedup-Example"  class="tablinks" 
    onclick="openTab(event, 'dedup', 'Example')">
Example
</button>
</div>

<div group="dedup" id="dedup-Description" class="tabcontent"  style="display: block;" >
Remove duplicated _consecutive_ elements from the array.

The operator `==` is used to compare elements and must be defined,
otherwise `false` is assumed.
</div>
<div group="dedup" id="dedup-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = [1, 2, 2, 2, 3, 4, 3, 3, 2, 1];

x.dedup();

print(x);       // prints "[1, 2, 3, 4, 3, 2, 1]"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> drain </h2>

```rust,ignore
fn drain(array: Array, range: Range<int>) -> Array
fn drain(blob: Blob, range: Range<int>) -> Blob
fn drain(array: Array, filter: FnPtr) -> Array
fn drain(blob: Blob, range: RangeInclusive<int>) -> Blob
fn drain(array: Array, range: RangeInclusive<int>) -> Array
fn drain(map: Map, filter: FnPtr) -> Map
fn drain(array: Array, filter: String) -> Array
fn drain(array: Array, start: int, len: int) -> Array
fn drain(blob: Blob, start: int, len: int) -> Blob
```

<div>
<div class="tab">
<button group="drain" id="link-drain-Description"  class="tablinks active" 
    onclick="openTab(event, 'drain', 'Description')">
Description
</button>
<button group="drain" id="link-drain-Example"  class="tablinks" 
    onclick="openTab(event, 'drain', 'Example')">
Example
</button>
</div>

<div group="drain" id="drain-Description" class="tabcontent"  style="display: block;" >
Remove all elements in the array within an exclusive `range` and return them as a new array.
</div>
<div group="drain" id="drain-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = [1, 2, 3, 4, 5];

let y = x.drain(1..3);

print(x);       // prints "[1, 4, 5]"

print(y);       // prints "[2, 3]"

let z = x.drain(2..3);

print(x);       // prints "[1, 4]"

print(z);       // prints "[5]"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> elapsed </h2>

```rust,ignore
fn elapsed(timestamp: Instant) -> ?
```

<div>
<div class="tab">
<button group="elapsed" id="link-elapsed-Description"  class="tablinks active" 
    onclick="openTab(event, 'elapsed', 'Description')">
Description
</button>
<button group="elapsed" id="link-elapsed-Example"  class="tablinks" 
    onclick="openTab(event, 'elapsed', 'Example')">
Example
</button>
</div>

<div group="elapsed" id="elapsed-Description" class="tabcontent"  style="display: block;" >
Return the number of seconds between the current system time and the timestamp.
</div>
<div group="elapsed" id="elapsed-Example" class="tabcontent"  style="display: none;" >

```rhai
let now = timestamp();

sleep(10.0);            // sleep for 10 seconds

print(now.elapsed);     // prints 10.???
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> end </h2>

```rust,ignore
fn end(range: Range<int>) -> int
fn end(range: RangeInclusive<int>) -> int
```

<div>
<div class="tab">
<button group="end" id="link-end-Description"  class="tablinks active" 
    onclick="openTab(event, 'end', 'Description')">
Description
</button>
</div>

<div group="end" id="end-Description" class="tabcontent"  style="display: block;" >
Return the end of the exclusive range.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> ends_with </h2>

```rust,ignore
fn ends_with(string: String, match_string: String) -> bool
```

<div>
<div class="tab">
<button group="ends_with" id="link-ends_with-Description"  class="tablinks active" 
    onclick="openTab(event, 'ends_with', 'Description')">
Description
</button>
<button group="ends_with" id="link-ends_with-Example"  class="tablinks" 
    onclick="openTab(event, 'ends_with', 'Example')">
Example
</button>
</div>

<div group="ends_with" id="ends_with-Description" class="tabcontent"  style="display: block;" >
Return `true` if the string ends with a specified string.
</div>
<div group="ends_with" id="ends_with-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "hello, world!";

print(text.ends_with("world!"));    // prints true

print(text.ends_with("hello"));     // prints false
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> exit </h2>

```rust,ignore
fn exit() -> ?
fn exit(value: ?) -> ?
```

<div>
<div class="tab">
<button group="exit" id="link-exit-Description"  class="tablinks active" 
    onclick="openTab(event, 'exit', 'Description')">
Description
</button>
<button group="exit" id="link-exit-Example"  class="tablinks" 
    onclick="openTab(event, 'exit', 'Example')">
Example
</button>
</div>

<div group="exit" id="exit-Description" class="tabcontent"  style="display: block;" >
Exit the script evaluation immediately with `()` as exit value.
</div>
<div group="exit" id="exit-Example" class="tabcontent"  style="display: none;" >
```rhai
exit();
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> exp </h2>

```rust,ignore
fn exp(x: float) -> float
```

<div>
<div class="tab">
<button group="exp" id="link-exp-Description"  class="tablinks active" 
    onclick="openTab(event, 'exp', 'Description')">
Description
</button>
</div>

<div group="exp" id="exp-Description" class="tabcontent"  style="display: block;" >
Return the exponential of the floating-point number.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> extract </h2>

```rust,ignore
fn extract(array: Array, start: int) -> Array
fn extract(array: Array, range: Range<int>) -> Array
fn extract(blob: Blob, start: int) -> Blob
fn extract(blob: Blob, range: RangeInclusive<int>) -> Blob
fn extract(array: Array, range: RangeInclusive<int>) -> Array
fn extract(blob: Blob, range: Range<int>) -> Blob
fn extract(blob: Blob, start: int, len: int) -> Blob
fn extract(array: Array, start: int, len: int) -> Array
```

<div>
<div class="tab">
<button group="extract" id="link-extract-Description"  class="tablinks active" 
    onclick="openTab(event, 'extract', 'Description')">
Description
</button>
<button group="extract" id="link-extract-Example"  class="tablinks" 
    onclick="openTab(event, 'extract', 'Example')">
Example
</button>
</div>

<div group="extract" id="extract-Description" class="tabcontent"  style="display: block;" >
Copy a portion of the array beginning at the `start` position till the end and return it as
a new array.

* If `start` < 0, position counts from the end of the array (`-1` is the last element).
* If `start` < -length of array, the entire array is copied and returned.
* If `start` ≥ length of array, an empty array is returned.
</div>
<div group="extract" id="extract-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = [1, 2, 3, 4, 5];

print(x.extract(2));        // prints "[3, 4, 5]"

print(x.extract(-3));       // prints "[3, 4, 5]"

print(x);                   // prints "[1, 2, 3, 4, 5]"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> f32.is_zero </h2>

```rust,ignore
get f32.is_zero -> bool
```

<div>
<div class="tab">
<button group="f32.is_zero" id="link-f32.is_zero-Description"  class="tablinks active" 
    onclick="openTab(event, 'f32.is_zero', 'Description')">
Description
</button>
</div>

<div group="f32.is_zero" id="f32.is_zero-Description" class="tabcontent"  style="display: block;" >
Return true if the floating-point number is zero.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> fill_with </h2>

```rust,ignore
fn fill_with(map: Map, map2: Map)
```

<div>
<div class="tab">
<button group="fill_with" id="link-fill_with-Description"  class="tablinks active" 
    onclick="openTab(event, 'fill_with', 'Description')">
Description
</button>
<button group="fill_with" id="link-fill_with-Example"  class="tablinks" 
    onclick="openTab(event, 'fill_with', 'Example')">
Example
</button>
</div>

<div group="fill_with" id="fill_with-Description" class="tabcontent"  style="display: block;" >
Add all property values of another object map into the object map.
Only properties that do not originally exist in the object map are added.
</div>
<div group="fill_with" id="fill_with-Example" class="tabcontent"  style="display: none;" >

```rhai
let m = #{a:1, b:2, c:3};
let n = #{a: 42, d:0};

m.fill_with(n);

print(m);       // prints "#{a:1, b:2, c:3, d:0}"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> filter </h2>

```rust,ignore
fn filter(array: Array, filter: FnPtr) -> Array
fn filter(array: Array, filter_func: String) -> Array
fn filter(map: Map, filter: FnPtr) -> Map
```

<div>
<div class="tab">
<button group="filter" id="link-filter-Description"  class="tablinks active" 
    onclick="openTab(event, 'filter', 'Description')">
Description
</button>
<button group="filter" id="link-filter-No Function Parameter"  class="tablinks" 
    onclick="openTab(event, 'filter', 'No Function Parameter')">
No Function Parameter
</button>
<button group="filter" id="link-filter-Function Parameters"  class="tablinks" 
    onclick="openTab(event, 'filter', 'Function Parameters')">
Function Parameters
</button>
<button group="filter" id="link-filter-Example"  class="tablinks" 
    onclick="openTab(event, 'filter', 'Example')">
Example
</button>
</div>

<div group="filter" id="filter-Description" class="tabcontent"  style="display: block;" >
Iterate through all the elements in the array, applying a `filter` function to each element
in turn, and return a copy of all elements (in order) that return `true` as a new array.
</div>
<div group="filter" id="filter-No Function Parameter" class="tabcontent"  style="display: none;" >

Array element (mutable) is bound to `this`.

This method is marked _pure_; the `filter` function should not mutate array elements.
</div>
<div group="filter" id="filter-Function Parameters" class="tabcontent"  style="display: none;" >

* `element`: copy of array element
* `index` _(optional)_: current index in the array
</div>
<div group="filter" id="filter-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = [1, 2, 3, 4, 5];

let y = x.filter(|v| v >= 3);

print(y);       // prints "[3, 4, 5]"

let y = x.filter(|v, i| v * i >= 10);

print(y);       // prints "[12, 20]"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> find </h2>

```rust,ignore
fn find(array: Array, filter: FnPtr) -> ?
fn find(array: Array, filter: FnPtr, start: int) -> ?
```

<div>
<div class="tab">
<button group="find" id="link-find-Description"  class="tablinks active" 
    onclick="openTab(event, 'find', 'Description')">
Description
</button>
<button group="find" id="link-find-No Function Parameter"  class="tablinks" 
    onclick="openTab(event, 'find', 'No Function Parameter')">
No Function Parameter
</button>
<button group="find" id="link-find-Function Parameters"  class="tablinks" 
    onclick="openTab(event, 'find', 'Function Parameters')">
Function Parameters
</button>
<button group="find" id="link-find-Example"  class="tablinks" 
    onclick="openTab(event, 'find', 'Example')">
Example
</button>
</div>

<div group="find" id="find-Description" class="tabcontent"  style="display: block;" >
Iterate through all the elements in the array, applying a `filter` function to each element
in turn, and return a copy of the first element that returns `true`. If no element returns
`true`, `()` is returned.
</div>
<div group="find" id="find-No Function Parameter" class="tabcontent"  style="display: none;" >

Array element (mutable) is bound to `this`.
</div>
<div group="find" id="find-Function Parameters" class="tabcontent"  style="display: none;" >

* `element`: copy of array element
* `index` _(optional)_: current index in the array
</div>
<div group="find" id="find-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = [1, 2, 3, 5, 8, 13];

print(x.find(|v| v > 3));                    // prints 5: 5 > 3

print(x.find(|v| v > 13) ?? "not found");    // prints "not found": nothing is > 13

print(x.find(|v, i| v * i > 13));            // prints 5: 3 * 5 > 13
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> find_map </h2>

```rust,ignore
fn find_map(array: Array, filter: FnPtr) -> ?
fn find_map(array: Array, filter: FnPtr, start: int) -> ?
```

<div>
<div class="tab">
<button group="find_map" id="link-find_map-Description"  class="tablinks active" 
    onclick="openTab(event, 'find_map', 'Description')">
Description
</button>
<button group="find_map" id="link-find_map-No Function Parameter"  class="tablinks" 
    onclick="openTab(event, 'find_map', 'No Function Parameter')">
No Function Parameter
</button>
<button group="find_map" id="link-find_map-Function Parameters"  class="tablinks" 
    onclick="openTab(event, 'find_map', 'Function Parameters')">
Function Parameters
</button>
<button group="find_map" id="link-find_map-Example"  class="tablinks" 
    onclick="openTab(event, 'find_map', 'Example')">
Example
</button>
</div>

<div group="find_map" id="find_map-Description" class="tabcontent"  style="display: block;" >
Iterate through all the elements in the array, applying a `mapper` function to each element
in turn, and return the first result that is not `()`. Otherwise, `()` is returned.
</div>
<div group="find_map" id="find_map-No Function Parameter" class="tabcontent"  style="display: none;" >

Array element (mutable) is bound to `this`.

This method is marked _pure_; the `mapper` function should not mutate array elements.
</div>
<div group="find_map" id="find_map-Function Parameters" class="tabcontent"  style="display: none;" >

* `element`: copy of array element
* `index` _(optional)_: current index in the array
</div>
<div group="find_map" id="find_map-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = [#{alice: 1}, #{bob: 2}, #{clara: 3}];

print(x.find_map(|v| v.alice));                  // prints 1

print(x.find_map(|v| v.dave) ?? "not found");    // prints "not found"

print(x.find_map(|| this.dave) ?? "not found");  // prints "not found"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> float.ceiling </h2>

```rust,ignore
get float.ceiling -> float
```

<div>
<div class="tab">
<button group="float.ceiling" id="link-float.ceiling-Description"  class="tablinks active" 
    onclick="openTab(event, 'float.ceiling', 'Description')">
Description
</button>
</div>

<div group="float.ceiling" id="float.ceiling-Description" class="tabcontent"  style="display: block;" >
Return the smallest whole number larger than or equals to the floating-point number.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> float.floor </h2>

```rust,ignore
get float.floor -> float
```

<div>
<div class="tab">
<button group="float.floor" id="link-float.floor-Description"  class="tablinks active" 
    onclick="openTab(event, 'float.floor', 'Description')">
Description
</button>
</div>

<div group="float.floor" id="float.floor-Description" class="tabcontent"  style="display: block;" >
Return the largest whole number less than or equals to the floating-point number.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> float.fraction </h2>

```rust,ignore
get float.fraction -> float
```

<div>
<div class="tab">
<button group="float.fraction" id="link-float.fraction-Description"  class="tablinks active" 
    onclick="openTab(event, 'float.fraction', 'Description')">
Description
</button>
</div>

<div group="float.fraction" id="float.fraction-Description" class="tabcontent"  style="display: block;" >
Return the fractional part of the floating-point number.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> float.int </h2>

```rust,ignore
get float.int -> float
```

<div>
<div class="tab">
<button group="float.int" id="link-float.int-Description"  class="tablinks active" 
    onclick="openTab(event, 'float.int', 'Description')">
Description
</button>
</div>

<div group="float.int" id="float.int-Description" class="tabcontent"  style="display: block;" >
Return the integral part of the floating-point number.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> float.is_finite </h2>

```rust,ignore
get float.is_finite -> bool
```

<div>
<div class="tab">
<button group="float.is_finite" id="link-float.is_finite-Description"  class="tablinks active" 
    onclick="openTab(event, 'float.is_finite', 'Description')">
Description
</button>
</div>

<div group="float.is_finite" id="float.is_finite-Description" class="tabcontent"  style="display: block;" >
Return `true` if the floating-point number is finite.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> float.is_infinite </h2>

```rust,ignore
get float.is_infinite -> bool
```

<div>
<div class="tab">
<button group="float.is_infinite" id="link-float.is_infinite-Description"  class="tablinks active" 
    onclick="openTab(event, 'float.is_infinite', 'Description')">
Description
</button>
</div>

<div group="float.is_infinite" id="float.is_infinite-Description" class="tabcontent"  style="display: block;" >
Return `true` if the floating-point number is infinite.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> float.is_nan </h2>

```rust,ignore
get float.is_nan -> bool
```

<div>
<div class="tab">
<button group="float.is_nan" id="link-float.is_nan-Description"  class="tablinks active" 
    onclick="openTab(event, 'float.is_nan', 'Description')">
Description
</button>
</div>

<div group="float.is_nan" id="float.is_nan-Description" class="tabcontent"  style="display: block;" >
Return `true` if the floating-point number is `NaN` (Not A Number).
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> float.is_zero </h2>

```rust,ignore
get float.is_zero -> bool
```

<div>
<div class="tab">
<button group="float.is_zero" id="link-float.is_zero-Description"  class="tablinks active" 
    onclick="openTab(event, 'float.is_zero', 'Description')">
Description
</button>
</div>

<div group="float.is_zero" id="float.is_zero-Description" class="tabcontent"  style="display: block;" >
Return true if the floating-point number is zero.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> float.round </h2>

```rust,ignore
get float.round -> float
```

<div>
<div class="tab">
<button group="float.round" id="link-float.round-Description"  class="tablinks active" 
    onclick="openTab(event, 'float.round', 'Description')">
Description
</button>
</div>

<div group="float.round" id="float.round-Description" class="tabcontent"  style="display: block;" >
Return the nearest whole number closest to the floating-point number.
Rounds away from zero.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> floor </h2>

```rust,ignore
fn floor(x: float) -> float
```

<div>
<div class="tab">
<button group="floor" id="link-floor-Description"  class="tablinks active" 
    onclick="openTab(event, 'floor', 'Description')">
Description
</button>
</div>

<div group="floor" id="floor-Description" class="tabcontent"  style="display: block;" >
Return the largest whole number less than or equals to the floating-point number.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> for_each </h2>

```rust,ignore
fn for_each(array: Array, map: FnPtr)
```

<div>
<div class="tab">
<button group="for_each" id="link-for_each-Description"  class="tablinks active" 
    onclick="openTab(event, 'for_each', 'Description')">
Description
</button>
<button group="for_each" id="link-for_each-Function Parameters"  class="tablinks" 
    onclick="openTab(event, 'for_each', 'Function Parameters')">
Function Parameters
</button>
<button group="for_each" id="link-for_each-Example"  class="tablinks" 
    onclick="openTab(event, 'for_each', 'Example')">
Example
</button>
</div>

<div group="for_each" id="for_each-Description" class="tabcontent"  style="display: block;" >
Iterate through all the elements in the array, applying a `process` function to each element in turn.
Each element is bound to `this` before calling the function.
</div>
<div group="for_each" id="for_each-Function Parameters" class="tabcontent"  style="display: none;" >

* `this`: bound to array element (mutable)
* `index` _(optional)_: current index in the array
</div>
<div group="for_each" id="for_each-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = [1, 2, 3, 4, 5];

x.for_each(|| this *= this);

print(x);       // prints "[1, 4, 9, 16, 25]"

x.for_each(|i| this *= i);

print(x);       // prints "[0, 2, 6, 12, 20]"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> fraction </h2>

```rust,ignore
fn fraction(x: float) -> float
```

<div>
<div class="tab">
<button group="fraction" id="link-fraction-Description"  class="tablinks active" 
    onclick="openTab(event, 'fraction', 'Description')">
Description
</button>
</div>

<div group="fraction" id="fraction-Description" class="tabcontent"  style="display: block;" >
Return the fractional part of the floating-point number.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> get </h2>

```rust,ignore
fn get(string: String, index: int) -> ?
fn get(array: Array, index: int) -> ?
fn get(map: Map, property: String) -> ?
fn get(blob: Blob, index: int) -> int
```

<div>
<div class="tab">
<button group="get" id="link-get-Description"  class="tablinks active" 
    onclick="openTab(event, 'get', 'Description')">
Description
</button>
<button group="get" id="link-get-Example"  class="tablinks" 
    onclick="openTab(event, 'get', 'Example')">
Example
</button>
</div>

<div group="get" id="get-Description" class="tabcontent"  style="display: block;" >
Get the character at the `index` position in the string.

* If `index` < 0, position counts from the end of the string (`-1` is the last character).
* If `index` < -length of string, zero is returned.
* If `index` ≥ length of string, zero is returned.
</div>
<div group="get" id="get-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "hello, world!";

print(text.get(0));     // prints 'h'

print(text.get(-1));    // prints '!'

print(text.get(99));    // prints empty (for '()')'
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> get_bit </h2>

```rust,ignore
fn get_bit(value: int, bit: int) -> bool
```

<div>
<div class="tab">
<button group="get_bit" id="link-get_bit-Description"  class="tablinks active" 
    onclick="openTab(event, 'get_bit', 'Description')">
Description
</button>
<button group="get_bit" id="link-get_bit-Example"  class="tablinks" 
    onclick="openTab(event, 'get_bit', 'Example')">
Example
</button>
</div>

<div group="get_bit" id="get_bit-Description" class="tabcontent"  style="display: block;" >
Return `true` if the specified `bit` in the number is set.

If `bit` < 0, position counts from the MSB (Most Significant Bit).
</div>
<div group="get_bit" id="get_bit-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = 123456;

print(x.get_bit(5));    // prints false

print(x.get_bit(6));    // prints true

print(x.get_bit(-48));  // prints true on 64-bit
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> get_bits </h2>

```rust,ignore
fn get_bits(value: int, range: Range<int>) -> int
fn get_bits(value: int, range: RangeInclusive<int>) -> int
fn get_bits(value: int, start: int, bits: int) -> int
```

<div>
<div class="tab">
<button group="get_bits" id="link-get_bits-Description"  class="tablinks active" 
    onclick="openTab(event, 'get_bits', 'Description')">
Description
</button>
<button group="get_bits" id="link-get_bits-Example"  class="tablinks" 
    onclick="openTab(event, 'get_bits', 'Example')">
Example
</button>
</div>

<div group="get_bits" id="get_bits-Description" class="tabcontent"  style="display: block;" >
Return an exclusive range of bits in the number as a new number.
</div>
<div group="get_bits" id="get_bits-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = 123456;

print(x.get_bits(5..10));       // print 18
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> get_fn_metadata_list </h2>

```rust,ignore
fn get_fn_metadata_list() -> Array
fn get_fn_metadata_list(name: String) -> Array
fn get_fn_metadata_list(name: String, params: int) -> Array
```

<div>
<div class="tab">
<button group="get_fn_metadata_list" id="link-get_fn_metadata_list-Description"  class="tablinks active" 
    onclick="openTab(event, 'get_fn_metadata_list', 'Description')">
Description
</button>
</div>

<div group="get_fn_metadata_list" id="get_fn_metadata_list-Description" class="tabcontent"  style="display: block;" >
Return an array of object maps containing metadata of all script-defined functions.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> hypot </h2>

```rust,ignore
fn hypot(x: float, y: float) -> float
```

<div>
<div class="tab">
<button group="hypot" id="link-hypot-Description"  class="tablinks active" 
    onclick="openTab(event, 'hypot', 'Description')">
Description
</button>
</div>

<div group="hypot" id="hypot-Description" class="tabcontent"  style="display: block;" >
Return the hypotenuse of a triangle with sides `x` and `y`.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> i128.is_even </h2>

```rust,ignore
get i128.is_even -> bool
```

<div>
<div class="tab">
<button group="i128.is_even" id="link-i128.is_even-Description"  class="tablinks active" 
    onclick="openTab(event, 'i128.is_even', 'Description')">
Description
</button>
</div>

<div group="i128.is_even" id="i128.is_even-Description" class="tabcontent"  style="display: block;" >
Return true if the number is even.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> i128.is_odd </h2>

```rust,ignore
get i128.is_odd -> bool
```

<div>
<div class="tab">
<button group="i128.is_odd" id="link-i128.is_odd-Description"  class="tablinks active" 
    onclick="openTab(event, 'i128.is_odd', 'Description')">
Description
</button>
</div>

<div group="i128.is_odd" id="i128.is_odd-Description" class="tabcontent"  style="display: block;" >
Return true if the number is odd.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> i128.is_zero </h2>

```rust,ignore
get i128.is_zero -> bool
```

<div>
<div class="tab">
<button group="i128.is_zero" id="link-i128.is_zero-Description"  class="tablinks active" 
    onclick="openTab(event, 'i128.is_zero', 'Description')">
Description
</button>
</div>

<div group="i128.is_zero" id="i128.is_zero-Description" class="tabcontent"  style="display: block;" >
Return true if the number is zero.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> i16.is_even </h2>

```rust,ignore
get i16.is_even -> bool
```

<div>
<div class="tab">
<button group="i16.is_even" id="link-i16.is_even-Description"  class="tablinks active" 
    onclick="openTab(event, 'i16.is_even', 'Description')">
Description
</button>
</div>

<div group="i16.is_even" id="i16.is_even-Description" class="tabcontent"  style="display: block;" >
Return true if the number is even.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> i16.is_odd </h2>

```rust,ignore
get i16.is_odd -> bool
```

<div>
<div class="tab">
<button group="i16.is_odd" id="link-i16.is_odd-Description"  class="tablinks active" 
    onclick="openTab(event, 'i16.is_odd', 'Description')">
Description
</button>
</div>

<div group="i16.is_odd" id="i16.is_odd-Description" class="tabcontent"  style="display: block;" >
Return true if the number is odd.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> i16.is_zero </h2>

```rust,ignore
get i16.is_zero -> bool
```

<div>
<div class="tab">
<button group="i16.is_zero" id="link-i16.is_zero-Description"  class="tablinks active" 
    onclick="openTab(event, 'i16.is_zero', 'Description')">
Description
</button>
</div>

<div group="i16.is_zero" id="i16.is_zero-Description" class="tabcontent"  style="display: block;" >
Return true if the number is zero.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> i32.is_even </h2>

```rust,ignore
get i32.is_even -> bool
```

<div>
<div class="tab">
<button group="i32.is_even" id="link-i32.is_even-Description"  class="tablinks active" 
    onclick="openTab(event, 'i32.is_even', 'Description')">
Description
</button>
</div>

<div group="i32.is_even" id="i32.is_even-Description" class="tabcontent"  style="display: block;" >
Return true if the number is even.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> i32.is_odd </h2>

```rust,ignore
get i32.is_odd -> bool
```

<div>
<div class="tab">
<button group="i32.is_odd" id="link-i32.is_odd-Description"  class="tablinks active" 
    onclick="openTab(event, 'i32.is_odd', 'Description')">
Description
</button>
</div>

<div group="i32.is_odd" id="i32.is_odd-Description" class="tabcontent"  style="display: block;" >
Return true if the number is odd.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> i32.is_zero </h2>

```rust,ignore
get i32.is_zero -> bool
```

<div>
<div class="tab">
<button group="i32.is_zero" id="link-i32.is_zero-Description"  class="tablinks active" 
    onclick="openTab(event, 'i32.is_zero', 'Description')">
Description
</button>
</div>

<div group="i32.is_zero" id="i32.is_zero-Description" class="tabcontent"  style="display: block;" >
Return true if the number is zero.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> i8.is_even </h2>

```rust,ignore
get i8.is_even -> bool
```

<div>
<div class="tab">
<button group="i8.is_even" id="link-i8.is_even-Description"  class="tablinks active" 
    onclick="openTab(event, 'i8.is_even', 'Description')">
Description
</button>
</div>

<div group="i8.is_even" id="i8.is_even-Description" class="tabcontent"  style="display: block;" >
Return true if the number is even.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> i8.is_odd </h2>

```rust,ignore
get i8.is_odd -> bool
```

<div>
<div class="tab">
<button group="i8.is_odd" id="link-i8.is_odd-Description"  class="tablinks active" 
    onclick="openTab(event, 'i8.is_odd', 'Description')">
Description
</button>
</div>

<div group="i8.is_odd" id="i8.is_odd-Description" class="tabcontent"  style="display: block;" >
Return true if the number is odd.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> i8.is_zero </h2>

```rust,ignore
get i8.is_zero -> bool
```

<div>
<div class="tab">
<button group="i8.is_zero" id="link-i8.is_zero-Description"  class="tablinks active" 
    onclick="openTab(event, 'i8.is_zero', 'Description')">
Description
</button>
</div>

<div group="i8.is_zero" id="i8.is_zero-Description" class="tabcontent"  style="display: block;" >
Return true if the number is zero.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> index_of </h2>

```rust,ignore
fn index_of(string: String, character: char) -> int
fn index_of(string: String, find_string: String) -> int
fn index_of(array: Array, filter: FnPtr) -> int
fn index_of(array: Array, value: ?) -> int
fn index_of(array: Array, filter: String) -> int
fn index_of(array: Array, value: ?, start: int) -> int
fn index_of(string: String, find_string: String, start: int) -> int
fn index_of(array: Array, filter: FnPtr, start: int) -> int
fn index_of(string: String, character: char, start: int) -> int
fn index_of(array: Array, filter: String, start: int) -> int
```

<div>
<div class="tab">
<button group="index_of" id="link-index_of-Description"  class="tablinks active" 
    onclick="openTab(event, 'index_of', 'Description')">
Description
</button>
<button group="index_of" id="link-index_of-Example"  class="tablinks" 
    onclick="openTab(event, 'index_of', 'Example')">
Example
</button>
</div>

<div group="index_of" id="index_of-Description" class="tabcontent"  style="display: block;" >
Find the specified `character` in the string and return the first index where it is found.
If the `character` is not found, `-1` is returned.
</div>
<div group="index_of" id="index_of-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "hello, world!";

print(text.index_of('l'));      // prints 2 (first index)

print(text.index_of('x'));      // prints -1
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> insert </h2>

```rust,ignore
fn insert(blob: Blob, index: int, value: int)
fn insert(array: Array, index: int, item: ?)
```

<div>
<div class="tab">
<button group="insert" id="link-insert-Description"  class="tablinks active" 
    onclick="openTab(event, 'insert', 'Description')">
Description
</button>
<button group="insert" id="link-insert-Example"  class="tablinks" 
    onclick="openTab(event, 'insert', 'Example')">
Example
</button>
</div>

<div group="insert" id="insert-Description" class="tabcontent"  style="display: block;" >
Add a byte `value` to the BLOB at a particular `index` position.

* If `index` < 0, position counts from the end of the BLOB (`-1` is the last byte).
* If `index` < -length of BLOB, the byte value is added to the beginning of the BLOB.
* If `index` ≥ length of BLOB, the byte value is appended to the end of the BLOB.

Only the lower 8 bits of the `value` are used; all other bits are ignored.
</div>
<div group="insert" id="insert-Example" class="tabcontent"  style="display: none;" >

```rhai
let b = blob(5, 0x42);

b.insert(2, 0x18);

print(b);       // prints "[4242184242]"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> int </h2>

```rust,ignore
fn int(x: float) -> float
```

<div>
<div class="tab">
<button group="int" id="link-int-Description"  class="tablinks active" 
    onclick="openTab(event, 'int', 'Description')">
Description
</button>
</div>

<div group="int" id="int-Description" class="tabcontent"  style="display: block;" >
Return the integral part of the floating-point number.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> int.bits </h2>

```rust,ignore
get int.bits -> BitRange
```

<div>
<div class="tab">
<button group="int.bits" id="link-int.bits-Description"  class="tablinks active" 
    onclick="openTab(event, 'int.bits', 'Description')">
Description
</button>
<button group="int.bits" id="link-int.bits-Example"  class="tablinks" 
    onclick="openTab(event, 'int.bits', 'Example')">
Example
</button>
</div>

<div group="int.bits" id="int.bits-Description" class="tabcontent"  style="display: block;" >
Return an iterator over all the bits in the number.
</div>
<div group="int.bits" id="int.bits-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = 123456;

for bit in x.bits {
    print(bit);
}
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> int.is_even </h2>

```rust,ignore
get int.is_even -> bool
```

<div>
<div class="tab">
<button group="int.is_even" id="link-int.is_even-Description"  class="tablinks active" 
    onclick="openTab(event, 'int.is_even', 'Description')">
Description
</button>
</div>

<div group="int.is_even" id="int.is_even-Description" class="tabcontent"  style="display: block;" >
Return true if the number is even.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> int.is_odd </h2>

```rust,ignore
get int.is_odd -> bool
```

<div>
<div class="tab">
<button group="int.is_odd" id="link-int.is_odd-Description"  class="tablinks active" 
    onclick="openTab(event, 'int.is_odd', 'Description')">
Description
</button>
</div>

<div group="int.is_odd" id="int.is_odd-Description" class="tabcontent"  style="display: block;" >
Return true if the number is odd.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> int.is_zero </h2>

```rust,ignore
get int.is_zero -> bool
```

<div>
<div class="tab">
<button group="int.is_zero" id="link-int.is_zero-Description"  class="tablinks active" 
    onclick="openTab(event, 'int.is_zero', 'Description')">
Description
</button>
</div>

<div group="int.is_zero" id="int.is_zero-Description" class="tabcontent"  style="display: block;" >
Return true if the number is zero.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> is_anonymous </h2>

```rust,ignore
fn is_anonymous(fn_ptr: FnPtr) -> bool
```

<div>
<div class="tab">
<button group="is_anonymous" id="link-is_anonymous-Description"  class="tablinks active" 
    onclick="openTab(event, 'is_anonymous', 'Description')">
Description
</button>
<button group="is_anonymous" id="link-is_anonymous-Example"  class="tablinks" 
    onclick="openTab(event, 'is_anonymous', 'Example')">
Example
</button>
</div>

<div group="is_anonymous" id="is_anonymous-Description" class="tabcontent"  style="display: block;" >
Return `true` if the function is an anonymous function.
</div>
<div group="is_anonymous" id="is_anonymous-Example" class="tabcontent"  style="display: none;" >

```rhai
let f = |x| x * 2;

print(f.is_anonymous);      // prints true
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> is_empty </h2>

```rust,ignore
fn is_empty(array: Array) -> bool
fn is_empty(map: Map) -> bool
fn is_empty(blob: Blob) -> bool
fn is_empty(string: String) -> bool
fn is_empty(range: Range<int>) -> bool
fn is_empty(range: RangeInclusive<int>) -> bool
```

<div>
<div class="tab">
<button group="is_empty" id="link-is_empty-Description"  class="tablinks active" 
    onclick="openTab(event, 'is_empty', 'Description')">
Description
</button>
</div>

<div group="is_empty" id="is_empty-Description" class="tabcontent"  style="display: block;" >
Return true if the array is empty.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> is_even </h2>

```rust,ignore
fn is_even(x: u16) -> bool
fn is_even(x: u8) -> bool
fn is_even(x: u128) -> bool
fn is_even(x: u64) -> bool
fn is_even(x: i8) -> bool
fn is_even(x: i16) -> bool
fn is_even(x: u32) -> bool
fn is_even(x: int) -> bool
fn is_even(x: i32) -> bool
fn is_even(x: i128) -> bool
```

<div>
<div class="tab">
<button group="is_even" id="link-is_even-Description"  class="tablinks active" 
    onclick="openTab(event, 'is_even', 'Description')">
Description
</button>
</div>

<div group="is_even" id="is_even-Description" class="tabcontent"  style="display: block;" >
Return true if the number is even.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> is_exclusive </h2>

```rust,ignore
fn is_exclusive(range: RangeInclusive<int>) -> bool
fn is_exclusive(range: Range<int>) -> bool
```

<div>
<div class="tab">
<button group="is_exclusive" id="link-is_exclusive-Description"  class="tablinks active" 
    onclick="openTab(event, 'is_exclusive', 'Description')">
Description
</button>
</div>

<div group="is_exclusive" id="is_exclusive-Description" class="tabcontent"  style="display: block;" >
Return `true` if the range is exclusive.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> is_finite </h2>

```rust,ignore
fn is_finite(x: float) -> bool
```

<div>
<div class="tab">
<button group="is_finite" id="link-is_finite-Description"  class="tablinks active" 
    onclick="openTab(event, 'is_finite', 'Description')">
Description
</button>
</div>

<div group="is_finite" id="is_finite-Description" class="tabcontent"  style="display: block;" >
Return `true` if the floating-point number is finite.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> is_inclusive </h2>

```rust,ignore
fn is_inclusive(range: Range<int>) -> bool
fn is_inclusive(range: RangeInclusive<int>) -> bool
```

<div>
<div class="tab">
<button group="is_inclusive" id="link-is_inclusive-Description"  class="tablinks active" 
    onclick="openTab(event, 'is_inclusive', 'Description')">
Description
</button>
</div>

<div group="is_inclusive" id="is_inclusive-Description" class="tabcontent"  style="display: block;" >
Return `true` if the range is inclusive.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> is_infinite </h2>

```rust,ignore
fn is_infinite(x: float) -> bool
```

<div>
<div class="tab">
<button group="is_infinite" id="link-is_infinite-Description"  class="tablinks active" 
    onclick="openTab(event, 'is_infinite', 'Description')">
Description
</button>
</div>

<div group="is_infinite" id="is_infinite-Description" class="tabcontent"  style="display: block;" >
Return `true` if the floating-point number is infinite.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> is_nan </h2>

```rust,ignore
fn is_nan(x: float) -> bool
```

<div>
<div class="tab">
<button group="is_nan" id="link-is_nan-Description"  class="tablinks active" 
    onclick="openTab(event, 'is_nan', 'Description')">
Description
</button>
</div>

<div group="is_nan" id="is_nan-Description" class="tabcontent"  style="display: block;" >
Return `true` if the floating-point number is `NaN` (Not A Number).
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> is_odd </h2>

```rust,ignore
fn is_odd(x: u32) -> bool
fn is_odd(x: i16) -> bool
fn is_odd(x: u64) -> bool
fn is_odd(x: u8) -> bool
fn is_odd(x: u128) -> bool
fn is_odd(x: i8) -> bool
fn is_odd(x: u16) -> bool
fn is_odd(x: i128) -> bool
fn is_odd(x: i32) -> bool
fn is_odd(x: int) -> bool
```

<div>
<div class="tab">
<button group="is_odd" id="link-is_odd-Description"  class="tablinks active" 
    onclick="openTab(event, 'is_odd', 'Description')">
Description
</button>
</div>

<div group="is_odd" id="is_odd-Description" class="tabcontent"  style="display: block;" >
Return true if the number is odd.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> is_zero </h2>

```rust,ignore
fn is_zero(x: u16) -> bool
fn is_zero(x: u8) -> bool
fn is_zero(x: u128) -> bool
fn is_zero(x: u64) -> bool
fn is_zero(x: i8) -> bool
fn is_zero(x: i16) -> bool
fn is_zero(x: u32) -> bool
fn is_zero(x: int) -> bool
fn is_zero(x: float) -> bool
fn is_zero(x: i32) -> bool
fn is_zero(x: f32) -> bool
fn is_zero(x: i128) -> bool
```

<div>
<div class="tab">
<button group="is_zero" id="link-is_zero-Description"  class="tablinks active" 
    onclick="openTab(event, 'is_zero', 'Description')">
Description
</button>
</div>

<div group="is_zero" id="is_zero-Description" class="tabcontent"  style="display: block;" >
Return true if the number is zero.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> keys </h2>

```rust,ignore
fn keys(map: Map) -> Array
```

<div>
<div class="tab">
<button group="keys" id="link-keys-Description"  class="tablinks active" 
    onclick="openTab(event, 'keys', 'Description')">
Description
</button>
<button group="keys" id="link-keys-Example"  class="tablinks" 
    onclick="openTab(event, 'keys', 'Example')">
Example
</button>
</div>

<div group="keys" id="keys-Description" class="tabcontent"  style="display: block;" >
Return an array with all the property names in the object map.
</div>
<div group="keys" id="keys-Example" class="tabcontent"  style="display: none;" >

```rhai
let m = #{a:1, b:2, c:3};

print(m.keys());        // prints ["a", "b", "c"]
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> len </h2>

```rust,ignore
fn len(array: Array) -> int
fn len(map: Map) -> int
fn len(blob: Blob) -> int
fn len(string: String) -> int
```

<div>
<div class="tab">
<button group="len" id="link-len-Description"  class="tablinks active" 
    onclick="openTab(event, 'len', 'Description')">
Description
</button>
</div>

<div group="len" id="len-Description" class="tabcontent"  style="display: block;" >
Number of elements in the array.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> ln </h2>

```rust,ignore
fn ln(x: float) -> float
```

<div>
<div class="tab">
<button group="ln" id="link-ln-Description"  class="tablinks active" 
    onclick="openTab(event, 'ln', 'Description')">
Description
</button>
</div>

<div group="ln" id="ln-Description" class="tabcontent"  style="display: block;" >
Return the natural log of the floating-point number.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> log </h2>

```rust,ignore
fn log(x: float) -> float
fn log(x: float, base: float) -> float
```

<div>
<div class="tab">
<button group="log" id="link-log-Description"  class="tablinks active" 
    onclick="openTab(event, 'log', 'Description')">
Description
</button>
</div>

<div group="log" id="log-Description" class="tabcontent"  style="display: block;" >
Return the log of the floating-point number with base 10.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> make_lower </h2>

```rust,ignore
fn make_lower(string: String)
fn make_lower(character: char)
```

<div>
<div class="tab">
<button group="make_lower" id="link-make_lower-Description"  class="tablinks active" 
    onclick="openTab(event, 'make_lower', 'Description')">
Description
</button>
<button group="make_lower" id="link-make_lower-Example"  class="tablinks" 
    onclick="openTab(event, 'make_lower', 'Example')">
Example
</button>
</div>

<div group="make_lower" id="make_lower-Description" class="tabcontent"  style="display: block;" >
Convert the string to all lower-case.
</div>
<div group="make_lower" id="make_lower-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "HELLO, WORLD!"

text.make_lower();

print(text);        // prints "hello, world!";
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> make_upper </h2>

```rust,ignore
fn make_upper(character: char)
fn make_upper(string: String)
```

<div>
<div class="tab">
<button group="make_upper" id="link-make_upper-Description"  class="tablinks active" 
    onclick="openTab(event, 'make_upper', 'Description')">
Description
</button>
<button group="make_upper" id="link-make_upper-Example"  class="tablinks" 
    onclick="openTab(event, 'make_upper', 'Example')">
Example
</button>
</div>

<div group="make_upper" id="make_upper-Description" class="tabcontent"  style="display: block;" >
Convert the character to upper-case.
</div>
<div group="make_upper" id="make_upper-Example" class="tabcontent"  style="display: none;" >

```rhai
let ch = 'a';

ch.make_upper();

print(ch);          // prints 'A'
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> map </h2>

```rust,ignore
fn map(array: Array, mapper: String) -> Array
fn map(array: Array, map: FnPtr) -> Array
```

<div>
<div class="tab">
<button group="map" id="link-map-Description"  class="tablinks active" 
    onclick="openTab(event, 'map', 'Description')">
Description
</button>
<button group="map" id="link-map-Deprecated API"  class="tablinks" 
    onclick="openTab(event, 'map', 'Deprecated API')">
Deprecated API
</button>
<button group="map" id="link-map-Function Parameters"  class="tablinks" 
    onclick="openTab(event, 'map', 'Function Parameters')">
Function Parameters
</button>
<button group="map" id="link-map-Example"  class="tablinks" 
    onclick="openTab(event, 'map', 'Example')">
Example
</button>
</div>

<div group="map" id="map-Description" class="tabcontent"  style="display: block;" >
Iterate through all the elements in the array, applying a function named by `mapper` to each
element in turn, and return the results as a new array.
</div>
<div group="map" id="map-Deprecated API" class="tabcontent"  style="display: none;" >

This method is deprecated and will be removed from the next major version.
Use `array.map(Fn("fn_name"))` instead.
</div>
<div group="map" id="map-Function Parameters" class="tabcontent"  style="display: none;" >

A function with the same name as the value of `mapper` must exist taking these parameters:

* `element`: copy of array element
* `index` _(optional)_: current index in the array
</div>
<div group="map" id="map-Example" class="tabcontent"  style="display: none;" >

```rhai
fn square(x) { x * x }

fn multiply(x, i) { x * i }

let x = [1, 2, 3, 4, 5];

let y = x.map("square");

print(y);       // prints "[1, 4, 9, 16, 25]"

let y = x.map("multiply");

print(y);       // prints "[0, 2, 6, 12, 20]"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> max </h2>

```rust,ignore
fn max(x: float, y: f32) -> float
fn max(x: int, y: float) -> float
fn max(x: f32, y: float) -> float
fn max(x: u64, y: u64) -> u64
fn max(x: i32, y: i32) -> i32
fn max(x: int, y: f32) -> f32
fn max(x: u8, y: u8) -> u8
fn max(x: u32, y: u32) -> u32
fn max(char1: char, char2: char) -> char
fn max(x: float, y: int) -> float
fn max(x: i128, y: i128) -> i128
fn max(x: i16, y: i16) -> i16
fn max(x: i8, y: i8) -> i8
fn max(x: float, y: float) -> float
fn max(x: f32, y: int) -> f32
fn max(x: u128, y: u128) -> u128
fn max(string1: String, string2: String) -> String
fn max(x: u16, y: u16) -> u16
fn max(x: f32, y: f32) -> f32
fn max(x: int, y: int) -> int
```

<div>
<div class="tab">
<button group="max" id="link-max-Description"  class="tablinks active" 
    onclick="openTab(event, 'max', 'Description')">
Description
</button>
<button group="max" id="link-max-Example"  class="tablinks" 
    onclick="openTab(event, 'max', 'Example')">
Example
</button>
</div>

<div group="max" id="max-Description" class="tabcontent"  style="display: block;" >
Return the character that is lexically greater than the other character.
</div>
<div group="max" id="max-Example" class="tabcontent"  style="display: none;" >

```rhai
max('h', 'w');      // returns 'w'
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> min </h2>

```rust,ignore
fn min(x: float, y: float) -> float
fn min(x: f32, y: int) -> f32
fn min(x: u128, y: u128) -> u128
fn min(string1: String, string2: String) -> String
fn min(x: u16, y: u16) -> u16
fn min(x: f32, y: f32) -> f32
fn min(x: int, y: int) -> int
fn min(x: u32, y: u32) -> u32
fn min(char1: char, char2: char) -> char
fn min(x: float, y: int) -> float
fn min(x: i128, y: i128) -> i128
fn min(x: i16, y: i16) -> i16
fn min(x: i8, y: i8) -> i8
fn min(x: u64, y: u64) -> u64
fn min(x: f32, y: float) -> float
fn min(x: i32, y: i32) -> i32
fn min(x: int, y: f32) -> f32
fn min(x: u8, y: u8) -> u8
fn min(x: float, y: f32) -> float
fn min(x: int, y: float) -> float
```

<div>
<div class="tab">
<button group="min" id="link-min-Description"  class="tablinks active" 
    onclick="openTab(event, 'min', 'Description')">
Description
</button>
<button group="min" id="link-min-Example"  class="tablinks" 
    onclick="openTab(event, 'min', 'Example')">
Example
</button>
</div>

<div group="min" id="min-Description" class="tabcontent"  style="display: block;" >
Return the string that is lexically smaller than the other string.
</div>
<div group="min" id="min-Example" class="tabcontent"  style="display: none;" >

```rhai
min("hello", "world");      // returns "hello"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> mixin </h2>

```rust,ignore
fn mixin(map: Map, map2: Map)
```

<div>
<div class="tab">
<button group="mixin" id="link-mixin-Description"  class="tablinks active" 
    onclick="openTab(event, 'mixin', 'Description')">
Description
</button>
<button group="mixin" id="link-mixin-Example"  class="tablinks" 
    onclick="openTab(event, 'mixin', 'Example')">
Example
</button>
</div>

<div group="mixin" id="mixin-Description" class="tabcontent"  style="display: block;" >
Add all property values of another object map into the object map.
Existing property values of the same names are replaced.
</div>
<div group="mixin" id="mixin-Example" class="tabcontent"  style="display: none;" >

```rhai
let m = #{a:1, b:2, c:3};
let n = #{a: 42, d:0};

m.mixin(n);

print(m);       // prints "#{a:42, b:2, c:3, d:0}"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> name </h2>

```rust,ignore
fn name(fn_ptr: FnPtr) -> String
```

<div>
<div class="tab">
<button group="name" id="link-name-Description"  class="tablinks active" 
    onclick="openTab(event, 'name', 'Description')">
Description
</button>
<button group="name" id="link-name-Example"  class="tablinks" 
    onclick="openTab(event, 'name', 'Example')">
Example
</button>
</div>

<div group="name" id="name-Description" class="tabcontent"  style="display: block;" >
Return the name of the function.
</div>
<div group="name" id="name-Example" class="tabcontent"  style="display: none;" >

```rhai
fn double(x) { x * 2 }

let f = Fn("double");

print(f.name);      // prints "double"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> pad </h2>

```rust,ignore
fn pad(string: String, len: int, character: char)
fn pad(string: String, len: int, padding: String)
fn pad(array: Array, len: int, item: ?)
fn pad(blob: Blob, len: int, value: int)
```

<div>
<div class="tab">
<button group="pad" id="link-pad-Description"  class="tablinks active" 
    onclick="openTab(event, 'pad', 'Description')">
Description
</button>
<button group="pad" id="link-pad-Example"  class="tablinks" 
    onclick="openTab(event, 'pad', 'Example')">
Example
</button>
</div>

<div group="pad" id="pad-Description" class="tabcontent"  style="display: block;" >
Pad the string to at least the specified number of characters with the specified `character`.

If `len` ≤ length of string, no padding is done.
</div>
<div group="pad" id="pad-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "hello";

text.pad(8, '!');

print(text);        // prints "hello!!!"

text.pad(5, '*');

print(text);        // prints "hello!!!"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> parse_be_float </h2>

```rust,ignore
fn parse_be_float(blob: Blob, range: Range<int>) -> float
fn parse_be_float(blob: Blob, range: RangeInclusive<int>) -> float
fn parse_be_float(blob: Blob, start: int, len: int) -> float
```

<div>
<div class="tab">
<button group="parse_be_float" id="link-parse_be_float-Description"  class="tablinks active" 
    onclick="openTab(event, 'parse_be_float', 'Description')">
Description
</button>
</div>

<div group="parse_be_float" id="parse_be_float-Description" class="tabcontent"  style="display: block;" >
Parse the bytes within an exclusive `range` in the BLOB as a `FLOAT`
in big-endian byte order.

* If number of bytes in `range` < number of bytes for `FLOAT`, zeros are padded.
* If number of bytes in `range` > number of bytes for `FLOAT`, extra bytes are ignored.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> parse_be_int </h2>

```rust,ignore
fn parse_be_int(blob: Blob, range: RangeInclusive<int>) -> int
fn parse_be_int(blob: Blob, range: Range<int>) -> int
fn parse_be_int(blob: Blob, start: int, len: int) -> int
```

<div>
<div class="tab">
<button group="parse_be_int" id="link-parse_be_int-Description"  class="tablinks active" 
    onclick="openTab(event, 'parse_be_int', 'Description')">
Description
</button>
</div>

<div group="parse_be_int" id="parse_be_int-Description" class="tabcontent"  style="display: block;" >
Parse the bytes within an inclusive `range` in the BLOB as an `INT`
in big-endian byte order.

* If number of bytes in `range` < number of bytes for `INT`, zeros are padded.
* If number of bytes in `range` > number of bytes for `INT`, extra bytes are ignored.

```rhai
let b = blob();

b += 1; b += 2; b += 3; b += 4; b += 5;

let x = b.parse_be_int(1..=3);  // parse three bytes

print(x.to_hex());              // prints "0203040000...00"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> parse_float </h2>

```rust,ignore
fn parse_float(string: String) -> float
```

<div>
<div class="tab">
<button group="parse_float" id="link-parse_float-Description"  class="tablinks active" 
    onclick="openTab(event, 'parse_float', 'Description')">
Description
</button>
<button group="parse_float" id="link-parse_float-Example"  class="tablinks" 
    onclick="openTab(event, 'parse_float', 'Example')">
Example
</button>
</div>

<div group="parse_float" id="parse_float-Description" class="tabcontent"  style="display: block;" >
Parse a string into a floating-point number.
</div>
<div group="parse_float" id="parse_float-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = parse_int("123.456");

print(x);       // prints 123.456
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> parse_int </h2>

```rust,ignore
fn parse_int(string: String) -> int
fn parse_int(string: String, radix: int) -> int
```

<div>
<div class="tab">
<button group="parse_int" id="link-parse_int-Description"  class="tablinks active" 
    onclick="openTab(event, 'parse_int', 'Description')">
Description
</button>
<button group="parse_int" id="link-parse_int-Example"  class="tablinks" 
    onclick="openTab(event, 'parse_int', 'Example')">
Example
</button>
</div>

<div group="parse_int" id="parse_int-Description" class="tabcontent"  style="display: block;" >
Parse a string into an integer number.
</div>
<div group="parse_int" id="parse_int-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = parse_int("123");

print(x);       // prints 123
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> parse_json </h2>

```rust,ignore
fn parse_json(json: String) -> ?
```

<div>
<div class="tab">
<button group="parse_json" id="link-parse_json-Description"  class="tablinks active" 
    onclick="openTab(event, 'parse_json', 'Description')">
Description
</button>
<button group="parse_json" id="link-parse_json-Example"  class="tablinks" 
    onclick="openTab(event, 'parse_json', 'Example')">
Example
</button>
</div>

<div group="parse_json" id="parse_json-Description" class="tabcontent"  style="display: block;" >
Parse a JSON string into a value.
</div>
<div group="parse_json" id="parse_json-Example" class="tabcontent"  style="display: none;" >

```rhai
let m = parse_json(`{"a":1, "b":2, "c":3}`);

print(m);       // prints #{"a":1, "b":2, "c":3}
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> parse_le_float </h2>

```rust,ignore
fn parse_le_float(blob: Blob, range: RangeInclusive<int>) -> float
fn parse_le_float(blob: Blob, range: Range<int>) -> float
fn parse_le_float(blob: Blob, start: int, len: int) -> float
```

<div>
<div class="tab">
<button group="parse_le_float" id="link-parse_le_float-Description"  class="tablinks active" 
    onclick="openTab(event, 'parse_le_float', 'Description')">
Description
</button>
</div>

<div group="parse_le_float" id="parse_le_float-Description" class="tabcontent"  style="display: block;" >
Parse the bytes within an inclusive `range` in the BLOB as a `FLOAT`
in little-endian byte order.

* If number of bytes in `range` < number of bytes for `FLOAT`, zeros are padded.
* If number of bytes in `range` > number of bytes for `FLOAT`, extra bytes are ignored.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> parse_le_int </h2>

```rust,ignore
fn parse_le_int(blob: Blob, range: Range<int>) -> int
fn parse_le_int(blob: Blob, range: RangeInclusive<int>) -> int
fn parse_le_int(blob: Blob, start: int, len: int) -> int
```

<div>
<div class="tab">
<button group="parse_le_int" id="link-parse_le_int-Description"  class="tablinks active" 
    onclick="openTab(event, 'parse_le_int', 'Description')">
Description
</button>
</div>

<div group="parse_le_int" id="parse_le_int-Description" class="tabcontent"  style="display: block;" >
Parse the bytes within an exclusive `range` in the BLOB as an `INT`
in little-endian byte order.

* If number of bytes in `range` < number of bytes for `INT`, zeros are padded.
* If number of bytes in `range` > number of bytes for `INT`, extra bytes are ignored.

```rhai
let b = blob();

b += 1; b += 2; b += 3; b += 4; b += 5;

let x = b.parse_le_int(1..3);   // parse two bytes

print(x.to_hex());              // prints "0302"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> pop </h2>

```rust,ignore
fn pop(blob: Blob) -> int
fn pop(string: String) -> ?
fn pop(array: Array) -> ?
fn pop(string: String, len: int) -> String
```

<div>
<div class="tab">
<button group="pop" id="link-pop-Description"  class="tablinks active" 
    onclick="openTab(event, 'pop', 'Description')">
Description
</button>
<button group="pop" id="link-pop-Example"  class="tablinks" 
    onclick="openTab(event, 'pop', 'Example')">
Example
</button>
</div>

<div group="pop" id="pop-Description" class="tabcontent"  style="display: block;" >
Remove the last byte from the BLOB and return it.

If the BLOB is empty, zero is returned.
</div>
<div group="pop" id="pop-Example" class="tabcontent"  style="display: none;" >

```rhai
let b = blob();

b += 1; b += 2; b += 3; b += 4; b += 5;

print(b.pop());         // prints 5

print(b);               // prints "[01020304]"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> print </h2>

```rust,ignore
fn print() -> String
fn print(string: String) -> String
fn print(number: float) -> String
fn print(unit: ?) -> String
fn print(number: f32) -> String
fn print(value: bool) -> String
fn print(item: ?) -> String
fn print(character: char) -> String
fn print(map: Map) -> String
fn print(array: Array) -> String
```

<div>
<div class="tab">
<button group="print" id="link-print-Description"  class="tablinks active" 
    onclick="openTab(event, 'print', 'Description')">
Description
</button>
</div>

<div group="print" id="print-Description" class="tabcontent"  style="display: block;" >
Return the empty string.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> push </h2>

```rust,ignore
fn push(blob: Blob, value: int)
fn push(array: Array, item: ?)
```

<div>
<div class="tab">
<button group="push" id="link-push-Description"  class="tablinks active" 
    onclick="openTab(event, 'push', 'Description')">
Description
</button>
<button group="push" id="link-push-Example"  class="tablinks" 
    onclick="openTab(event, 'push', 'Example')">
Example
</button>
</div>

<div group="push" id="push-Description" class="tabcontent"  style="display: block;" >
Add a new byte `value` to the end of the BLOB.

Only the lower 8 bits of the `value` are used; all other bits are ignored.
</div>
<div group="push" id="push-Example" class="tabcontent"  style="display: none;" >

```rhai
let b = blob();

b.push(0x42);

print(b);       // prints "[42]"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> range </h2>

```rust,ignore
fn range(from: u64, to: u64) -> Range<u64>
fn range(from: i32, to: i32) -> Range<i32>
fn range(range: Range<int>, step: int) -> StepRange<int>
fn range(from: u8, to: u8) -> Range<u8>
fn range(range: Range<i16>, step: i16) -> StepRange<i16>
fn range(range: Range<u128>, step: u128) -> StepRange<u128>
fn range(range: Range<u16>, step: u16) -> StepRange<u16>
fn range(range: Range<u64>, step: u64) -> StepRange<u64>
fn range(from: u128, to: u128) -> Range<u128>
fn range(from: int, to: int) -> Range<int>
fn range(from: u16, to: u16) -> Range<u16>
fn range(from: u32, to: u32) -> Range<u32>
fn range(range: Range<i128>, step: i128) -> StepRange<i128>
fn range(range: Range<i32>, step: i32) -> StepRange<i32>
fn range(from: i128, to: i128) -> Range<i128>
fn range(range: Range<u32>, step: u32) -> StepRange<u32>
fn range(range: Range<u8>, step: u8) -> StepRange<u8>
fn range(from: i16, to: i16) -> Range<i16>
fn range(from: i8, to: i8) -> Range<i8>
fn range(range: Range<i8>, step: i8) -> StepRange<i8>
fn range(range: Range<float>, step: float) -> StepRange<float>
fn range(from: u32, to: u32, step: u32) -> StepRange<u32>
fn range(from: float, to: float, step: float) -> StepRange<float>
fn range(from: i8, to: i8, step: i8) -> StepRange<i8>
fn range(from: u8, to: u8, step: u8) -> StepRange<u8>
fn range(from: u64, to: u64, step: u64) -> StepRange<u64>
fn range(from: u16, to: u16, step: u16) -> StepRange<u16>
fn range(from: int, to: int, step: int) -> StepRange<int>
fn range(from: i32, to: i32, step: i32) -> StepRange<i32>
fn range(from: u128, to: u128, step: u128) -> StepRange<u128>
fn range(from: i16, to: i16, step: i16) -> StepRange<i16>
fn range(from: i128, to: i128, step: i128) -> StepRange<i128>
```

<div>
<div class="tab">
<button group="range" id="link-range-Description"  class="tablinks active" 
    onclick="openTab(event, 'range', 'Description')">
Description
</button>
<button group="range" id="link-range-Example"  class="tablinks" 
    onclick="openTab(event, 'range', 'Example')">
Example
</button>
</div>

<div group="range" id="range-Description" class="tabcontent"  style="display: block;" >
Return an iterator over the exclusive range of `from..to`.
The value `to` is never included.
</div>
<div group="range" id="range-Example" class="tabcontent"  style="display: none;" >

```rhai
// prints all values from 8 to 17
for n in range(8, 18) {
    print(n);
}
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> reduce </h2>

```rust,ignore
fn reduce(array: Array, reducer: String) -> ?
fn reduce(array: Array, reducer: FnPtr) -> ?
fn reduce(array: Array, reducer: String, initial: ?) -> ?
fn reduce(array: Array, reducer: FnPtr, initial: ?) -> ?
```

<div>
<div class="tab">
<button group="reduce" id="link-reduce-Description"  class="tablinks active" 
    onclick="openTab(event, 'reduce', 'Description')">
Description
</button>
<button group="reduce" id="link-reduce-Deprecated API"  class="tablinks" 
    onclick="openTab(event, 'reduce', 'Deprecated API')">
Deprecated API
</button>
<button group="reduce" id="link-reduce-Function Parameters"  class="tablinks" 
    onclick="openTab(event, 'reduce', 'Function Parameters')">
Function Parameters
</button>
<button group="reduce" id="link-reduce-Example"  class="tablinks" 
    onclick="openTab(event, 'reduce', 'Example')">
Example
</button>
</div>

<div group="reduce" id="reduce-Description" class="tabcontent"  style="display: block;" >
Reduce an array by iterating through all elements while applying a function named by `reducer`.
</div>
<div group="reduce" id="reduce-Deprecated API" class="tabcontent"  style="display: none;" >

This method is deprecated and will be removed from the next major version.
Use `array.reduce(Fn("fn_name"))` instead.
</div>
<div group="reduce" id="reduce-Function Parameters" class="tabcontent"  style="display: none;" >

A function with the same name as the value of `reducer` must exist taking these parameters:

* `result`: accumulated result, initially `()`
* `element`: copy of array element
* `index` _(optional)_: current index in the array
</div>
<div group="reduce" id="reduce-Example" class="tabcontent"  style="display: none;" >

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
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> reduce_rev </h2>

```rust,ignore
fn reduce_rev(array: Array, reducer: String) -> ?
fn reduce_rev(array: Array, reducer: FnPtr) -> ?
fn reduce_rev(array: Array, reducer: String, initial: ?) -> ?
fn reduce_rev(array: Array, reducer: FnPtr, initial: ?) -> ?
```

<div>
<div class="tab">
<button group="reduce_rev" id="link-reduce_rev-Description"  class="tablinks active" 
    onclick="openTab(event, 'reduce_rev', 'Description')">
Description
</button>
<button group="reduce_rev" id="link-reduce_rev-Deprecated API"  class="tablinks" 
    onclick="openTab(event, 'reduce_rev', 'Deprecated API')">
Deprecated API
</button>
<button group="reduce_rev" id="link-reduce_rev-Function Parameters"  class="tablinks" 
    onclick="openTab(event, 'reduce_rev', 'Function Parameters')">
Function Parameters
</button>
<button group="reduce_rev" id="link-reduce_rev-Example"  class="tablinks" 
    onclick="openTab(event, 'reduce_rev', 'Example')">
Example
</button>
</div>

<div group="reduce_rev" id="reduce_rev-Description" class="tabcontent"  style="display: block;" >
Reduce an array by iterating through all elements, in _reverse_ order,
while applying a function named by `reducer`.
</div>
<div group="reduce_rev" id="reduce_rev-Deprecated API" class="tabcontent"  style="display: none;" >

This method is deprecated and will be removed from the next major version.
Use `array.reduce_rev(Fn("fn_name"))` instead.
</div>
<div group="reduce_rev" id="reduce_rev-Function Parameters" class="tabcontent"  style="display: none;" >

A function with the same name as the value of `reducer` must exist taking these parameters:

* `result`: accumulated result, initially `()`
* `element`: copy of array element
* `index` _(optional)_: current index in the array
</div>
<div group="reduce_rev" id="reduce_rev-Example" class="tabcontent"  style="display: none;" >

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
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> remove </h2>

```rust,ignore
fn remove(string: String, sub_string: String)
fn remove(array: Array, index: int) -> ?
fn remove(string: String, character: char)
fn remove(map: Map, property: String) -> ?
fn remove(blob: Blob, index: int) -> int
```

<div>
<div class="tab">
<button group="remove" id="link-remove-Description"  class="tablinks active" 
    onclick="openTab(event, 'remove', 'Description')">
Description
</button>
<button group="remove" id="link-remove-Example"  class="tablinks" 
    onclick="openTab(event, 'remove', 'Example')">
Example
</button>
</div>

<div group="remove" id="remove-Description" class="tabcontent"  style="display: block;" >
Remove all occurrences of a sub-string from the string.
</div>
<div group="remove" id="remove-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "hello, world! hello, foobar!";

text.remove("hello");

print(text);        // prints ", world! , foobar!"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> replace </h2>

```rust,ignore
fn replace(string: String, find_character: char, substitute_character: char)
fn replace(string: String, find_string: String, substitute_character: char)
fn replace(string: String, find_character: char, substitute_string: String)
fn replace(string: String, find_string: String, substitute_string: String)
```

<div>
<div class="tab">
<button group="replace" id="link-replace-Description"  class="tablinks active" 
    onclick="openTab(event, 'replace', 'Description')">
Description
</button>
<button group="replace" id="link-replace-Example"  class="tablinks" 
    onclick="openTab(event, 'replace', 'Example')">
Example
</button>
</div>

<div group="replace" id="replace-Description" class="tabcontent"  style="display: block;" >
Replace all occurrences of the specified character in the string with another character.
</div>
<div group="replace" id="replace-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "hello, world! hello, foobar!";

text.replace("l", '*');

print(text);        // prints "he**o, wor*d! he**o, foobar!"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> retain </h2>

```rust,ignore
fn retain(array: Array, range: Range<int>) -> Array
fn retain(blob: Blob, range: Range<int>) -> Blob
fn retain(array: Array, filter: FnPtr) -> Array
fn retain(array: Array, range: RangeInclusive<int>) -> Array
fn retain(map: Map, filter: FnPtr) -> Map
fn retain(array: Array, filter: String) -> Array
fn retain(blob: Blob, range: RangeInclusive<int>) -> Blob
fn retain(array: Array, start: int, len: int) -> Array
fn retain(blob: Blob, start: int, len: int) -> Blob
```

<div>
<div class="tab">
<button group="retain" id="link-retain-Description"  class="tablinks active" 
    onclick="openTab(event, 'retain', 'Description')">
Description
</button>
<button group="retain" id="link-retain-Example"  class="tablinks" 
    onclick="openTab(event, 'retain', 'Example')">
Example
</button>
</div>

<div group="retain" id="retain-Description" class="tabcontent"  style="display: block;" >
Remove all elements in the array not within an exclusive `range` and return them as a new array.
</div>
<div group="retain" id="retain-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = [1, 2, 3, 4, 5];

let y = x.retain(1..4);

print(x);       // prints "[2, 3, 4]"

print(y);       // prints "[1, 5]"

let z = x.retain(1..3);

print(x);       // prints "[3, 4]"

print(z);       // prints "[1]"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> reverse </h2>

```rust,ignore
fn reverse(blob: Blob)
fn reverse(array: Array)
```

<div>
<div class="tab">
<button group="reverse" id="link-reverse-Description"  class="tablinks active" 
    onclick="openTab(event, 'reverse', 'Description')">
Description
</button>
<button group="reverse" id="link-reverse-Example"  class="tablinks" 
    onclick="openTab(event, 'reverse', 'Example')">
Example
</button>
</div>

<div group="reverse" id="reverse-Description" class="tabcontent"  style="display: block;" >
Reverse the BLOB.
</div>
<div group="reverse" id="reverse-Example" class="tabcontent"  style="display: none;" >

```rhai
let b = blob();

b += 1; b += 2; b += 3; b += 4; b += 5;

print(b);           // prints "[0102030405]"

b.reverse();

print(b);           // prints "[0504030201]"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> round </h2>

```rust,ignore
fn round(x: float) -> float
```

<div>
<div class="tab">
<button group="round" id="link-round-Description"  class="tablinks active" 
    onclick="openTab(event, 'round', 'Description')">
Description
</button>
</div>

<div group="round" id="round-Description" class="tabcontent"  style="display: block;" >
Return the nearest whole number closest to the floating-point number.
Rounds away from zero.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> set </h2>

```rust,ignore
fn set(map: Map, property: String, value: ?)
fn set(string: String, index: int, character: char)
fn set(array: Array, index: int, value: ?)
fn set(blob: Blob, index: int, value: int)
```

<div>
<div class="tab">
<button group="set" id="link-set-Description"  class="tablinks active" 
    onclick="openTab(event, 'set', 'Description')">
Description
</button>
<button group="set" id="link-set-Example"  class="tablinks" 
    onclick="openTab(event, 'set', 'Example')">
Example
</button>
</div>

<div group="set" id="set-Description" class="tabcontent"  style="display: block;" >
Set the value of the `property` in the object map to a new `value`.

If `property` does not exist in the object map, it is added.
</div>
<div group="set" id="set-Example" class="tabcontent"  style="display: none;" >

```rhai
let m = #{a: 1, b: 2, c: 3};

m.set("b", 42)'

print(m);           // prints "#{a: 1, b: 42, c: 3}"

x.set("x", 0);

print(m);           // prints "#{a: 1, b: 42, c: 3, x: 0}"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> set_bit </h2>

```rust,ignore
fn set_bit(value: int, bit: int, new_value: bool)
```

<div>
<div class="tab">
<button group="set_bit" id="link-set_bit-Description"  class="tablinks active" 
    onclick="openTab(event, 'set_bit', 'Description')">
Description
</button>
<button group="set_bit" id="link-set_bit-Example"  class="tablinks" 
    onclick="openTab(event, 'set_bit', 'Example')">
Example
</button>
</div>

<div group="set_bit" id="set_bit-Description" class="tabcontent"  style="display: block;" >
Set the specified `bit` in the number if the new value is `true`.
Clear the `bit` if the new value is `false`.

If `bit` < 0, position counts from the MSB (Most Significant Bit).
</div>
<div group="set_bit" id="set_bit-Example" class="tabcontent"  style="display: none;" >

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
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> set_bits </h2>

```rust,ignore
fn set_bits(value: int, range: RangeInclusive<int>, new_value: int)
fn set_bits(value: int, range: Range<int>, new_value: int)
fn set_bits(value: int, bit: int, bits: int, new_value: int)
```

<div>
<div class="tab">
<button group="set_bits" id="link-set_bits-Description"  class="tablinks active" 
    onclick="openTab(event, 'set_bits', 'Description')">
Description
</button>
<button group="set_bits" id="link-set_bits-Example"  class="tablinks" 
    onclick="openTab(event, 'set_bits', 'Example')">
Example
</button>
</div>

<div group="set_bits" id="set_bits-Description" class="tabcontent"  style="display: block;" >
Replace an inclusive range of bits in the number with a new value.
</div>
<div group="set_bits" id="set_bits-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = 123456;

x.set_bits(5..=9, 42);

print(x);           // print 123200
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> set_tag </h2>

```rust,ignore
fn set_tag(value: ?, tag: int)
```

<div>
<div class="tab">
<button group="set_tag" id="link-set_tag-Description"  class="tablinks active" 
    onclick="openTab(event, 'set_tag', 'Description')">
Description
</button>
<button group="set_tag" id="link-set_tag-Example"  class="tablinks" 
    onclick="openTab(event, 'set_tag', 'Example')">
Example
</button>
</div>

<div group="set_tag" id="set_tag-Description" class="tabcontent"  style="display: block;" >
Set the _tag_ of a `Dynamic` value.
</div>
<div group="set_tag" id="set_tag-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = "hello, world!";

x.tag = 42;

print(x.tag);           // prints 42
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> shift </h2>

```rust,ignore
fn shift(blob: Blob) -> int
fn shift(array: Array) -> ?
```

<div>
<div class="tab">
<button group="shift" id="link-shift-Description"  class="tablinks active" 
    onclick="openTab(event, 'shift', 'Description')">
Description
</button>
<button group="shift" id="link-shift-Example"  class="tablinks" 
    onclick="openTab(event, 'shift', 'Example')">
Example
</button>
</div>

<div group="shift" id="shift-Description" class="tabcontent"  style="display: block;" >
Remove the first byte from the BLOB and return it.

If the BLOB is empty, zero is returned.
</div>
<div group="shift" id="shift-Example" class="tabcontent"  style="display: none;" >

```rhai
let b = blob();

b += 1; b += 2; b += 3; b += 4; b += 5;

print(b.shift());       // prints 1

print(b);               // prints "[02030405]"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> sign </h2>

```rust,ignore
fn sign(x: float) -> int
fn sign(x: int) -> int
fn sign(x: f32) -> int
fn sign(x: i128) -> int
fn sign(x: i32) -> int
fn sign(x: i8) -> int
fn sign(x: i16) -> int
```

<div>
<div class="tab">
<button group="sign" id="link-sign-Description"  class="tablinks active" 
    onclick="openTab(event, 'sign', 'Description')">
Description
</button>
</div>

<div group="sign" id="sign-Description" class="tabcontent"  style="display: block;" >
Return the sign (as an integer) of the floating-point number according to the following:

* `0` if the number is zero
* `1` if the number is positive
* `-1` if the number is negative
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> sin </h2>

```rust,ignore
fn sin(x: float) -> float
```

<div>
<div class="tab">
<button group="sin" id="link-sin-Description"  class="tablinks active" 
    onclick="openTab(event, 'sin', 'Description')">
Description
</button>
</div>

<div group="sin" id="sin-Description" class="tabcontent"  style="display: block;" >
Return the sine of the floating-point number in radians.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> sinh </h2>

```rust,ignore
fn sinh(x: float) -> float
```

<div>
<div class="tab">
<button group="sinh" id="link-sinh-Description"  class="tablinks active" 
    onclick="openTab(event, 'sinh', 'Description')">
Description
</button>
</div>

<div group="sinh" id="sinh-Description" class="tabcontent"  style="display: block;" >
Return the hyperbolic sine of the floating-point number in radians.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> sleep </h2>

```rust,ignore
fn sleep(seconds: float)
fn sleep(seconds: int)
```

<div>
<div class="tab">
<button group="sleep" id="link-sleep-Description"  class="tablinks active" 
    onclick="openTab(event, 'sleep', 'Description')">
Description
</button>
<button group="sleep" id="link-sleep-Example"  class="tablinks" 
    onclick="openTab(event, 'sleep', 'Example')">
Example
</button>
</div>

<div group="sleep" id="sleep-Description" class="tabcontent"  style="display: block;" >
Block the current thread for a particular number of `seconds`.
</div>
<div group="sleep" id="sleep-Example" class="tabcontent"  style="display: none;" >

```rhai
// Do nothing for 10 seconds!
sleep(10.0);
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> some </h2>

```rust,ignore
fn some(array: Array, filter: String) -> bool
fn some(array: Array, filter: FnPtr) -> bool
```

<div>
<div class="tab">
<button group="some" id="link-some-Description"  class="tablinks active" 
    onclick="openTab(event, 'some', 'Description')">
Description
</button>
<button group="some" id="link-some-Deprecated API"  class="tablinks" 
    onclick="openTab(event, 'some', 'Deprecated API')">
Deprecated API
</button>
<button group="some" id="link-some-Function Parameters"  class="tablinks" 
    onclick="openTab(event, 'some', 'Function Parameters')">
Function Parameters
</button>
<button group="some" id="link-some-Example"  class="tablinks" 
    onclick="openTab(event, 'some', 'Example')">
Example
</button>
</div>

<div group="some" id="some-Description" class="tabcontent"  style="display: block;" >
Return `true` if any element in the array that returns `true` when applied a function named
by `filter`.
</div>
<div group="some" id="some-Deprecated API" class="tabcontent"  style="display: none;" >

This method is deprecated and will be removed from the next major version.
Use `array.some(Fn("fn_name"))` instead.
</div>
<div group="some" id="some-Function Parameters" class="tabcontent"  style="display: none;" >

A function with the same name as the value of `filter` must exist taking these parameters:

* `element`: copy of array element
* `index` _(optional)_: current index in the array
</div>
<div group="some" id="some-Example" class="tabcontent"  style="display: none;" >

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
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> sort </h2>

```rust,ignore
fn sort(array: Array)
fn sort(array: Array, comparer: FnPtr)
fn sort(array: Array, comparer: String)
```

<div>
<div class="tab">
<button group="sort" id="link-sort-Description"  class="tablinks active" 
    onclick="openTab(event, 'sort', 'Description')">
Description
</button>
<button group="sort" id="link-sort-Supported Data Types"  class="tablinks" 
    onclick="openTab(event, 'sort', 'Supported Data Types')">
Supported Data Types
</button>
<button group="sort" id="link-sort-Example"  class="tablinks" 
    onclick="openTab(event, 'sort', 'Example')">
Example
</button>
</div>

<div group="sort" id="sort-Description" class="tabcontent"  style="display: block;" >
Sort the array.

All elements in the array must be of the same data type.
</div>
<div group="sort" id="sort-Supported Data Types" class="tabcontent"  style="display: none;" >

* integer numbers
* floating-point numbers
* decimal numbers
* characters
* strings
* booleans
* `()`
</div>
<div group="sort" id="sort-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = [1, 3, 5, 7, 9, 2, 4, 6, 8, 10];

x.sort();

print(x);       // prints "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> splice </h2>

```rust,ignore
fn splice(array: Array, range: RangeInclusive<int>, replace: Array)
fn splice(array: Array, range: Range<int>, replace: Array)
fn splice(blob: Blob, range: RangeInclusive<int>, replace: Blob)
fn splice(blob: Blob, range: Range<int>, replace: Blob)
fn splice(array: Array, start: int, len: int, replace: Array)
fn splice(blob: Blob, start: int, len: int, replace: Blob)
```

<div>
<div class="tab">
<button group="splice" id="link-splice-Description"  class="tablinks active" 
    onclick="openTab(event, 'splice', 'Description')">
Description
</button>
<button group="splice" id="link-splice-Example"  class="tablinks" 
    onclick="openTab(event, 'splice', 'Example')">
Example
</button>
</div>

<div group="splice" id="splice-Description" class="tabcontent"  style="display: block;" >
Replace an inclusive range of the array with another array.
</div>
<div group="splice" id="splice-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = [1, 2, 3, 4, 5];
let y = [7, 8, 9, 10];

x.splice(1..=3, y);

print(x);       // prints "[1, 7, 8, 9, 10, 5]"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> split </h2>

```rust,ignore
fn split(string: String) -> Array
fn split(blob: Blob, index: int) -> Blob
fn split(string: String, delimiter: String) -> Array
fn split(array: Array, index: int) -> Array
fn split(string: String, delimiter: char) -> Array
fn split(string: String, index: int) -> Array
fn split(string: String, delimiter: char, segments: int) -> Array
fn split(string: String, delimiter: String, segments: int) -> Array
```

<div>
<div class="tab">
<button group="split" id="link-split-Description"  class="tablinks active" 
    onclick="openTab(event, 'split', 'Description')">
Description
</button>
<button group="split" id="link-split-Example"  class="tablinks" 
    onclick="openTab(event, 'split', 'Example')">
Example
</button>
</div>

<div group="split" id="split-Description" class="tabcontent"  style="display: block;" >
Split the string into segments based on whitespaces, returning an array of the segments.
</div>
<div group="split" id="split-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "hello, world! hello, foo!";

print(text.split());        // prints ["hello,", "world!", "hello,", "foo!"]
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> split_rev </h2>

```rust,ignore
fn split_rev(string: String, delimiter: char) -> Array
fn split_rev(string: String, delimiter: String) -> Array
fn split_rev(string: String, delimiter: String, segments: int) -> Array
fn split_rev(string: String, delimiter: char, segments: int) -> Array
```

<div>
<div class="tab">
<button group="split_rev" id="link-split_rev-Description"  class="tablinks active" 
    onclick="openTab(event, 'split_rev', 'Description')">
Description
</button>
<button group="split_rev" id="link-split_rev-Example"  class="tablinks" 
    onclick="openTab(event, 'split_rev', 'Example')">
Example
</button>
</div>

<div group="split_rev" id="split_rev-Description" class="tabcontent"  style="display: block;" >
Split the string into segments based on a `delimiter` character, returning an array of
the segments in _reverse_ order.
</div>
<div group="split_rev" id="split_rev-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "hello, world! hello, foo!";

print(text.split_rev('l'));     // prints ["o, foo!", "", "d! he", "o, wor", "", "he"]
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> sqrt </h2>

```rust,ignore
fn sqrt(x: float) -> float
```

<div>
<div class="tab">
<button group="sqrt" id="link-sqrt-Description"  class="tablinks active" 
    onclick="openTab(event, 'sqrt', 'Description')">
Description
</button>
</div>

<div group="sqrt" id="sqrt-Description" class="tabcontent"  style="display: block;" >
Return the square root of the floating-point number.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> start </h2>

```rust,ignore
fn start(range: RangeInclusive<int>) -> int
fn start(range: Range<int>) -> int
```

<div>
<div class="tab">
<button group="start" id="link-start-Description"  class="tablinks active" 
    onclick="openTab(event, 'start', 'Description')">
Description
</button>
</div>

<div group="start" id="start-Description" class="tabcontent"  style="display: block;" >
Return the start of the inclusive range.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> starts_with </h2>

```rust,ignore
fn starts_with(string: String, match_string: String) -> bool
```

<div>
<div class="tab">
<button group="starts_with" id="link-starts_with-Description"  class="tablinks active" 
    onclick="openTab(event, 'starts_with', 'Description')">
Description
</button>
<button group="starts_with" id="link-starts_with-Example"  class="tablinks" 
    onclick="openTab(event, 'starts_with', 'Example')">
Example
</button>
</div>

<div group="starts_with" id="starts_with-Description" class="tabcontent"  style="display: block;" >
Return `true` if the string starts with a specified string.
</div>
<div group="starts_with" id="starts_with-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "hello, world!";

print(text.starts_with("hello"));   // prints true

print(text.starts_with("world"));   // prints false
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> sub_string </h2>

```rust,ignore
fn sub_string(string: String, range: RangeInclusive<int>) -> String
fn sub_string(string: String, range: Range<int>) -> String
fn sub_string(string: String, start: int) -> String
fn sub_string(string: String, start: int, len: int) -> String
```

<div>
<div class="tab">
<button group="sub_string" id="link-sub_string-Description"  class="tablinks active" 
    onclick="openTab(event, 'sub_string', 'Description')">
Description
</button>
<button group="sub_string" id="link-sub_string-Example"  class="tablinks" 
    onclick="openTab(event, 'sub_string', 'Example')">
Example
</button>
</div>

<div group="sub_string" id="sub_string-Description" class="tabcontent"  style="display: block;" >
Copy an inclusive range of characters from the string and return it as a new string.
</div>
<div group="sub_string" id="sub_string-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "hello, world!";

print(text.sub_string(3..=7));  // prints "lo, w"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> tag </h2>

```rust,ignore
fn tag(value: ?) -> int
```

<div>
<div class="tab">
<button group="tag" id="link-tag-Description"  class="tablinks active" 
    onclick="openTab(event, 'tag', 'Description')">
Description
</button>
<button group="tag" id="link-tag-Example"  class="tablinks" 
    onclick="openTab(event, 'tag', 'Example')">
Example
</button>
</div>

<div group="tag" id="tag-Description" class="tabcontent"  style="display: block;" >
Return the _tag_ of a `Dynamic` value.
</div>
<div group="tag" id="tag-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = "hello, world!";

x.tag = 42;

print(x.tag);           // prints 42
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> take </h2>

```rust,ignore
fn take(value: ?) -> ?
```

<div>
<div class="tab">
<button group="take" id="link-take-Description"  class="tablinks active" 
    onclick="openTab(event, 'take', 'Description')">
Description
</button>
<button group="take" id="link-take-Example"  class="tablinks" 
    onclick="openTab(event, 'take', 'Example')">
Example
</button>
</div>

<div group="take" id="take-Description" class="tabcontent"  style="display: block;" >
Take ownership of the data in a `Dynamic` value and return it.
The data is _NOT_ cloned.

The original value is replaced with `()`.
</div>
<div group="take" id="take-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = 42;

print(take(x));         // prints 42

print(x);               // prints ()
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> tan </h2>

```rust,ignore
fn tan(x: float) -> float
```

<div>
<div class="tab">
<button group="tan" id="link-tan-Description"  class="tablinks active" 
    onclick="openTab(event, 'tan', 'Description')">
Description
</button>
</div>

<div group="tan" id="tan-Description" class="tabcontent"  style="display: block;" >
Return the tangent of the floating-point number in radians.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> tanh </h2>

```rust,ignore
fn tanh(x: float) -> float
```

<div>
<div class="tab">
<button group="tanh" id="link-tanh-Description"  class="tablinks active" 
    onclick="openTab(event, 'tanh', 'Description')">
Description
</button>
</div>

<div group="tanh" id="tanh-Description" class="tabcontent"  style="display: block;" >
Return the hyperbolic tangent of the floating-point number in radians.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> timestamp </h2>

```rust,ignore
fn timestamp() -> Instant
```

<div>
<div class="tab">
<button group="timestamp" id="link-timestamp-Description"  class="tablinks active" 
    onclick="openTab(event, 'timestamp', 'Description')">
Description
</button>
<button group="timestamp" id="link-timestamp-Example"  class="tablinks" 
    onclick="openTab(event, 'timestamp', 'Example')">
Example
</button>
</div>

<div group="timestamp" id="timestamp-Description" class="tabcontent"  style="display: block;" >
Create a timestamp containing the current system time.
</div>
<div group="timestamp" id="timestamp-Example" class="tabcontent"  style="display: none;" >

```rhai
let now = timestamp();

sleep(10.0);            // sleep for 10 seconds

print(now.elapsed);     // prints 10.???
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> to_array </h2>

```rust,ignore
fn to_array(blob: Blob) -> Array
```

<div>
<div class="tab">
<button group="to_array" id="link-to_array-Description"  class="tablinks active" 
    onclick="openTab(event, 'to_array', 'Description')">
Description
</button>
<button group="to_array" id="link-to_array-Example"  class="tablinks" 
    onclick="openTab(event, 'to_array', 'Example')">
Example
</button>
</div>

<div group="to_array" id="to_array-Description" class="tabcontent"  style="display: block;" >
Convert the BLOB into an array of integers.
</div>
<div group="to_array" id="to_array-Example" class="tabcontent"  style="display: none;" >

```rhai
let b = blob(5, 0x42);

let x = b.to_array();

print(x);       // prints "[66, 66, 66, 66, 66]"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> to_binary </h2>

```rust,ignore
fn to_binary(value: i128) -> String
fn to_binary(value: i32) -> String
fn to_binary(value: int) -> String
fn to_binary(value: u32) -> String
fn to_binary(value: i16) -> String
fn to_binary(value: u64) -> String
fn to_binary(value: u8) -> String
fn to_binary(value: u128) -> String
fn to_binary(value: i8) -> String
fn to_binary(value: u16) -> String
```

<div>
<div class="tab">
<button group="to_binary" id="link-to_binary-Description"  class="tablinks active" 
    onclick="openTab(event, 'to_binary', 'Description')">
Description
</button>
</div>

<div group="to_binary" id="to_binary-Description" class="tabcontent"  style="display: block;" >
Convert the `value` into a string in binary format.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> to_blob </h2>

```rust,ignore
fn to_blob(string: String) -> Blob
```

<div>
<div class="tab">
<button group="to_blob" id="link-to_blob-Description"  class="tablinks active" 
    onclick="openTab(event, 'to_blob', 'Description')">
Description
</button>
<button group="to_blob" id="link-to_blob-Example"  class="tablinks" 
    onclick="openTab(event, 'to_blob', 'Example')">
Example
</button>
</div>

<div group="to_blob" id="to_blob-Description" class="tabcontent"  style="display: block;" >
Convert the string into an UTF-8 encoded byte-stream as a BLOB.
</div>
<div group="to_blob" id="to_blob-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "朝には紅顔ありて夕べには白骨となる";

let bytes = text.to_blob();

print(bytes.len());     // prints 51
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> to_chars </h2>

```rust,ignore
fn to_chars(string: String) -> Array
```

<div>
<div class="tab">
<button group="to_chars" id="link-to_chars-Description"  class="tablinks active" 
    onclick="openTab(event, 'to_chars', 'Description')">
Description
</button>
<button group="to_chars" id="link-to_chars-Example"  class="tablinks" 
    onclick="openTab(event, 'to_chars', 'Example')">
Example
</button>
</div>

<div group="to_chars" id="to_chars-Description" class="tabcontent"  style="display: block;" >
Return an array containing all the characters of the string.
</div>
<div group="to_chars" id="to_chars-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "hello";

print(text.to_chars());     // prints "['h', 'e', 'l', 'l', 'o']"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> to_debug </h2>

```rust,ignore
fn to_debug(value: bool) -> String
fn to_debug(number: f32) -> String
fn to_debug(unit: ?) -> String
fn to_debug(f: FnPtr) -> String
fn to_debug(string: String) -> String
fn to_debug(number: float) -> String
fn to_debug(array: Array) -> String
fn to_debug(map: Map) -> String
fn to_debug(character: char) -> String
fn to_debug(item: ?) -> String
```

<div>
<div class="tab">
<button group="to_debug" id="link-to_debug-Description"  class="tablinks active" 
    onclick="openTab(event, 'to_debug', 'Description')">
Description
</button>
</div>

<div group="to_debug" id="to_debug-Description" class="tabcontent"  style="display: block;" >
Convert the boolean value into a string in debug format.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> to_degrees </h2>

```rust,ignore
fn to_degrees(x: float) -> float
```

<div>
<div class="tab">
<button group="to_degrees" id="link-to_degrees-Description"  class="tablinks active" 
    onclick="openTab(event, 'to_degrees', 'Description')">
Description
</button>
</div>

<div group="to_degrees" id="to_degrees-Description" class="tabcontent"  style="display: block;" >
Convert radians to degrees.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> to_float </h2>

```rust,ignore
fn to_float()
fn to_float()
fn to_float()
fn to_float()
fn to_float(x: f32) -> float
fn to_float()
fn to_float()
fn to_float()
fn to_float()
fn to_float()
fn to_float()
fn to_float()
```

<div>
<div class="tab">
<button group="to_float" id="link-to_float-Description"  class="tablinks active" 
    onclick="openTab(event, 'to_float', 'Description')">
Description
</button>
</div>

<div group="to_float" id="to_float-Description" class="tabcontent"  style="display: block;" >
Convert the 32-bit floating-point number to 64-bit.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> to_hex </h2>

```rust,ignore
fn to_hex(value: i128) -> String
fn to_hex(value: i32) -> String
fn to_hex(value: int) -> String
fn to_hex(value: u32) -> String
fn to_hex(value: i16) -> String
fn to_hex(value: i8) -> String
fn to_hex(value: u64) -> String
fn to_hex(value: u8) -> String
fn to_hex(value: u128) -> String
fn to_hex(value: u16) -> String
```

<div>
<div class="tab">
<button group="to_hex" id="link-to_hex-Description"  class="tablinks active" 
    onclick="openTab(event, 'to_hex', 'Description')">
Description
</button>
</div>

<div group="to_hex" id="to_hex-Description" class="tabcontent"  style="display: block;" >
Convert the `value` into a string in hex format.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> to_int </h2>

```rust,ignore
fn to_int()
fn to_int()
fn to_int()
fn to_int()
fn to_int()
fn to_int()
fn to_int()
fn to_int()
fn to_int(x: f32) -> int
fn to_int()
fn to_int()
fn to_int(x: float) -> int
fn to_int()
```

<div>
<div class="tab">
<button group="to_int" id="link-to_int-Description"  class="tablinks active" 
    onclick="openTab(event, 'to_int', 'Description')">
Description
</button>
</div>

<div group="to_int" id="to_int-Description" class="tabcontent"  style="display: block;" >
Convert the floating-point number into an integer.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> to_json </h2>

```rust,ignore
fn to_json(map: Map) -> String
```

<div>
<div class="tab">
<button group="to_json" id="link-to_json-Description"  class="tablinks active" 
    onclick="openTab(event, 'to_json', 'Description')">
Description
</button>
<button group="to_json" id="link-to_json-Data types"  class="tablinks" 
    onclick="openTab(event, 'to_json', 'Data types')">
Data types
</button>
<button group="to_json" id="link-to_json-Errors"  class="tablinks" 
    onclick="openTab(event, 'to_json', 'Errors')">
Errors
</button>
<button group="to_json" id="link-to_json-Example"  class="tablinks" 
    onclick="openTab(event, 'to_json', 'Example')">
Example
</button>
</div>

<div group="to_json" id="to_json-Description" class="tabcontent"  style="display: block;" >
Return the JSON representation of the object map.
</div>
<div group="to_json" id="to_json-Data types" class="tabcontent"  style="display: none;" >

Only the following data types should be kept inside the object map:
`INT`, `FLOAT`, `ImmutableString`, `char`, `bool`, `()`, `Array`, `Map`.
</div>
<div group="to_json" id="to_json-Errors" class="tabcontent"  style="display: none;" >

Data types not supported by JSON serialize into formats that may
invalidate the result.
</div>
<div group="to_json" id="to_json-Example" class="tabcontent"  style="display: none;" >

```rhai
let m = #{a:1, b:2, c:3};

print(m.to_json());     // prints {"a":1, "b":2, "c":3}
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> to_lower </h2>

```rust,ignore
fn to_lower(character: char) -> char
fn to_lower(string: String) -> String
```

<div>
<div class="tab">
<button group="to_lower" id="link-to_lower-Description"  class="tablinks active" 
    onclick="openTab(event, 'to_lower', 'Description')">
Description
</button>
<button group="to_lower" id="link-to_lower-Example"  class="tablinks" 
    onclick="openTab(event, 'to_lower', 'Example')">
Example
</button>
</div>

<div group="to_lower" id="to_lower-Description" class="tabcontent"  style="display: block;" >
Convert the character to lower-case and return it as a new character.
</div>
<div group="to_lower" id="to_lower-Example" class="tabcontent"  style="display: none;" >

```rhai
let ch = 'A';

print(ch.to_lower());       // prints 'a'

print(ch);                  // prints 'A'
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> to_octal </h2>

```rust,ignore
fn to_octal(value: i32) -> String
fn to_octal(value: i128) -> String
fn to_octal(value: int) -> String
fn to_octal(value: i16) -> String
fn to_octal(value: u32) -> String
fn to_octal(value: u16) -> String
fn to_octal(value: u64) -> String
fn to_octal(value: u8) -> String
fn to_octal(value: u128) -> String
fn to_octal(value: i8) -> String
```

<div>
<div class="tab">
<button group="to_octal" id="link-to_octal-Description"  class="tablinks active" 
    onclick="openTab(event, 'to_octal', 'Description')">
Description
</button>
</div>

<div group="to_octal" id="to_octal-Description" class="tabcontent"  style="display: block;" >
Convert the `value` into a string in octal format.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> to_radians </h2>

```rust,ignore
fn to_radians(x: float) -> float
```

<div>
<div class="tab">
<button group="to_radians" id="link-to_radians-Description"  class="tablinks active" 
    onclick="openTab(event, 'to_radians', 'Description')">
Description
</button>
</div>

<div group="to_radians" id="to_radians-Description" class="tabcontent"  style="display: block;" >
Convert degrees to radians.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> to_string </h2>

```rust,ignore
fn to_string(array: Array) -> String
fn to_string(map: Map) -> String
fn to_string(character: char) -> String
fn to_string(item: ?) -> String
fn to_string(value: bool) -> String
fn to_string(number: f32) -> String
fn to_string(unit: ?) -> String
fn to_string(number: float) -> String
fn to_string(string: String) -> String
```

<div>
<div class="tab">
<button group="to_string" id="link-to_string-Description"  class="tablinks active" 
    onclick="openTab(event, 'to_string', 'Description')">
Description
</button>
</div>

<div group="to_string" id="to_string-Description" class="tabcontent"  style="display: block;" >
Convert the array into a string.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> to_upper </h2>

```rust,ignore
fn to_upper(string: String) -> String
fn to_upper(character: char) -> char
```

<div>
<div class="tab">
<button group="to_upper" id="link-to_upper-Description"  class="tablinks active" 
    onclick="openTab(event, 'to_upper', 'Description')">
Description
</button>
<button group="to_upper" id="link-to_upper-Example"  class="tablinks" 
    onclick="openTab(event, 'to_upper', 'Example')">
Example
</button>
</div>

<div group="to_upper" id="to_upper-Description" class="tabcontent"  style="display: block;" >
Convert the string to all upper-case and return it as a new string.
</div>
<div group="to_upper" id="to_upper-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "hello, world!"

print(text.to_upper());     // prints "HELLO, WORLD!"

print(text);                // prints "hello, world!"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> trim </h2>

```rust,ignore
fn trim(string: String)
```

<div>
<div class="tab">
<button group="trim" id="link-trim-Description"  class="tablinks active" 
    onclick="openTab(event, 'trim', 'Description')">
Description
</button>
<button group="trim" id="link-trim-Example"  class="tablinks" 
    onclick="openTab(event, 'trim', 'Example')">
Example
</button>
</div>

<div group="trim" id="trim-Description" class="tabcontent"  style="display: block;" >
Remove whitespace characters from both ends of the string.
</div>
<div group="trim" id="trim-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "   hello     ";

text.trim();

print(text);    // prints "hello"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> truncate </h2>

```rust,ignore
fn truncate(string: String, len: int)
fn truncate(array: Array, len: int)
fn truncate(blob: Blob, len: int)
```

<div>
<div class="tab">
<button group="truncate" id="link-truncate-Description"  class="tablinks active" 
    onclick="openTab(event, 'truncate', 'Description')">
Description
</button>
<button group="truncate" id="link-truncate-Example"  class="tablinks" 
    onclick="openTab(event, 'truncate', 'Example')">
Example
</button>
</div>

<div group="truncate" id="truncate-Description" class="tabcontent"  style="display: block;" >
Cut off the string at the specified number of characters.

* If `len` ≤ 0, the string is cleared.
* If `len` ≥ length of string, the string is not truncated.
</div>
<div group="truncate" id="truncate-Example" class="tabcontent"  style="display: none;" >

```rhai
let text = "hello, world! hello, foobar!";

text.truncate(13);

print(text);    // prints "hello, world!"

text.truncate(10);

print(text);    // prints "hello, world!"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> u128.is_even </h2>

```rust,ignore
get u128.is_even -> bool
```

<div>
<div class="tab">
<button group="u128.is_even" id="link-u128.is_even-Description"  class="tablinks active" 
    onclick="openTab(event, 'u128.is_even', 'Description')">
Description
</button>
</div>

<div group="u128.is_even" id="u128.is_even-Description" class="tabcontent"  style="display: block;" >
Return true if the number is even.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> u128.is_odd </h2>

```rust,ignore
get u128.is_odd -> bool
```

<div>
<div class="tab">
<button group="u128.is_odd" id="link-u128.is_odd-Description"  class="tablinks active" 
    onclick="openTab(event, 'u128.is_odd', 'Description')">
Description
</button>
</div>

<div group="u128.is_odd" id="u128.is_odd-Description" class="tabcontent"  style="display: block;" >
Return true if the number is odd.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> u128.is_zero </h2>

```rust,ignore
get u128.is_zero -> bool
```

<div>
<div class="tab">
<button group="u128.is_zero" id="link-u128.is_zero-Description"  class="tablinks active" 
    onclick="openTab(event, 'u128.is_zero', 'Description')">
Description
</button>
</div>

<div group="u128.is_zero" id="u128.is_zero-Description" class="tabcontent"  style="display: block;" >
Return true if the number is zero.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> u16.is_even </h2>

```rust,ignore
get u16.is_even -> bool
```

<div>
<div class="tab">
<button group="u16.is_even" id="link-u16.is_even-Description"  class="tablinks active" 
    onclick="openTab(event, 'u16.is_even', 'Description')">
Description
</button>
</div>

<div group="u16.is_even" id="u16.is_even-Description" class="tabcontent"  style="display: block;" >
Return true if the number is even.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> u16.is_odd </h2>

```rust,ignore
get u16.is_odd -> bool
```

<div>
<div class="tab">
<button group="u16.is_odd" id="link-u16.is_odd-Description"  class="tablinks active" 
    onclick="openTab(event, 'u16.is_odd', 'Description')">
Description
</button>
</div>

<div group="u16.is_odd" id="u16.is_odd-Description" class="tabcontent"  style="display: block;" >
Return true if the number is odd.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> u16.is_zero </h2>

```rust,ignore
get u16.is_zero -> bool
```

<div>
<div class="tab">
<button group="u16.is_zero" id="link-u16.is_zero-Description"  class="tablinks active" 
    onclick="openTab(event, 'u16.is_zero', 'Description')">
Description
</button>
</div>

<div group="u16.is_zero" id="u16.is_zero-Description" class="tabcontent"  style="display: block;" >
Return true if the number is zero.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> u32.is_even </h2>

```rust,ignore
get u32.is_even -> bool
```

<div>
<div class="tab">
<button group="u32.is_even" id="link-u32.is_even-Description"  class="tablinks active" 
    onclick="openTab(event, 'u32.is_even', 'Description')">
Description
</button>
</div>

<div group="u32.is_even" id="u32.is_even-Description" class="tabcontent"  style="display: block;" >
Return true if the number is even.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> u32.is_odd </h2>

```rust,ignore
get u32.is_odd -> bool
```

<div>
<div class="tab">
<button group="u32.is_odd" id="link-u32.is_odd-Description"  class="tablinks active" 
    onclick="openTab(event, 'u32.is_odd', 'Description')">
Description
</button>
</div>

<div group="u32.is_odd" id="u32.is_odd-Description" class="tabcontent"  style="display: block;" >
Return true if the number is odd.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> u32.is_zero </h2>

```rust,ignore
get u32.is_zero -> bool
```

<div>
<div class="tab">
<button group="u32.is_zero" id="link-u32.is_zero-Description"  class="tablinks active" 
    onclick="openTab(event, 'u32.is_zero', 'Description')">
Description
</button>
</div>

<div group="u32.is_zero" id="u32.is_zero-Description" class="tabcontent"  style="display: block;" >
Return true if the number is zero.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> u64.is_even </h2>

```rust,ignore
get u64.is_even -> bool
```

<div>
<div class="tab">
<button group="u64.is_even" id="link-u64.is_even-Description"  class="tablinks active" 
    onclick="openTab(event, 'u64.is_even', 'Description')">
Description
</button>
</div>

<div group="u64.is_even" id="u64.is_even-Description" class="tabcontent"  style="display: block;" >
Return true if the number is even.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> u64.is_odd </h2>

```rust,ignore
get u64.is_odd -> bool
```

<div>
<div class="tab">
<button group="u64.is_odd" id="link-u64.is_odd-Description"  class="tablinks active" 
    onclick="openTab(event, 'u64.is_odd', 'Description')">
Description
</button>
</div>

<div group="u64.is_odd" id="u64.is_odd-Description" class="tabcontent"  style="display: block;" >
Return true if the number is odd.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> u64.is_zero </h2>

```rust,ignore
get u64.is_zero -> bool
```

<div>
<div class="tab">
<button group="u64.is_zero" id="link-u64.is_zero-Description"  class="tablinks active" 
    onclick="openTab(event, 'u64.is_zero', 'Description')">
Description
</button>
</div>

<div group="u64.is_zero" id="u64.is_zero-Description" class="tabcontent"  style="display: block;" >
Return true if the number is zero.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> u8.is_even </h2>

```rust,ignore
get u8.is_even -> bool
```

<div>
<div class="tab">
<button group="u8.is_even" id="link-u8.is_even-Description"  class="tablinks active" 
    onclick="openTab(event, 'u8.is_even', 'Description')">
Description
</button>
</div>

<div group="u8.is_even" id="u8.is_even-Description" class="tabcontent"  style="display: block;" >
Return true if the number is even.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> u8.is_odd </h2>

```rust,ignore
get u8.is_odd -> bool
```

<div>
<div class="tab">
<button group="u8.is_odd" id="link-u8.is_odd-Description"  class="tablinks active" 
    onclick="openTab(event, 'u8.is_odd', 'Description')">
Description
</button>
</div>

<div group="u8.is_odd" id="u8.is_odd-Description" class="tabcontent"  style="display: block;" >
Return true if the number is odd.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> u8.is_zero </h2>

```rust,ignore
get u8.is_zero -> bool
```

<div>
<div class="tab">
<button group="u8.is_zero" id="link-u8.is_zero-Description"  class="tablinks active" 
    onclick="openTab(event, 'u8.is_zero', 'Description')">
Description
</button>
</div>

<div group="u8.is_zero" id="u8.is_zero-Description" class="tabcontent"  style="display: block;" >
Return true if the number is zero.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> values </h2>

```rust,ignore
fn values(map: Map) -> Array
```

<div>
<div class="tab">
<button group="values" id="link-values-Description"  class="tablinks active" 
    onclick="openTab(event, 'values', 'Description')">
Description
</button>
<button group="values" id="link-values-Example"  class="tablinks" 
    onclick="openTab(event, 'values', 'Example')">
Example
</button>
</div>

<div group="values" id="values-Description" class="tabcontent"  style="display: block;" >
Return an array with all the property values in the object map.
</div>
<div group="values" id="values-Example" class="tabcontent"  style="display: none;" >

```rhai
let m = #{a:1, b:2, c:3};

print(m.values());      // prints "[1, 2, 3]""
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> write_ascii </h2>

```rust,ignore
fn write_ascii(blob: Blob, range: Range<int>, string: String)
fn write_ascii(blob: Blob, range: RangeInclusive<int>, string: String)
fn write_ascii(blob: Blob, start: int, len: int, string: String)
```

<div>
<div class="tab">
<button group="write_ascii" id="link-write_ascii-Description"  class="tablinks active" 
    onclick="openTab(event, 'write_ascii', 'Description')">
Description
</button>
</div>

<div group="write_ascii" id="write_ascii-Description" class="tabcontent"  style="display: block;" >
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
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> write_be </h2>

```rust,ignore
fn write_be(blob: Blob, range: RangeInclusive<int>, value: float)
fn write_be(blob: Blob, range: Range<int>, value: int)
fn write_be(blob: Blob, range: Range<int>, value: float)
fn write_be(blob: Blob, range: RangeInclusive<int>, value: int)
fn write_be(blob: Blob, start: int, len: int, value: int)
fn write_be(blob: Blob, start: int, len: int, value: float)
```

<div>
<div class="tab">
<button group="write_be" id="link-write_be-Description"  class="tablinks active" 
    onclick="openTab(event, 'write_be', 'Description')">
Description
</button>
</div>

<div group="write_be" id="write_be-Description" class="tabcontent"  style="display: block;" >
Write a `FLOAT` value to the bytes within an inclusive `range` in the BLOB
in big-endian byte order.

* If number of bytes in `range` < number of bytes for `FLOAT`, extra bytes in `FLOAT` are not written.
* If number of bytes in `range` > number of bytes for `FLOAT`, extra bytes in `range` are not modified.
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> write_le </h2>

```rust,ignore
fn write_le(blob: Blob, range: Range<int>, value: int)
fn write_le(blob: Blob, range: RangeInclusive<int>, value: float)
fn write_le(blob: Blob, range: RangeInclusive<int>, value: int)
fn write_le(blob: Blob, range: Range<int>, value: float)
fn write_le(blob: Blob, start: int, len: int, value: float)
fn write_le(blob: Blob, start: int, len: int, value: int)
```

<div>
<div class="tab">
<button group="write_le" id="link-write_le-Description"  class="tablinks active" 
    onclick="openTab(event, 'write_le', 'Description')">
Description
</button>
</div>

<div group="write_le" id="write_le-Description" class="tabcontent"  style="display: block;" >
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
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> write_utf8 </h2>

```rust,ignore
fn write_utf8(blob: Blob, range: RangeInclusive<int>, string: String)
fn write_utf8(blob: Blob, range: Range<int>, string: String)
fn write_utf8(blob: Blob, start: int, len: int, string: String)
```

<div>
<div class="tab">
<button group="write_utf8" id="link-write_utf8-Description"  class="tablinks active" 
    onclick="openTab(event, 'write_utf8', 'Description')">
Description
</button>
</div>

<div group="write_utf8" id="write_utf8-Description" class="tabcontent"  style="display: block;" >
Write a string to the bytes within an inclusive `range` in the BLOB in UTF-8 encoding.

* If number of bytes in `range` < length of `string`, extra bytes in `string` are not written.
* If number of bytes in `range` > length of `string`, extra bytes in `range` are not modified.

```rhai
let b = blob(8);

b.write_utf8(1..=5, "朝には紅顔ありて夕べには白骨となる");

print(b);       // prints "[00e69c9de3810000]"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> zip </h2>

```rust,ignore
fn zip(array1: Array, array2: Array, map: FnPtr) -> Array
```

<div>
<div class="tab">
<button group="zip" id="link-zip-Description"  class="tablinks active" 
    onclick="openTab(event, 'zip', 'Description')">
Description
</button>
<button group="zip" id="link-zip-Function Parameters"  class="tablinks" 
    onclick="openTab(event, 'zip', 'Function Parameters')">
Function Parameters
</button>
<button group="zip" id="link-zip-Example"  class="tablinks" 
    onclick="openTab(event, 'zip', 'Example')">
Example
</button>
</div>

<div group="zip" id="zip-Description" class="tabcontent"  style="display: block;" >
Iterate through all elements in two arrays, applying a `mapper` function to them,
and return a new array containing the results.
</div>
<div group="zip" id="zip-Function Parameters" class="tabcontent"  style="display: none;" >

* `array1`: First array
* `array2`: Second array
* `index` _(optional)_: current index in the array
</div>
<div group="zip" id="zip-Example" class="tabcontent"  style="display: none;" >

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
</br>
