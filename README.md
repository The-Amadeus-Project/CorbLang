# CorbLang
A Language designed to let the programmer control everything

# Syntax
```rs
[public string rest] name = "Jack Sparrow"; // restricted

fn main(){
    [list<string>] name_as_list = name.split();
    [string] last_name = name_as_list.pop();
    [string] first_name = name_as_list.pop();
    if name_as_list.len > 0 {
        print("WARNING! RESIDUAL EXISTS")
    }
}
```
