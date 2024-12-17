<div class='rhai-doc'>

# IO Functions

A collection of functions that can read the environment and filesystem.
These return standardized values in canonical mode.

---

**namespace**: `io`

---



<div class='doc-block'>

## command_available

<div class='doc-content'>

```rust,ignore
command_available(name: &str) -> Result<bool>
```

Check if a given command is available

</div>
</div>




<div class='doc-block'>

## env

<div class='doc-content'>

```rust,ignore
env(name: &str, def: &str) -> Result<String>
```

Read an environment variable, or return the given default

</div>
</div>




<div class='doc-block'>

## path_exists

<div class='doc-content'>

```rust,ignore
path_exists(p: &str) -> Result<bool>
```

Check if something exists at a given path

</div>
</div>




<div class='doc-block'>

## path_is_dir

<div class='doc-content'>

```rust,ignore
path_is_dir(p: &str) -> Result<bool>
```

Check if the given path is a directory

</div>
</div>




<div class='doc-block'>

## path_is_file

<div class='doc-content'>

```rust,ignore
path_is_file(p: &str) -> Result<bool>
```

Check if the given path is a file

</div>
</div>




<div class='doc-block'>

## read_dir

<div class='doc-content'>

```rust,ignore
read_dir(p: &str) -> Result<Vec<String>>
```

Read the children of a given dir

</div>
</div>




<div class='doc-block'>

## read_file

<div class='doc-content'>

```rust,ignore
read_file(p: &str) -> Result<String>
```

Read the contents of a given file

</div>
</div>




</div>