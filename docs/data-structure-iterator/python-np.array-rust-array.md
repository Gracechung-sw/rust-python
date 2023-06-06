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

## 배열 Array
리스트(Python의 list, Rust의 vec)와 배열(Python의 numpy array와 Rust의 array)의 차이점은 
**같은 타입의 값이 모여 있는 길이가 고정된 자료형** 이라는 것.

그럼 배열(array)이 리스트(list)에 비해 가지고 있는 장점은 뭘까?
rust에서는 **배열을 사용하면 벡터와 다르게 메모리가 스택 영역에 저장되기 때문에 빠르게 값에 접근할 수 있다는 장점**이 있다. (벡터는 길이의 가변성 때먼에 힙 영역을 사용한다고 ./python-list-rust-vec.md에 적은 바 있다. )

### 배열 선언
#### Python
파이썬에서 비슷한 내장 자료형은 없지만, 넘파이(numpy)의 배열(array)가 가장 이와 유사합니다. 넘파이는 내부적으로 C로 구현된 배열을 가지고 있고, 파이썬에서 이 배열의 값을 꺼내서 사용하는 방식으로 동작합니다. 넘파이 배열을 이용해 열두 달을 나타내면 다음과 같습니다.
```python
import numpy as np

months = np.array(
    [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ]
)
print(months)

# full 함수를 사용하면 배열을 간단하게 한 번에 초기화할 수 있습니다.
nums = np.full(5, 3)
print(nums) # [3 3 3 3 3]

```
#### Rust
러스트의 배열의 길이는 처음 선언된 이후 변경할 수 없습니다. 
```rust
fn main() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("{:?}", months);
}
// Python의 numpy full과 같은 선언 방식. [3; 5] 와 같이 표기하면 숫자 3을 5번 나열하라는 의미
fn main() {
    let nums = [3; 5];
    println!("{:?}", nums);
}

```


### 배열 원소 참조
리스트와 마찬가지로 인덱스를 통해 접근 가능. 
#### Python
```python
import numpy as np

nums = np.full(5, 3)
nums[1] = 1
print(nums)
```
**같은 타입의 값이 모여 있는 길이가 고정된 자료형** 이라는 특징에 따라
넘파이 배열의 길이보다 큰 값을 참조하려고 하면 에러가 발생합니다.
```python
import numpy as np

nums = np.full(5, 3)
print(nums[5])
# 실행 결과
# Traceback (most recent call last):
#   File "/Users/code/temp/python/main.py", line 4, in <module>
#     print(nums[5])
# IndexError: index 5 is out of bounds for axis 0 with size 5

```

#### Rust
```rust
fn main() {
    let mut nums = [3; 5]; // 배열 원소를 수정해야 하기 때문에 nums 배열을 가변 변수로 선언
    nums[1] = 1;
    println!("{:?}", nums);
}
```
**같은 타입의 값이 모여 있는 길이가 고정된 자료형** 이라는 특징에 따라
컴파일 시 인덱스가 범위를 벗어난다는 에러가 발생합니다.
```rust
fn main() {
    let nums = [3; 5];
    println!("{}", nums[5]);
}

// 실행결과
// Compiling rust_part v0.1.0 (/Users/code/temp/rust_part)
// error: this operation will panic at runtime
//  --> src/main.rs:3:20
//   |
// 3 |     println!("{}", nums[5]);
//   |                    ^^^^^^^ index out of bounds: the length is 5 but the index is 5
//   |
//   = note: `#[deny(unconditional_panic)]` on by default

// error: could not compile `rust_part` due to previous error
```

## 결론! 중요!

Rust에서 컴파일 시 인덱스가 범위를 벗어난다는 에러가 발생하는데, 
아래 코드와 같이  미리 참조할 배열 인덱스를 컴파일러가 알 수 없는 경우,
컴파일 에러가 아닌 런타임 에러가 발생한다. 
```rust
fn main() {
    let nums = [3; 5];
    for i in 0..nums.len() + 1 {
        println!("{}", nums[i]);
    }
}

// 실행결과
// 3
// 3
// 3
// 3
// 3
// thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 5', src/main.rs:4:24
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

따라서
**데이터의 길이가 컴파일 타임에 정해지는 경우에는 배열을, 데이터의 길이가 런타임에 정해지는 경우에는 벡터를 사용합니다.**

