# ownership (소유권)


## 메모리 관리
메모리라는 자원은 한정되어 있기 때문에 프로그래밍 언어들은 각자의 방식으로 이 메모리를 효율적으로 관리

### 메모리 구조(스택과 힙)
- 스택: 스택 영역은 함수가 실행될 때 사용하는 메모리 공간으로, 함수에서 사용하는 지역 변수가 스택에 저장됩니다. 일반적으로 스택에서 사용될 메모리 공간이 미리 정해지기 때문에 매우 빠르게 값을 저장하고 접근할 수 있습니다. 만일 함수 실행이 종료되면 스택 영역에서 사용된 모든 지역 변수는 메모리에서 삭제됩니다. 
- 힙: 힙 영역은 동적으로 할당되는 메모리를 위해 존재하는 공간으로, 개발자가 명시적으로 특정 크기의 메모리 공간을 사용하겠다고 선언해야 합니다. 만일 해당 메모리 공간이 더 이상 필요하지 않은 경우에는 해당 메모리를 할당 해제해주어야 합니다. 왜냐하면 이미 점유된 메모리 공간은 다른 프로그램이나 스레드에서 사용할 수 없기 때문입니다.



### Python
**`가비지 콜렉터`를 이용해 언어 차원에서 자동으로 메모리를 관리.**
Python은 모든 객체의 데이터를 힙 영역에 저장.

**파이썬은 스택을 사용하지 않고 모든 객체를 힙 영역에 저장**. 이렇게 저장된 객체들은 파이썬에서 가비지 콜렉션을 통해 메모리를 관리하기 때문에 파이썬을 사용할 때는 메모리 관리에 신경쓰지 않아도 됩니다. 위에서 힙 영역에 대해서 설명할 때 언급한 개발자가 **할당하고 할당 해제하는 메모리를 파이썬의 가비지 콜렉터가 대신**해주는 것입니다.


이 가비지 컬렉터가 런타임에 사용되지 않는 객체가 있으면 주기적으로 객체를 삭제
또는 메모리 사용량이 너무 높은 경우에도 삭제.

> 드디어 '가비지 컬렉터' 라는 용어가 나왔다. 
> docs/Intro_why_rust.md 의 <CPU 연산이 많이 필요한 코드를 러스트로 교체하면 빠르게 동작하는 프로그램을 만들 수 있습니다.> 에서  러스트와 자주 비교되는 언어인 고(Go)와 다르게, 러스트에는 소유권(ownership) 모델을 사용해서  가비지 콜렉터가 없기 때문에 훨씬 좋은 성능을 내게 됩니다. 이러한 특징 때문에 퍼포먼스가 매우 중요한 서비스에 자주 사용됩니다.
> 라는 말에 대해서 이제야 이해할 준비가 되었다. 

위 '러스트에는 소유권(ownership) 모델을 사용해서  가비지 콜렉터가 없기 때문에 훨씬 좋은 성능을 내게 됩니다.' 라는 말을 거꾸로 하면 Python에서는 가비지 컬렉터가 메모리를 관리하기 때문에 성능이 떨어진다.(느리다.) 가 된다. 
가비지 컬렉터가 무슨 영향을 주길래 느려지는 걸까? 
이유는 
1. 가비지 콜렉션이 수행되는 동안에는 다른 파이썬 코드가 실행될 수 없기 때문에 파이썬의 코드 실행 속도가 느려지는 원인이 됩니다. 
2. 어떤 객체가 언제 메모리에서 할당 해제되는지를 개발자가 명시적으로 알 수 있는 방법이 없고 가비지 콜렉터가 이를 전담하기 때문에 프로그램이 불필요하게 많은 메모리를 사용할 가능성도 있습니다.

### Rust
**`소유권(Ownership)`이라는 개념을 통해 메모리를 관리** 러스트에서는 어떤 값이 더이상 사용되지 않는지를 소유권을 사용해 판단합니다. 모든 값에 소유자를 지정하고, 이 값을 소유하고 있는 소유자가 없게 되면 즉시 값이 메모리에서 할당 해제되는 원리

**러스트는 스택 영역과 힙 영역 모두를 사용**합니다. 러스트는 기본적으로 아래와 같이 **함수에서 사용하는 모든 값을 제한된 크기의 스택 영역에 저장**합니다. 따라서 함수 호출이 종료되면 지역 변수 foo 와 var는 모두 삭제됩니다.
```rust
fn foo() {
    let foo = "foo";
    let var = 5;
}
```
**힙 영역은 함수에서 명시적으로 선언하는 경우에만 사용**되는데, 힙 영역에 저장하는 값은 전역적으로(globally) 접근이 가능합니다. 나중에 배울 Box 타입을 사용해 선언하면 됩니다. 그리고 **멀티스레딩에서 여러 스레드가 접근하는 변수의 값은 힙 영역에 저장**되게 됩니다.

```rust
fn main() {
    let num = Box::new(1);
}
```

> 가비지 컬렉터 측면 뿐만 아니라 소유권 개념이 주는 장점
> - 메모리 안전성: 메모리 안전성이란, 하나의 값에 대해서 단 하나의 코드만 접근하기 때문에 예상치 못하게 값이 변경되는 일이 없다는 의미
> - 스레드 안전성이 보장:  여러 개의 스레드에서 하나의 값에 접근하고자 할 때 발생할 수 있는 경합 조건(Race condition)이나 데드락(Deadlock)이 발생하지 않는다는 의미입니다. 이 두 가지 문제가 멀티스레딩 프로그램을 만들 때 가장 어렵고 복잡한 문제이지만 러스트에서는 이를 컴파일 타임에 탐지할 수 있기 때문에 안정성이 보장됩니다.





## ownership 소유권 규칙
- 모든 "값"들은 해당 값을 "소유"하고 있는 소유자(Owner)가 존재합니다.
- 한 값에 하나의 소유자만 존재할 수 있습니다. 하나의 값에 두 개의 소유자가 동시에 존재할 수 없습니다.
- 소유자가 현재 코드의 스코프(./variable.md 의 Scope에서 rust 의 scope는 {} 라고 했음. )에서 벗어나면, 값은 메모리에서 할당 해제됩니다.
```rust
fn dummy(x: String) {
    println!("{}", x);
  	// x is dropped
}

fn main() {
    let x = String::from("Hello");
    dummy(x);
    println!("{}", x);  // This line won't compile
}
```
함수 dummy 에 문자열이 전달된 다음, 함수를 벗어나면 그 즉시 x 는 할당 해제됩니다. 그런데 이미 할당 해제된 x를 9번 라인에서 참조하고 있기 때문에 오류가 발생합니다. 그러면 모든 값은 다른 함수에 전달하면 영원히 사용하지 못하는 걸까요? 이런 경우 사용할 수 있는 두 가지 방법이 있습니다.

## 소유권 돌려주기
위의 예제처럼 함수 parameter에 전달함으로써 소유권이 넘어간 이후, 사용하고 나서 다시 소유권을 호출했던 곳으로 돌려주는 방법
1. 리턴해주기.
```rust
fn dummy(x: String) -> String {
    println!("{}", x);
    x
}

fn main() {
    let x = String::from("Hello");
    let x = dummy(x);
    println!("{}", x);
}
```
하지만 이 방법은 매번 함수의 리턴값을 변수로 재선언해주어야 하기 때문에 코드의 가독성이 떨어지고, 값이 어느 변수로 이동하는지를 알기 어려운 단점이 있습니다.

2. 레퍼런스와 소유권 빌리기  
러스트에는 값의 소유권을 잠시 빌려줄 수 있는 개념인 `대여(borrow)`가 있음.   
변수 앞에 `&` 키워드를 사용하면 되는데, 해당 변수의 레퍼런스(reference)를 선언한다는 의미입니다. 레퍼런스란 소유권을 가져가지 않고 해당 값을 참조할 수 있는 방법입니다. 

```rust
fn main() {
    let x = String::from("Hello");
    let y = &x; // &를 사용해서 할당했기 때문에 "Hello"의 값의 소유권은 여전히 x에 있고, y는 단순히 값을 참조만 합니다. 따라서 마지막에서 변수 x와 y를 모두 프린트해도 에러가 발생하지 않습니다.

    println!("{} {}", x, y);
}
```
이 때 y의 타입은 `&String` 으로 문자열의 레퍼런스 타입임. 

## 가변 레퍼런스
소유권을 빌려온 후(레퍼런스), 값을 변경하고 싶다면 `가변 레퍼런스`로 빌려오면 된다. 
1. dummy함수의 파라미터 y 의 타입이 &mut String 으로 변경
2. 변수 x를 가변 변수로 선언
3. dummy함수에 x를 전달할 때 가변 레퍼런스 &mut x로 전달
```rust
fn dummy(y: &mut String) {
    y.push_str(" world!");
    println!("{}", y);
    // ownership returns to `x`
}

fn main() {
    let mut x = String::from("Hello");
    dummy(&mut x);
    println!("{}", x);
}

// 실행 결과
// Hello world!
// Hello world!
```

## 다수의 가변 레퍼런스 불가
변수 x의 소유권을 한 번 이상 대여할 수 없다고 합니다. 만일 하나의 소유권을 여러 개의 변수가 빌릴 수 있다면 큰 문제가 발생할 가능성이 있습니다. 
하나의 메모리를 여러 곳에서 접근할 수 있기 때문에 버그가 발생할 수 있습니다. 예를 들어 어떤 가변 레퍼런스에서 값을 변경했는데, 다른 곳에서는 변경 전의 값을 필요로 한다면 예상치 못한 결과가 나올 수 있습니다. 따라서 러스트에서는 하나의 값에 대한 여러 개의 가변 레퍼런스를 허용하지 않습니다. 
```rust
fn main() {
    let mut x = String::from("Hello");
    let y = &mut x; // 여기도 가변 레퍼런스 하고 
    let z = &mut x; // 여기도 가변 레퍼런스 하고 

    println!("{} {}", y, z);
}

// 실행 결과

   Compiling rust_part v0.1.0 (/Users/code/temp/rust_part)
error[E0499]: cannot borrow `x` as mutable more than once at a time
 --> src/main.rs:4:13
  |
3 |     let y = &mut x;
  |             ------ first mutable borrow occurs here
4 |     let z = &mut x;
  |             ^^^^^^ second mutable borrow occurs here
5 |
6 |     println!("{} {}", y, z);
  |                       - first borrow later used here

```

하지만 가변 레퍼런스가 아니라 단순 레퍼런스는 여러개 가능. 
```rust
fn main() {
    let x = String::from("Hello");
    let y = &x;
    let z = &x;

    println!("{} {}", y, z);
}

// 실행 결과
Hello Hello
```


## 클로저와 소유권
./function.md 에서 Rust의 클로저에 대해서 진짜 간단하게만 살펴보았는데, 소유권 개념과 합쳐서 좀 더 자세히 알아보고자함. 

> 러스트에도 람다 함수와 비슷한 개념이 있는데 바로 클로저(Closure). 
> JS의 closure, 익명 함수 개념과 차이점은 없는지 알아보고 적기. 클로저는 파라미터를 `| | 의 사이에 선언하고, 그 뒤에 함수에서 리턴하는 부분을 작성`합니다.
```rust
fn main() {
    let my_func = |x| x + 1;
    println!("{}", my_func(3));
}

```
### 클로저의 역할
1. 환경 캡쳐 (Environment capture)
1) 클로저가 변수를 자신의 스코프 내부로 가져가는 방법1 - 불변 소유권 대여
클로저는 클로저가 선언된 스코프에 있는 지역 변수를 자신의 함수 내부에서 사용할 수 있는데, 이를 `환경 캡처(Environment capture)`라고 부릅니다.
```rust
fn main() {
    let multiplier = 5;

    let func = |x: i32| -> i32 { x * multiplier }; // 클로저 
    // 클로저 내에서도 multiplier 지역 변수가 사용 가능. 
    // 어떻게 사용 가능? 클로저는 multiplier를 불변 소유권 대여 방법으로 자신의 내부에서 사용한 것임. 

    for i in 1..=5 {
        println!("{}", func(i));
    }

    println!("{}", multiplier); // 👍
}

// 실행 결과
// 5
// 10
// 15
// 20
// 25
// 5
```

2) 클로저가 변수를 자신의 스코프 내부로 가져가는 방법2 - 가변 소유권 대여
```rust
fn main() {
    let mut multiplier = 5; // 가변 변수로 선언

    let mut func = |x: i32| -> i32 {
        multiplier += 1; // 클로저 내부에서 multiplier의 값을 변경
        x * multiplier
    };

    for i in 1..=5 {
        println!("{}", func(i));
    }

    println!("{}", multiplier); // 클로저 호출이 끝난 다음에도 여전히 multiplier에 접근이 가능
}

// 6
// 14
// 24
// 36
// 50
// 10

```

3) 클로저가 변수를 자신의 스코프 내부로 가져가는 방법2 - 소유권 가져가기

위 두 방법은 '대여' 였다면 이건 아에 '소유권을 가져가는 것'임. 
클로저가 같은 스코프에 선언된 지역 변수의 소유권을 가져가도록 하려면 클로저의 파라미터를 선언하는 코드 앞에 `move` 키워드를 사용하면 됩니다.
```rust
fn factory(factor: i32) -> impl Fn(i32) -> i32 { // 클로저를 리턴하는 함수 factory
    |x| x * factor
}

fn main() {
    let multiplier = 5;
    let mult = factory(multiplier);
    for i in 1..=3 {
        println!("{}", mult(i));
    }
}

//하지만 위 코드를 컴파일하면, 아래와 같은 에러가 발생합니다.

error[E0597]: `factor` does not live long enough
 --> src/main.rs:2:13
  |
2 |     |x| x * factor
  |     ---     ^^^^^^ borrowed value does not live long enough
  |     |
  |     value captured here
3 | }
  |  -
  |  |
  |  `factor` dropped here while still borrowed
  |  borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `notebook` due to previous error
```

factor 변수가 클로저 안에 캡처될 때, 소유권이 factory로부터 클로저로 대여됩니다. 하지만 factory함수가 종료되면 factor 변수의 값이 삭제되기 때문에 리턴된 클로저에서 더 이상 factor 를 사용할 수 없는 문제가 발생합니다. 이를 방지하기 위해서는 클로저 안으로 factor의 소유권을 이동시키면 됩니다. 이때 사용되는 키워드가 move입니다. move는 캡처된 변수의 소유권을 클로저 안으로 이동시킵니다.

```rust
fn factory(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

fn main() {
    let multiplier = 5;
    let mult = factory(multiplier);
    for i in 1..=3 {
        println!("{}", mult(i));
    }
}

// 실행 결과

// 5
// 10
// 15
```
클로저에서 move 를 가장 많이 사용하는 경우는 멀티스레드 혹은 비동기 프로그래밍을 작성할 때입니다.