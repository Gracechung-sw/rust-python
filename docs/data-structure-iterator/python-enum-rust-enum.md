# Data Structure와 Iterator
자료구조(Data Structure)와 이 자료구조에서 값을 하나씩 꺼내서 사용하는 iterator에 대해 알아본다. 
## Data Structure
> Data Structure: 컴퓨터에서 어떠한 값의 모음을 효율적으로 나타내기 위한 방법.

|Python|Rust|
|---|---|
|list|Vec|
|np.array|array|
|tuple|()|
|Enum|Enum|
|dict|std::collections::HashMap|
|str|String, &str|
이 외에 python이란 비교할 순 없지만 Rust만의 데이터 구조
- Sequences: VecDeque, LinkedList
- Maps: BTreeMap
- Sets: HashSet, BTreeSet
- Misc: BinaryHeap
이 각각에 대해 차례대로 알아보자. 

# 열거형
> 열거형이란, 여러 상수들의 집합으로 새로운 타입을 선언하는 방법. 

## Enum
#### Python
`Enum` class를 상속해서 열거형을 만들 수 있음. 
Enum 사용법
```python
from enum import Enum

class Languages(Enum):
  PYTHON = "python"
  RUST = "rust"
  JAVASCRIPT = "javascript"
  GO = "go"

  def echo(self):
      print(self.name) # echo 메소드를 정의했는데, 이 메소드는 Enum 클래스에 미리 정의된 name 프로퍼티를 프린트합니다.

language = Languages.RUST
language.echo()

if language == Languages.PYTHON:
    print("I love Python")
elif language == Languages.GO:
    print("I love Go")
elif language == Languages.JAVASCRIPT:
    print("I love Javascript")
else:
    print("I love Rust🦀")

# 실행 결과
# RUST
# I love Rust🦀
```

#### Rust
`enum` 키워드 사용
1. 값이 없는 열거형
``` rust
fn main() {
  // Enum
  #[allow(dead_code)]
  #[derive(Debug)] // derive Debug trait, to print the enum

  enum Languages {
    Python,
    Rust,
    Javascript,
    Go,
  }

  impl Languages { // impl 블럭을 이용해 열거형에서 사용할 메소드를 만들 수 있음. 
      fn echo(&self) {
          println!("{:?}", &self);
      }
  }

  let language = Languages::Rust;
  language.echo();

  // match
  match language {
      Languages::Python => println!("I love Python"),
      Languages::Go => println!("I love Go"),
      Languages::Javascript => println!("I love Javascript"),
      _ => println!("I love Rust🦀"),
  }
}

// 실행 결과
// Rust
// I love Rust🦀

```
2. 값이 있는 열거형
- 열거형에 값을 지정하려면 열거형을 선언하면서 열거형 변수 뒤에 (타입) 과 같이 입력해서 타입을 지정
```rust
#[allow(dead_code)]
fn main() {
    #[derive(Debug)] // derive Debug trait, to print the enum
    enum Grade {
        A,
        B,
        C,
    }

    enum Job { // 값이 있는 열거형 선언
        Student(Grade, String),
        Developer(String),
    }

    let indo = Job::Student(Grade::A, "indo".to_string()); // 그 열거형에 값 넣어주기. 

    match indo {
        Job::Student(grade, name) => {
            println!("{} is a student with grade {:?}", name, grade);
        }
        Job::Developer(name) => {
            println!("{} is a developer", name);
        }
    }
}

// 실행 결과
// indo is a student with grade A

```

## Option 열거형
`Option<T>` 열거형은 Some(T)와 None 값을 가질 수 있음. 
즉, T 라는 타입의 값이 있을 수도 있도 없을 수도 있음을 나타냄. 
```rust
enum Option<T> {
    Some(T),
    None,
}
```
```rust
fn check_len(vec: Vec<i32>) -> Option<usize> {
    match vec.len() {
        0 => None,
        _ => Some(vec.len()),
    }
}

fn main() {
    let nums = vec![1, 2, 3];

    match check_len(nums) {
        Some(len) => println!("Length: {}", len),
    }
}
```

## Result<T, E> 열거형
Result<T, E> 열거형은 Ok(T)와 Err(E) 값을 가질 수 있습니다. 
Ok는 결과값이 정상적으로 존재함을 의미, Err는 에러가 발생했음을 나타냅니다.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

> ? 연산자
>> ? 연산자는 함수 안에서 사용이 가능합니다. 이때 Result 의 결과값을 리턴

```rust
use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Info {
    name: String,
    age: i32,
    rating: i32,
}

fn write_info(info: &Info) -> io::Result<()> {
    let mut file = File::create("my_best_friends.txt")?;
    // Early return on error
    file.write_all(format!("name: {}\n", info.name).as_bytes())?;
    file.write_all(format!("age: {}\n", info.age).as_bytes())?;
    file.write_all(format!("rating: {}\n", info.rating).as_bytes())?;
    Ok(())
}

fn main() {
    if let Ok(_) = write_info(&Info {
        name: "John".to_string(),
        age: 32,
        rating: 10,
    }) {
        println!("Writing to file succeeded!");
    }
}
```