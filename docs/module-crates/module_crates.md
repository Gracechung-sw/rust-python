# Module과 Crates
## Python
## Python module 사용 예시
> ./python folder에 아래 example code가 모두 있습니다. 
1. create ./python/my_module.py
```python
def greet():
  print(f"Hi! I am hello bot")

class Person:
  def __init__(self, name, age):
    self.name = name
    self.age = age

  def get_older(self, year):
    self.age += year
```

2. 위 ./python/my_module.py에서 선언한 함수와 클래스를 ./python/main.py에서 참조한다. 
```python
from my_module import greet, Person

if __name__ == '__main__':
  greet()

  john = Person("john", 20)
  john.get_older(3)
  print(john.age)
```

3. create ./python/bots/hello_bot.py 
```python
BOT_NAME = "hello_bot"

def hello():
    print("Hello, humans!")
```

4. 위  ./python/bots/hello_bot.py에서 선언한 전역 변수 BOT_NAME와 함수를 ./python/my_module.py에서 참조한다. 
```python
from bots.hello_bot import BOT_NAME

def greet():
  print(f"Hi! I am hello bot")
  print(f"Hi I am {BOT_NAME}")

class Person:
  def __init__(self, name, age):
    self.name = name
    self.age = age

  def get_older(self, year):
    self.age += year
```

5. ./python/main.py 에서도 bots 모듈을 사용해보자. 
```python
from my_module import greet, Person
from bots.hello_bot import hello

if __name__ == '__main__':
  hello()

  greet()

  john = Person("john", 20)
  john.get_older(3)
  print(john.age)
```


## Rust
러스트의 모듈 시스템은 4가지
- Packages: cargo(러스트의 내장 패키지 매니저인 Cargo 덕분에 빌드, 테스트, 의존성 관리 등이 매우 간편합니다.)에서 제공하는 기능으로, crate를 빌드하고 생성 가능. 
- Crates: Library 또는 Binary를 생성하는 a tree of modules. Library crates, binary crates가 있나봄. 
- mod와 use: 코드 안에서 다른 module을 구성하고, 불러오거나 다른 module에 노출할 지 여부(private or public)을 결정.
- 경로: Module에서 특정 요소(함수, 구조체, 변수 등)를 찾기 위한 방법

이 각각에 대해서 알아보고자 함. 

## Packages
cargo(러스트의 내장 패키지 매니저인 Cargo 덕분에 빌드, 테스트, 의존성 관리 등이 매우 간편합니다.)에서 제공하는 기능으로, crate를 빌드하고 생성 가능. 

README.md 에
`Cargo.toml`: 프로젝트의 모든 설정값을 가지고 있는 파일
이라는 설명이 있다. 


`Cargo.toml`: 프로젝트의 모든 설정값을 가지고 있는 파일
```
[package]
name = "rust_part"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies] 
# 현재 프로젝트에서 설치하는 크레이트(러스트에서는 패키지를 크레이트(crate)라고 부릅니다)의 이름과 버전이 들어가게 됩니다. 
```
하나의 패키지에는 단 하나의 library crate만 포함할 수 있고, binary crates는 여러 개 가능. 

## Crates
Library 또는 Binary를 생성하는 a tree of modules. Library crates, binary crates가 있나봄. 

### Library crates
다른 크레이트나 패키지에서 코드를 참조할 수 있도록 제공되는 크레이트.    
 
lib.rs    

cargo new --lib   
컴파일되지 않기 때문에 바이너리를 생성하지 않습니다.    

### Binary crates
컴파일되어 바이너리 파일을 생성하는 크레이트    
main.rs    

cargo new   

### crates root
컴파일 엔트리포인트를 의미합니다. 바이너리 크레이트는 src/main.rs 파일이, 라이브러리 크레이트는 src/lib.rs 파일이 크레이트 루트가 됩니다.

## mod와 use
코드 안에서 다른 module을 구성하고, 불러오거나 다른 module에 노출할 지 여부(private or public)을 결정.
### private or Public
러스트의 모든 모듈과 객체는 기본적으로 private이다. 즉, 모듈 외부에서 해당 모듈이나 객체에 접근이 불가능. 
따라서 외부에서 모듈에 접근하거나 모듈 내부의 객체에 접근을 허용하려면 `pub` 키워드를 써서 public으로 만들어줘야함. 
```rust
pub mod {

}

pub fn {

}

pub struct {

}

pub static
```
### use
`use` 키워드는 특정 경로를 현재 스코프로 가져오는 역할을 한다. 
> 주의: 경로는 항상 crate root로부터 시작된다. 

### mod
`mod` 키워드는 해당 모듈을 사용하겠다고 선언하는 역할. 
예를 들어 `mod new_module`이 사용되면, 컴파일러는 아래 위치에서 해당 모듈을 찾아봅니다.
`mod new_module`이 사용이 아래처럼 되었다고 치자. 
```rust
mod new_module {
  // 아래 해당 모듈의 정의가 되어 있다. 
  fn new_func() {
    ...
  }

  ...
}
```
그러면 컴파일러는 아래 순서로 이 모듈을 찾는다. 
1. src/new_module.rs 파일을 찾아봅니다.
2. src/new_module 폴더에서 mod.rs 파일을 찾아봅니다.

crates root 가 아닌 모듈에서 선언되는 모듈인 서브 모듈일 경우는?  해당 서브모듈을 컴파일러가 찾는 규칙은 위와 동일하다. 

#### 절대 경로 사용
특정 모듈에 대한 접근은 크레이트 루트를 기준으로 절대경로를 사용하면 됩니다. 예를 들어 코드 어디에서라도 다음과 같이 모듈에 접근이 가능합니다.
```rust
// src/new_module.rs -> MyType
use crate::new_module::MyType
```

#### 상대 경로 사용
상대 경로를 사용할 때는 `self`(self는 struct 자기 자신을 의미) 와 `super`(현재 모듈의 상위 모듈을 의미) 키워드를 사용합니다.
##### self 사용 예시 
```rust
mod mod2 {
  fn func() {
    println!("mod2::func()");
  }

  mod mod1 {
    pub fn func() {
      println!("mod2::mod1::func()");
    }
  }

  pub fn dummy() {
    func();
    self::func(); 
    mod1::func();
    self::mod1::func();
  }
}

fn main() {
    mod2::dummy();
}

// 실행 결과
// mod2::func()
// mod2::func()
// mod2::mod1::func()
// mod2::mod1::func()
```
##### super 사용 예시 
```rust
mod mod1 {
  pub fn dummy() {
    println!("Hello, world!");
  }
}

mod mod2 {
  use super::mode1;

  pub fn dummy() {
    mode1::dummy();
  }
}

fn main() {
  mod2::dummy();
}

// 실행 결과
// Hello, world!
```

## Rust module 사용 예시
위 <Python module 사용 예시> 부분을 Rust로 동일하게 구현해보자. 
> ./rust folder에 아래 example code가 모두 있습니다. 
1. create ./rust/my_module.rs
```rust
pub fn greet() {
  println!("Hi I am hello bot")
}

pub struct Person {
  pub name: String,
  age: i32
}

impl Person {
  pub fn new(name: &str, age: i32) -> Self {
    Person {
      name: String::from(name),
      age: age
    }
  }

  pub fn get_older(&mut self, year: i32) {
    self.age += year;
  }
}
```

2. 위 ./rust/my_module.rs에서 선언한 함수와 클래스를 ./rust/main.rs에서 참조한다. 
```rust
mod my_module; // mod 키워드로 ./rust/my_module.rs의 my_module을 스코프로 가져온다. 
use my_module::{greet, Person} // 그 다음 가져오고자 하는 함수와 구조체를 가져올 수 있습니다.

// 이렇게 가져온 함수와 구조체를 이제 main() 함수 내에서 사용할 수 있습니다
fn main() {
  greet();

  let mut john = Person::new("john", 20);
  john.get_older(3);
  println!("{}", john.age);
}
```

3. create ./rust/bots/hello_bot.rs and ./rust/bots/mod.rs
> python에서는 ./python/bots/hello_bot.py 만 만들면 됐었는데, rust에서는 항상 하위 폴더를 모듈로 만드는 경우에는 mod.rs 가 있어야 합니다. 이 파일은 해당 모듈의 엔트리포인트가 되어 이 모듈 안에 있는 다른 하위 모듈들을 찾을 수 있게 합니다. 따라서 mod.rs 에는 hello_bot 모듈의 정보가 있어야 합니다.

./rust/bots/mod.rs
```rust
pub mod hello_bot; // will look for hello_bot.rs
```

./rust/bots/hello_bot.ts
```rust
pub static BOT_NAME: &str = "hello_bot";

pub fn hello() {
  println!("Hello, humans!")
}
```

4. 위  ./rust/bots/hello_bot.rs에서 선언한 전역 변수 BOT_NAME와 함수를 ./rust/my_module.rs에서 참조한다.    
> 절대경로 형식으로  `use crate::new_module::MyType` 이런 식으로 적어준다. 

```rust
use crate::bots::hello_bot::BOT_NAME;

pub fn greet() {
    println!("Hi I am hello bot")
    println!("Hi! I am {}", BOT_NAME);
}

pub struct Person {
    pub name: String,
    age: i32,
}

impl Person {
    pub fn new(name: &str, age: i32) -> Self {
        Person {
            name: String::from(name),
            age: age,
        }
    }

    pub fn get_older(&mut self, year: i32) {
        self.age += year;
    }
}
```

5. ./rust/main.rs에서도 bots 모듈을 사용해보자. 
> main.rs는 크레이트 루트기 때문에 use bots::hello_bot::hello; 로 모듈을 불러올 수 있습니다.
```rust
mod my_module; // mod 키워드로 ./rust/my_module.rs의 my_module을 스코프로 가져온다. 
mod bots; // will look for a file src/bots/mod.rs

use my_module::{greet, Person} // 그 다음 가져오고자 하는 함수와 구조체를 가져올 수 있습니다.
use bots::hello_bot::hello; // actually import the function from hello.rs

// 이렇게 가져온 함수와 구조체를 이제 main() 함수 내에서 사용할 수 있습니다
fn main() {
  hello(); 
  
  greet();

  let mut john = Person::new("john", 20);
  john.get_older(3);
  println!("{}", john.age);
}
```