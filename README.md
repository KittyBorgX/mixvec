# mixvec

Loosely typed, python style lists in rust.

### Example:

```rust
use mixvec::MixVec;
let myvec = MixVec::new();

myvec.push(42);
myvec.push("Meaning of life");
myvec.push(true);

println!("{}", myvec); // Prints out [42, "Meaning of life", true]
```

### Features:

- Can accept data of Integer, Float, String, Boolean and Character
- Has methods such as : `insert`, `remove`, `reverse`, `sort`, `push`, `pop`
- 2 `MixVec`'s can be added to give a new `MixVec` with combined elements
- A `MixVec` can be multiplied by an integer to duplicate the elements

### Roadmap:

- [ ] Finish tests
- [ ] Write documentation
- [ ] Write examples
- [ ] Setup CI
- [ ] Publish on `crates.io`
