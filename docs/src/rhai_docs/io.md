# IO Functions

A collection of functions that can read the environment and filesystem.
These return standardized values in canonical mode.

---

**namespace**: `io`

---

## command_available

```rust,ignore
command_available(name: &str) -> Result<bool>
```

> Check if a given command is available

---
## env

```rust,ignore
env(name: &str, def: &str) -> Result<String>
```

> Read an environment variable, or return the given default

---
## path_exists

```rust,ignore
path_exists(p: &str) -> Result<bool>
```

> Check if something exists at a given path

---
## path_is_dir

```rust,ignore
path_is_dir(p: &str) -> Result<bool>
```

> Check if the given path is a directory

---
## path_is_file

```rust,ignore
path_is_file(p: &str) -> Result<bool>
```

> Check if the given path is a file

---
## read_dir

```rust,ignore
read_dir(p: &str) -> Result<Vec<String>>
```

> Read the children of a given dir

---
## read_file

```rust,ignore
read_file(p: &str) -> Result<String>
```

> Read the contents of a given file

---
