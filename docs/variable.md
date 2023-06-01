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

## 3. 섀도잉
> 섀도잉: 이미 선언된 변수명을 재사용해서 새로운 변수를 다시 선언하는 것. 
#### Rust
섀도잉을 사용할 경우, mut 키워드 없이도 새로운 값을 변수에 할당할 수 있고, 새로운 변수이기 때문에 타입도 변경할 수 있습니다. 아래 예제에서는 변수 x 에 처음에는 "5" 라는 문자열을 할당했지만, 그 다음에는 섀도잉을 사용해 x에 정수 6을 할당했습니다. 코드를 실행해보면 정상적으로 컴파일됩니다.

```rust
fn main() {
    let x = "5";

    let x = 6; // x is redeclared as 6

    println!("The value of x is: {}", x); // 6
}
```

## 4. 타입
#### Python
엄청 단순 
```python
int_var: int = 8
str_var: str = 'hello'
float_var: float = 88.9
bool_bar: bool = True
```
#### Rust
러스트의 원시 타입(primitive type) 목록

| 이름 | 타입 | 이름 | 타입 |
| ---- | ---- | ---- | ---- |
| 8비트 정수 | `i8` | 부호 없는 32비트 정수 | `u32` |
| 16비트 정수 | `i16` | 부호 없는 64비트 정수 | `u64` |
| 32비트 정수  | `i32` | 부호 없는 128비트 정수 |`u128` |
| 64비트 정수  | `i64` | 부호 없는 아키텍처 |`usize`  |
| 128비트 정수 | `i128` | 불리언 | `bool` |
| 아키텍처 | `isize` | 문자열 | `String` |
| 부호 없는 8비트 정수 | `u8` | 문자열 슬라이스 | `str` |
| 부호 없는 16비트 정수 |`u16` | 32비트 부동소수점 실수 | `f32` |
|  | |64비트 부동소수점 실수 | `f64` |

## 5. 타입 추론
#### Python
type을 잘 못 적어도 에러를 뱉지는 않는다. ex. int_var: str = 8 이렇게 해도 에러 안 뱉음. 
```python
int_var: str = 8 # 이러면 에러는 발생하지 않지만 vscode editor 단에서 
# Expression of type "Literal[8]" cannot be assigned to declared type "str"
#   "Literal[8]" is incompatible with "str"PylancereportGeneralTypeIssues
```
이런 type hint는 알 수 있음. 

Rust에 rust-analyzer가 있다면 python에는 mypy, pyright 같은 것들이 있음. 
mypy, pyright 설치와 사용법 & 주의사항

```bash
# mypy 공식 문서: https://mypy.readthedocs.io/en/stable/getting_started.html
$ pip install mypy
# 사용방법: mypy [타입 체크가 필요한 파일명]

# pyright 공식 문서: https://github.com/microsoft/pyright
$ sudo npm install -g pyright
# 사용방법: pyright [타입 체크가 필요한 파일명]  
```  
#### Rust

러스트 코드를 작성할 때 대부분의 경우에는 개발자가 변수에 타입을 지정하지 않아도 앞에서 설치한 rust-analyzer가 알아서 타입을 추론(inference)해서 화면에 보여줍니다. 비슷한 원리로 코드가 컴파일될 때에는 컴파일러가 타입을 추론해서 변수를 선언하게 됩니다. 이때, 추측되는 타입의 기본값은 정수형은 i32 , 실수형은 f64 입니다.

## 6. 타입 변환(Casting)

#### Python
파이썬에서는 타입 이름을 바로 사용해 타입 변환을 수행합니다.
```python
x = 1.2
y = int(x)
print(f"{x} -> {y}");
# 실행 결과: 
# 1.2 -> 1
```
#### Rust
`as` 키워드 사용. 
``` Rust
fn main() {
    let x: f64 = 1.2;
    let y = x as i32;
    println!("{} -> {}", x, y);
}
// 실행 결과: 
// 1.2 -> 1
```

## 7. 상수
#### Python
Python에서는 상수를 선언해도 변경할 수 있음. 
#### Rust
```rust
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    println!("{}", THRESHOLD);
    println!("{}", is_big(5));
}

```
컴파일러가 친절하게 상수 THRESHOLD 에는 새로운 값을 할당할 수 없다고 알려주게 됩니다.