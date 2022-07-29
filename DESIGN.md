# Syntax

```rust
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

## Error handling
```rust
fn minus([unsigned int] num1, [unsigned int] num2) -> Result<int> {
    if num1 < num2 {
        Bad("Results in signed int")
    } else {
        Good(num1 - num2)
    }
}

fn main(){
    [unsigned int] number_1 = 1;
    [unisgned int] number_2 = 2;
    [Result<int>] res = minus(number_1 - number_2);
    case res { 
    Good(res_num) -> {
            print(f"{number_1} - {number_2} = {res_num}")
        }
    Bad(_res_err_message) -> {
            print(f"{number_1} - {number_2} results in negatives")
        }
    }
}
```

```rust
use std::err::ErrMessage;
[public unsigned int rest] two = 22;

class ProgrammerErr(ErrMessage):
class ComputerErr(ErrMessage):

fn minus([unsigned int] num1, [unsigned int] num2) -> MultiResult<string, (ProgrammerErr, ComputerErr)> {
    if 1 + 1 == 3 {
        ComputerErr("Somethings is wrong")
    } else if 1 + two != 3 {
        ProgrammerErr("Who Deployed This?")
    } else {
        Good("LGTM")
    }
}

fn main(){
    [auto] test_res = test()
    case test_res { 
        ProgrammerErr(message) -> {
            print(message);
        }
        ComputerErr(message) -> {
            print(message);
        }
        Good(message) -> { 
            print(message);
        }
    }
    
}
```