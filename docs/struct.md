# Struct
### Python
객체지향적으로 짤 수 있음
class 존재

### Rust
함수형 프로그래밍에 더 가까움
class 없음 대신 비슷한 역할을 구조체 struct를 통해 구현 가능

## 구조체 선언
### Python
클래스 정의
```python
class Person:
  def __init__(self, name, age):
    self.name = name
    self.age = age
```
인스턴스 생성
```python
jane = Person("jane", 30)
jane.age += 1
print(jane.name, jane.age) # jane 31
print(jane.__dict__)
```
### Rust
`struct` 키워드 뒤에 구조체 이름을 명시한다. 
구조체 안에서는 `필드명: 타입명`으로 **필드**를 적어준다. 
```rust
#[derive(Debug)] // derived traits(미리 정의되어 있는 기능 이라는 뜻). 구조체의 내용을 보기 위해서 필요합
struct Person {
  name: String,
  age: i32
}
```
인스턴스 생성
```rust
fn main() {
  let mut jane = Person { // jane.age += 1; 에서 값을 변경시켜 주고 있기 때문에 mut 키워드로 mutable로 선언함. 
    name: String::from("jane"),
    age: 30
  };
  jane.age += 1;
  println!("{} {}", jane.name, jane.age);
  println!("{:?}", jane); // 러스트에서 어떤 값을 출력하려면 그 값은 Format이 정의되어 있어야 하는데, Person 구조체는 정의되어 있지 않기 때문에 {:?}를 활용한 디버그 출력을 이용해 간편하게 내용을 확인할 수 있습니다.
}
```
> 아 struct가 무슨 python 의 dict같고 근데 python dict랑 대응되는 거는 rust에서 hashMap이고, struct랑 hashMap 사용하는 문법은 전혀 다르고... 하  헷갈려 ㅠ

## 메소드
함수는 메소드를 통해 구조체에 묶을 수 있다. 
### Python
```python
class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age
        self.alive = True
        
    def info(self):
      	print(self.name, self.age)

    def get_older(self, year):
        self.age += year
```
### Rust
`impl` 키워드 뒤에 구조체 이름을 명시해서 해당 구조체에 속한 **메소드**를 선언할 수 있다. 
```rust
#[derive(Debug)] // derived traits(미리 정의되어 있는 기능 이라는 뜻). 구조체의 내용을 보기 위해서 필요합
struct Person {
  name: String,
  age: i32,
  alive: bool,
}


impl Person {
  fn info(&self) {
    println!("{} {}", self.name, self.age);
  }

  fn get_older(&mut self, year:i32) {
    // if we don't borrow the ownership, ownership will be moved to the
    // function and the variable will be dropped
    // self must be passed as mutable reference
    self.age += year;
  }
}
```

메소드가 아닌 함수도 구조체 정의에 포함가능. 연관 함수(Associated function)이라고 부르고, 파라미터에 self가 들어있지 않습니다. 
```rust
#[derive(Debug)] // derived traits(미리 정의되어 있는 기능 이라는 뜻). 구조체의 내용을 보기 위해서 필요합
struct Person {
  name: String,
  age: i32,
  alive: bool,
}

impl Person {
  // 또 다른 메소드 예시) 연관 함수
  fn new(name: &str, age:i32) -> Self { // return type이 Self 인 것의 의미: 자신이 속한 구조체 타입인 Person 클래스를 리턴한다는 의미
    Person {
      name: String::from(name),
      age: age,
      alive: true
    }
  }
}
```

## Tuple Struct
튜플 구조체는 구조체 필드가 이름 대신 튜플 순서대로 정의되는 구조체입니다. 필드 참조 역시 튜플의 원소를 인덱스로 참조하는 것과 동일합니다.

### Rust
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{} {}", black.0, origin.0);
}
```

## 구조체간 메소드 공유하기. 상속? trait!
### Python
python에서는 상속을 통해서 하나 이상의 class에서 메소드를 공유할 수 있다. 
```python
class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age
        self.alive = True

    def say_hello(self):
        print("Hello, Rustacean!")

    def get_older(self, year):
        self.age += year


class Student(Person):
    def __init__(self, name, age, major):
        super().__init__(name, age)
        self.major = major

    def say_hello(self): # 오버라이드
        print(f"Hello, I am {self.name} and I am studying {self.major}")
```

### Rust
러스트에는 구조체의 상속이 안 된다.   
서로 다른 struct 타입들이 메소드를 공유할 수 있는 하나의 속성을 정의할 수 있는데, 바로 trait.

구조체 전에 trait에 공유할 메소드의 원형을 선언한다. 
```rust
trait Greet {
    fn say_hello(&self) {}
}

struct Person {
    name: String,
    age: i32,
    alive: bool,
}

impl Person {
    fn new(name: &str, age: i32) -> Person {
        Person {
            name: String::from(name),
            age: age,
            alive: true
        }
    }

    fn get_older(&mut self, year: i32) {
        self.age += year;
    }
}

impl Greet for Person {
  fn say_hello(&self) {
      println!("Hello, Rustacean!")
  }
}


struct Student {
    name: String,
    age: i32,
    alive: bool,
    major: String,
}

impl Student {
    fn new(name: &str, age: i32, major: &str) -> Student {
        Student {
            name: String::from(name),
            age: age,
            alive: true,
            major: String::from(major),
        }
    }
}

impl Greet for Student {
    fn say_hello(&self) {
        println!("Hello, I am {} and I am studying {}", self.name, self.major)
    }
}
```