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

## 문자열
### Rust
rust에서 문자열을 만드는 방법
방법 1. 코드가 컴파일 될 때 스택 영역에 만들어지는 `str`(문자열 리터럴)
- str 타입은 한 번 만들어지면 값을 변경하거나 길이를 바꿀 수 없습니다.
```rust
fn main() {
    let s = "hello";
    println!("{}", s);
}
```

방법 2. 런타임에 힙 영역에 메모리가 할당되는 `String`
- 벡터와 마찬가지로 동적으로 값을 바꾸거나 길이를 바꿀 수 있습니다.
```rust
fn main() {
    // 비어 있는 스트링 만들기
    let mut s = String::new();

    // 스트링 리터럴로부터 스트링 만들기
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();

    // String::from()을 사용하여 스트링 만들기
    let s = String::from("initial contents");
}

```


-> 그럼 메모리를 어떻게 쓸건지 개발자가 결정해서 이에 맞게 선언해줘야 한다?!
-> 맞다
### 정리! 중요!
- 문자열 데이터의 소유권을 다뤄야 하는 경우 String을 사용
- 문자열의 값만 필요한 경우 &str을 사용

### 문자열 슬라이스
#### Python
Slice (슬라이스)를 사용

- 슬라이스는 시퀀스 객체(예: 문자열, 리스트)에서 일부를 추출하는 작업을 의미합니다.
- 슬라이스는 start:stop:step의 구문을 사용하여 작성됩니다.
- start 인덱스부터 stop 인덱스 전까지의 요소를 추출합니다. start 인덱스의 요소는 포함하고, stop 인덱스의 요소는 포함하지 않습니다.
- step 인덱스는 선택적이며, 추출할 요소 사이의 건너뛸 간격을 나타냅니다.
- 슬라이스는 기존 시퀀스 객체의 일부분을 추출하여 새로운 시퀀스 객체로 반환합니다. 
```python
string = "Hello, World!"

# 문자열의 첫 5글자를 추출
substring1 = string[:5]
print(substring1)  # 출력: "Hello"

# 문자열의 7번째부터 마지막까지 추출
substring2 = string[7:]
print(substring2)  # 출력: "World!"

# 문자열의 2번째부터 8번째까지 추출
substring3 = string[1:8]
print(substring3)  # 출력: "ello, W"

# 문자열의 뒤에서 6번째부터 끝까지 추출
substring4 = string[-6:]
print(substring4)  # 출력: "World!"

# 문자열의 처음부터 끝까지 건너뛰며 추출
substring5 = string[::2]
print(substring5)  # 출력: "HloWrd"

# 문자열을 거꾸로 출력
substring6 = string[::-1]
print(substring6)  # 출력: "!dlroW ,olleH"

```
슬라이스는 원본 문자열을 수정하지 않고 새로운 문자열을 반환하기 때문에 원본 문자열은 변경되지 않습니다. 슬라이스를 통해 원하는 범위를 선택하고, 필요에 따라 문자열을 자를 수 있습니다.

#### Rust
&str 은 문자열의 일부분을 의미하기도 합니다. 따라서 &str을 문자열 슬라이스라고 부릅니다. String 타입으로 문자열을 선언하고, 해당 문자열로부터 문자열 슬라이스를 만들어 프린트해 보겠습니다.
```rust
fn main() {
    let greet = String::from("Hi, buzzi!");
    // let name = "buzzi!";
    let name = &greet[4..];
    println!("{}", name);
}

// 실행 결과
// buzzi!
```

문자열 슬라이스를 사용할 때 주의해야 하는 점은, **러스트의 모든 문자열은 UTF-8로 인코딩**되어 있다는 점입니다. 실제로 문자열 슬라이스의 인덱스는 문자 단위가 아닌 바이트 스트림의 바이트 단위 입니다. 아래 예제를 살펴봅시다.
```rust
fn main() {
    let greet = String::from("Hi😃 buzzi!");
    let name = &greet[4..];
    println!("{}", name);
}
// 실행 결과

// thread 'main' panicked at 'byte index 4 is not a char boundary; it is inside '😃' (bytes 2..6) of `Hi😃 buzzi!`', src/main.rs:4:17
```


일반적인 알파벳 문자는 바이트 스트림에서 1바이트를 차지하지만, 유니코드로 만들어진 이모지의 경우는 4바이트를 차지하기 때문입니다. 바이트 4에 해당하는 인덱스가 이모지 중간에 위치하므로 정상적으로 문자열을 잘라낼 수 없게 됩니다. 따라서 **반드시 스트링을 문자 단위로 슬라이스하고 싶은 경우라면 문자열을 벡터로 만들어줘야 합니다.**

```rust
fn main() {
    let greet = String::from("Hi😃 buzzi!");
    let greet_chars: Vec<char> = greet.chars().collect();
    let name = &greet_chars[4..].iter().collect::<String>();
    println!("{:?}", name);
}
```