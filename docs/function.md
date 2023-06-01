# 함수
## 1. 함수 선언
타입 명시와 함께 함수 선언
### Python
def 키워드 사용
```python
def add(num1: int, num2: int) -> int:
    return num1 + num2

```
### Rust
fn 키워드 사용
```rust
fn add(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}
```
> 주의: 이때 주의해야 하는 점은 파이썬에서는 타입을 생략할 수 있지만, 러스트에서는 반드시 파라미터와 리턴 타입을 명시해야 한다는 것입니다. 타입이 잘못되거나 표기되지 않았다면 컴파일되지 않습니다.

> Rust에서는 return을 생략할 수는 있지만 가독성 측면에서 적어주는게 좋을 것 같음. 

## 2. 여러 개의 값 리턴시 타이핑
### Python
```python
def swap(num1: int, num2: int) -> tuple[int, int]:
    return num2, num1


num1, num2 = swap(1, 2)
print(f"{num1}, {num2}")

```
### Rust
```rust
fn swap(num1: i32, num2: i32) -> (i32, i32) {
    (num2, num1)
}

fn main() {
    let (num1, num2) = swap(1, 2);
    println!("{num1}, {num2}");
}
```

## 3. Scope
> Scope: 변수에 접근할 수 있는 범위
### Python
기본적으로 scope를 함수 단위로 구분.
> 실제로는 파이썬은 LEGB 룰이라고 불리는 좀더 복잡한 스코프 규칙을 가지고 있지만, 여기서는 단순화해서 함수 기준으로 설명합니다.
> LEGB가 뭔지 알아보기. 

```python
def hello(name: str):
    num = 3
    print(f"Hello {name}")


if __name__ == '__main__':
    my_name = "buzzi"

    if True:
        print("My name is", my_name)
        my_name = "mellon" #  파이썬은 스코프를 함수 단위로만 구분하고 있기 때문에 이제 코드 전체에서 값이 바뀌게 됩니다. 따라서 hello(my_name)의 출력은 Hello mellon이 됩니다.

    hello(my_name)

    # print(num) # error # hello 함수 안에서 선언된 num 이라는 변수를 프린트하기 때문입니다. 즉, num 의 스코프가 hello 함수이기 때문에 함수 바깥에서 참조할 수 없는 것입니다.

# 실행결과
# My name is buzzi
# Hello mellon
```

### Rust
scope를 중괄호 "{}" 기준으로 구분.->  **소유권 모델과 밀접한 연관**이 있으므로 매우 중요. 
```rust
fn hello(name: String) {
    let num = 3;
    println!("Hello {}", name);
}

fn main() {
    let my_name = "buzzi".to_string();

    {
        println!("My name is {}", my_name);
        let my_name = "mellon"; // 중괄호를 벗어나면 my_name의 스코프가 끝나게 됨. 
    }

    hello(my_name);

    // println!("{}", num); // error
}

// 실행 결과
// My name is buzzi
// Hello buzzi
```