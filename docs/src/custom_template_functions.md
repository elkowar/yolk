# Custom Template Functions

Given that all of the template functions are just regular Rhai code, you might ask yourself if you can define your own template tag functions.
The answer is YES!

## How do template tag functions work

Template tag functions are executed in a special context, in which a function called `get_yolk_text()` is availale.
This variable contains the text block that the template tag operates on.
A template tag function then returns a string which yolk will replace the old text block with.

## Example

Let's define a simple, useless template tag function in your `yolk.rhai`.

```rust,ignore
fn scream_or_not(should_scream)
  if should_scream {
    get_yolk_text().to_upper()
  } else {
    get_yolk_text().to_lower()
  }
}
```

That's it!
Now, we can go into any templated file, and use our new template tag function.

```toml
# {# scream_or_not(SYSTEM.hostname == "loud-host") #}
screaming = "HELLO"
```
