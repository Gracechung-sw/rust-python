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

## 자료형
###  Python - list
// python list에 대해 <전문가를 위한 파이썬> 보고 공부한 것 여기 적어두기. 

### Rust - Vec (벡터)
- 여러 개의 값을 하나로 묶어서 사용할 때 사용하는 자료구조
- **길이를 런타임에 동적으로 변경 가능** -> 이를 위해 런타임시 값이 **힙 영역**에 저장됨.

### 벡터 선언
방법 1. Vec 구조체의 from 메소드를 사용해 배열로부터 벡터를 만드는 방법
방법 2. vec! 매크로를 사용해 벡터를 만드는 방법
```rust
fn main() {
    let vec1 = Vec::from([1, 2, 3]); // 방법 1
    let vec2 = vec![1, 2, 3]; // 방법 2
}
```
값이 비어있고 벡터를 선언하는 경우에는 컴파일러가 원소로부터 타입을 추론할 수 없기 때문에 반드시 타입을 명시해야 한다. 
```rust
fn main() {
    let vec3: Vec<i32> = Vec::new();
    let vec4: Vec<i32> = vec![];
}
```

### 벡터 원소에 접근하기

벡터의 원소는 인덱스(index)를 사용해 접근할 수 있음. 
#### Python
```python
vec1 = [1, 2, 3]
num = vec1[1]

print(num)
# 실행 결과
# 2
```

#### Rust
```rust
fn main() {
    let vec1 = vec![1, 2, 3];

    let num = vec1[1];

    println!("{}", num);
}
// 실행 결과
// 2
```
### 벡터에 값 추가하기 
#### Python
python은 list에 값 추가할 때 append를 쓴다. 
```python
vec1 = [1, 2, 3]
vec1.append(4)
vec1.append(5)
vec1.append(6)

print(vec1)
# 실행결과
# [1, 2, 3, 4, 5, 6]
```
#### Rust
push 메소드를 사용하면 원소를 벡터 마지막에 하나씩 추가할 수 있습니다. 
> 주의!  하는 점은 벡터 vec1 이 변경되기 때문에 처음에 vec1을 가변 변수(mut 키워드 사용)로 선언해야 한다

> 주의! 벡터를 프린트할 때는 디버그 모드를 사용해야 합니다. 따라서 서식을 "{:?}"로 사용해야 합니다.

> 왜? ChatGPT에게 물어보았다. 

rust에서 왜 벡터를 프린트할 때는 디버그 모드를 사용해야해?
ChatGPT
Rust에서 벡터를 프린트할 때 디버그 모드를 사용하는 이유는 Rust의 기본 프린트 포맷터인 Debug 트레이트와 Display 트레이트의 동작 방식 때문입니다.

Rust는 Debug 트레이트를 사용하여 디버깅 용도로 값을 출력합니다. Debug 트레이트는 println!("{:?}", value)와 같은 형식으로 사용됩니다. 이 트레이트는 값의 내부 표현을 정확하게 보여주기 위해 노력합니다. 벡터를 Debug 포맷으로 출력할 때는 벡터의 요소들을 모두 출력하므로, 벡터가 매우 큰 경우 출력이 매우 길어질 수 있습니다.

반면에 Display 트레이트는 사용자 친화적인 형식으로 값을 출력합니다. println!("{}", value)와 같은 형식으로 사용됩니다. Display 트레이트는 보다 가독성 좋은 출력을 위해 최적화되어 있습니다. 따라서 벡터를 Display 포맷으로 출력할 때는 벡터의 요소를 일부 생략하고 요약된 형태로 출력될 수 있습니다.

기본적으로, 벡터의 Debug 포맷 출력은 개발자가 디버깅하는 데 유용하도록 설계되었습니다. 반면에 Display 포맷 출력은 최종 사용자에게 보여지는 용도로 사용됩니다.

벡터를 println!("{:?}", vector)와 같은 형식으로 출력하면 벡터의 내부 표현을 정확하게 확인할 수 있습니다. 하지만 출력이 매우 길어질 수 있으므로, 실제 사용자에게 보여질 때는 println!("{}", vector)와 같은 형식으로 출력하는 것이 일반적입니다.

따라서, 디버그 모드에서 벡터를 프린트할 때는 println!("{:?}", vector)와 같은 형식을 사용하고, 프로덕션 환경에서는 println!("{}", vector)와 같은 형식을 사용하는 것이 권장됩니다.


### 벡터에서 값 삭제하기
#### Python
pop 메소드를 사용한다. 
```python
vec1 = [1, 2, 3]
num1 = vec1.pop()
num2 = vec1.pop(0)

print(num1, num2, vec1)
# 실행 결과
# 3 1 [2]
```

## 데크 (Deque)
참고로 파이썬의 리스트와 러스트의 벡터 모두 맨 앞의 원소를 제거하는 데 시간 복잡도가 $O(n) 만큼 소요되기 때문에 맨 앞에서 원소를 자주 제거해야 한다면 데크(deque)를 사용하는 것이 좋습니다. 
> 왜 파이썬의 리스트와 러스트의 벡터 모두 맨 앞의 원소를 제거하는 데 시간 복잡도가 $O(n)???
#### Python
```python
from collections import deque

deq = deque([1, 2, 3])
print(deq.popleft())
```
#### Rust
```rust
use std::collections::VecDeque;

fn main() {
    let mut deq = VecDeque::from([1, 2, 3]);
    println!("{}", deq.pop_front().unwrap());
}
```


