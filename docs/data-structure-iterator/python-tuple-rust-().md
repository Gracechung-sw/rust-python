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

## 튜플
값들을 순서대로 나열해 저장하는 데이터 구조입니다. 파이썬과 러스트 모두 튜플 자료형을 가지고 있습니다.

**python의 튜플과 rust의 튜플의 차이점**
- 튜플의 크기는 python, rust 모두 바꿀 수 없음
- 튜플 원소의 내용은 python에서는 바꿀 수 없지만 rust에서는 바꿀 수 있음. 단, 타입은 같아야함. 
```rust
fn main() {
    let mut tup1 = (0, 0.1, "hello");

    let mut x = tup1.0;
    let (_, mut y, _) = tup1;

    x = 1;
    y = 1.1;

    println!("{:?} {} {}", tup1, x, y);

    tup1.0 = 3;
}
```


### 튜플 선언
#### Python
파이썬의 튜플은 소괄호 안에 콤마로 구분된 값을 넣어서 선언
```python
tup1 = (0, 0.1, "hello")
tup2 = (1, 1.01, "bye")

_, y, _ = tup2

print(f"tup1 has {tup1} and the value of y is {y}")
```

#### Rust
러스트의 튜플도 소괄호 안에 콤마로 구분된 값을 넣어서 선언합니다. 변수의 타입을 컴파일러가 추론하는 것처럼 튜플의 타입도 컴파일러가 추론하기 때문에 타입을 명시할 필요가 없습니다. 하지만 타입을 직접 명시해도 상관없습니다.
```rust
fn main() {
    let tup1 = (0, 0.1, "hello");
    let tup2: (i32, f64, &str) = (1, 1.01, "bye");

    let (_, y, _) = tup2;

    println!("tup1 has {:?} and the value of y is: {}", tup1, y);
}
```

### 튜플 원소 참조
#### Python
리스트, numpy array과 마찬가지로 인덱스를 통해 접근 가능. 
```python
tup1 = (0, 0.1, ("hello", "world"))

print(tup1[2][0], tup1[2][1])

```

#### Rust
**튜플 이름 뒤에 점(.)을 붙이고 그 뒤에 인덱스를 입력합니다. 만일 다중 튜플인 경우, 점을 한번 더 찍고 인덱스를 입력**하면 됩니다.
```rust
fn main() {
    let tup1 = (0, 0.1, ("hello", "world"));

    println!("{} {}", tup1.2 .0, tup1.2 .1);
}
```