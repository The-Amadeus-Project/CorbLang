```rust
[public string rest] name = "Jack Sparrow"; // restricted

fn test() -> Result<Error, bool>{
    [list<string>] name_as_list = nane.split();
    [string] last_name = name_as_list.pop();
    [string] first_name = name_as_list.pop();
    if name_as_list.len > 0 {
        Err(Error, "A Hole")
    }
}

fn main(){
    test()
}
```