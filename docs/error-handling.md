# 에러 처리

## Python
파이썬에서 에러 처리를 하는 두 가지 스타일
1. LBYL(Look Before You Leap)
이 스타일은 호출이나 조회를 하기 전에 명시적으로 전제 조건을 확인한다. 
그래서 if문이 많다는 특징이 있다.
```python
if key in mapping:
  return mapping[key]
```
2. EAFP(Easier to Ask for Forgiveness than Permission). 
허락보다 용서받는 것이 더 쉽다. 이 스타일은 유효한 키 또는 속성이 있다고 가정한 후에 먼저 실행을 한 다음
가정이 거짓으로 판명되면 예외를 처리한다. 
try/except가 많다는 특징이 있다. 
```python
try:
  file = open("file.txt", "r")
except FileNotFoundError:
  print("File not found")
```

## Rust
러스트에서 에러 처리는 LBYL, EAFP와 비슷한 점이 있긴 하지만 둘 중 어느 것에도 똑같지는 않다.    
러스트에서 에러 처리를 하는 스타일은
1. panic!을 사용해서 치명적인 오류를 처리하는 방법
2. 값이 선택 사항이거나 값이 없어도 오류 조건이 아닌 경우 Option 열거형을 사용하는 방법
3. 오류가 발생할 수 있고 호출자가 문제를 처리해야하는 경우 Result 열거형을 사용하는 방법
이 있다. 

이것에 대해 순차적으로 알아본다. 

## panic!
### Python
파이썬에서는 에러를 발생시켜(raise) 코드를 즉시 종료할 수 있다. 
```python
raise Exception

# 실행 결과
# Traceback (most recent call last):
#   File "/temp/python/main.py", line 1, in <module>
#     raise Exception
# Exception
```
### Rust
러스트에서는 에러가 발생해 코드가 즉시 종료되는걸 panic이 발생했다고 한다. 
패닉을 직접 실행할 수도 있고, 패닉이 발생하는 코드가 실행되서 패닉이 발생할 수도 있다. 
```rust
fn main() {
    panic!("🤯");
}

// 실행 결과
// thread 'main' panicked at '🤯', src/main.rs:2:5
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Option 열거형
입력받는 파라미터에 따라서 결과값이 있을 수도 있고, 없을 수도 있는 경우가 있다. 
Option 열거형은 Some과 None을 가진다. 
```rust
fn give_some_or_none(some: bool) -> Option<String> {
  if some {
      Some(String::from("💖"))
    } else {
      None
    }
  }
```
그럼 이 give_some_or_none함수를 호출해서 사용하는 곳에서는 이 결과값을 어떻게 처리하는게 좋을까?   
match문을 사용해 각 경우를 모두 처리할 수 있다.

하지만 때로는 Some인 경우는 그냥 값을 바로 사용하고, 에러가 발생하는 경우만 따로 처리하고 싶은 경우가 있습니다. if let Some 을 사용해도 되지만, 함수형 프로그래밍답게 처리하는 방법이 존재합니다.
### unwrap
### unwrap_or
### unwrap_or_else
### unwrap_or_default
### ? 
> 이 각각에 대해서는 https://indosaram.github.io/rust-python-book/ch10-02.html 여기에 있는 설명이 너무 부족하다. 따로 알아봐서 적자. 

## Result 열거형
Result 열거형은 Ok와 Err을 가진다. 
```rust
use std::fmt::Error;

fn give_ok_or_err(bool: bool) -> Result<String, Error> {
    if bool {
        Ok(String::from("💖"))
    } else {
        Err(Error)
    }
}

fn main() {
    for bool in [true, false].iter() {
        let result = give_ok_or_err(*bool);
        match result {
            Ok(value) => println!("value: {}", value),
            Err(e) => println!("error: {}", e),
        }
    }
}
```
### unwrap
### unwrap_or
### ok_or
### ok_or_else
### ?

# 커스텀 예외 정의하기
## Python
```python
import os

def get_content():
  filepath = os.path.join(os.path.pardir, "test.txt")
  with open(filepath, "r") as f:
    return f.read()

class CustomError(Exception):
  pass

if __name__ == '__main__':
  try:
    with open("hello.txt", "r") as file:
        file.read()
  except FileNotFoundError as exc:
    print(exc)
  except:
    print("Unexpected error")

  try:
    content = get_content()
  except:
    raise CustomError
  print(content)

```
## Rust
```rust
use std::fmt;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::Path;

fn get_content() -> Result<String, Error> {
    let mut content = String::new();
    let filepath = Path::new(std::env::current_dir().unwrap().parent().unwrap()).join("test.txt");
    File::open(filepath)?.read_to_string(&mut content)?;
    Ok(content)
}

#[derive(Debug, Clone)]
struct CustomError;

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File not found!")
    }
}

fn main() {
    let file = File::open("hello.txt");
    match file {
        Ok(file) => println!("{:?}", file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("{:?}", error),
            _ => println!("Unexpected error"),
        },
    }

    let content = get_content().unwrap_or_else(|_| {
        panic!("{}", CustomError);
    });
    println!("{}", content);
}
```

TODO:  
- https://indosaram.github.io/rust-python-book/ch10-00.html 이 자료의 에러 처리 설명은 너무 부족해서
- https://rinthel.github.io/rust-lang-book-ko/ch06-00-enums.html  이것과
- https://rinthel.github.io/rust-lang-book-ko/ch09-00-error-handling.html 이것으로 다시 보충하는게 필요하다. 