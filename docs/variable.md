# 변수

## 1. 변수 선언과 값 출력

### 1) 값 출력
#### Python
모든 객체를 print 함수로 출력할 수 있음.
```python
print("Hello, world!")
```
#### Rust
러스트에서는 매크로(macro)를 사용해 값을 출력한다.
> 매크로란 사전 정의된 편리한 기능을 의미하고, 항상 이름 뒤에 !가 붙음. 
> 주의: 러스트 코드는 매 줄의 마지막에 세미콜론(;) 이 붙습니다. 세미콜론이 없으면 컴파일 에러가 발생
```rust
fn main() {
    println!("Hello, world!");
}
```

### 2) 변수 선언
#### Python
파이썬은 변수 선언 시 타입을 명시하지 않아도 됨.
```python
x = 1.0
y = 10

print(f"x = {x}, y = {y}")
```
#### Rust
러스트에서는 let 키워드를 사용해 변수를 선언합니다. 그리고 **타입을 : 뒤에 명시**합니다.
```rust
// 변수명  타입   값
let x: i32 = 10;
```

### 3) naming convention

|  |Python|Rust|
|---|---|---|
|변수|snake_case = 3|let snake_case = 3;|
|함수|def my_function|fn my_function|
|클래스/구조체|class MyClass|struct MyStruct|
|상수|SCREAMING_SNAKE_CASE = 1|const SCREAMING_SNAKE_CASE: i32 = 1;|

---

## 2. 변수의 불변성
#### Python
파이썬에서는 변수를 선언한 다음 다른 값을 넣는 것이 매우 자유롭습니다. 변수의 타입도 상관 없이 새로운 값을 마음대로 넣을 수 있습니다.
```python
x = 1
x = "2"
x = 3.141592
# 에러 안 남. 
```

#### Rust
Python에는 없는 두 개념이 Rust에는 있음. 
러스트의 모든 변수는 기본적으로 불변(immutable). 예를 들어, 아래 코드와 같이 let 키워드로 변수를 선언하고, 해당 변수의 값을 바꾸려고 한다면 컴파일이 되지 않습니다.
```rust
fn main() {
    let x = 1;
    x = 2; // won't compile!
    println!("{}", x);
}

// 아래와 같은 에러 발생
// error[E0384]: cannot assign twice to immutable variable `x`
//  --> src/main.rs:3:5
//   |
// 2 |     let x = 1;
//   |         -
//   |         |
//   |         first assignment to `x`
//   |         help: consider making this binding mutable: `mut x`
// 3 |     x = 2; // won't compile!
//   |     ^^^^^ cannot assign twice to immutable variable

```

따라서 변수(mutable)로 선언하고 싶다면 `mut` 키워드로 가변성을 부여해야함. 

```rust
fn main() {
    let mut x = 1;
    x = 2;
    println!("{}", x);
}
```

