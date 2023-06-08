# Iterator
반복 가능한 시퀀스(sequence)를 입력으로 받아 각 원소에 특정 작업을 수행할 수 있도록 하는 기능

### Python
python의 generator, iterator에 대해서도 적기. 

### Rust

#### iter과 소유권
```rust
fn main() {
    let names = vec!["james", "cameron", "indo"];
    for name in names {
        println!("{}", name);
    }
    println!("{:?}", names);
}

// 실행 결과
// error[E0382]: borrow of moved value: `names`
//    --> src/main.rs:6:22
//     |
// 2   |     let names = vec!["james", "cameron", "indo"];
//     |         ----- move occurs because `names` has type `Vec<&str>`, which does not implement the `Copy` trait
// 3   |     for name in names {
//     |                 -----
//     |                 |
//     |                 `names` moved due to this implicit call to `.into_iter()`
//     |                 help: consider borrowing to avoid moving into the for loop: `&names`
// ...
// 6   |     println!("{:?}", names);
//     |                      ^^^^^ value borrowed here after move
//     |

```
println!("{:?}", names); 여기서 에러가 발생하는 이유는 for문의 names 가 암묵적으로 into_iter() 를 호출한다. 
이때 이 into_iter() 메소드는 터 원소의 값을 for 루프 안으로 가져와 반복하는 역할을 수행한다. 
즉, for문 안으로 각 원소(name)의 소유권이 이동하고 for문이 끝나면서 없어지는데, 
밖에 println!("{:?}", names); 에서 참조하려고 하니까 에러가 발생하는 것이다. 

이를 해결하기 위해서 iter()를 사용하도록 명시해줘야 한다. 
```rust
fn main() {
    let names = vec!["james", "cameron", "indo"];
    for name in names.iter() {
        println!("{}", name);
    }
    println!("{:?}", names);
}

// 실행 결과
// james
// cameron
// indo
// ["james", "cameron", "indo"]
```


#### iter 의 특징 
rust에서 `iter()` 메소드는 선언 즉시 원소를 내놓는 것이 아니라, 값이 필요해지면 그때 원소를 리턴한다. 
```rust
fn main() {
    let names = vec!["james", "cameron", "indo"];
    let names_iter = names.iter();
    for name in names_iter {
        println!("{}", name);
    }
    println!("{:?}", names);
}
```

## Iterator 관련 method
### sum
#### Python
```python
nums = [1, 2, 3]

sum_result = sum(nums)
print(sum_result) # 6
```

#### Rust
러스트는 iterator에서 sum 메소드를 호출한다. 
```rust
fn main() {
  let num = vec![1, 2, 3];

  let sum_result: i32 = num.iter().sum();
  println!(sum_result) // 6
}
```
### min, max
min, max도 sum과 마찬가지. 
#### Python
```python
nums = [1, 2, 3]

max = max(nums)
min = min(nums)
```

#### Rust
러스트는 iterator에서 sum 메소드를 호출한다. 
```rust
fn main() {
  let num = vec![1, 2, 3];

  let max = num.iter().max().unwrap();
  let min = num.iter().min().unwrap();
}
```

## 새로운 이터레이터를 만드는 메소드들
### enumerate, zip
- `enumerate`: index와 원소를 함께 반복
- `zip`: 두 sequence(ex. 리스트) 의 원소를 순서대로 함께 묶어서 반복
#### Python
```python
nums1 = [1, 2, 3]
nums2 = [4, 5, 6]

enumer = list(enumerate(nums1))
print(enum) # [(0, 1), (1, 2), (2, 3)]

zip = list(zip(nums1, nums2))
print(zip) # [(1, 4), (2, 5), (3, 6)]
```

#### Rust
```rust
fn main() {
  let nums1 = vec![1, 2, 3];
  let nums2 = vec![4, 5, 6];

  let enumer: Vec<(usize, &i32)> = nums1.iter().enumerate().collect();
  println!("{:?}", enumer); // [(0, 1), (1, 2), (2, 3)]

  let zip: Vec<(&i32, &i32)> = nums1.iter().zip(nums2.iter()).collect();
  println!("{:?}", zip); // [(1, 4), (2, 5), (3, 6)]
}
```
### map (중요)
- map: 주어진 함수를 sequence의 각 원소에 적용
#### Python
```python
nums = [1, 2, 3]

f = lambda x: x + 1

print(list(map(f, nums))) # [2, 3, 4]
```
#### Rust
```rust
fn main() {
    let nums: Vec<i32> = vec![1, 2, 3];

    let f = |x: &i32| x + 1;

    let maps: Vec<i32> = nums.iter().map(f).collect();
    println!("{:?}", maps); // [2, 3, 4]
}

```
### filter (중요)
- `filter`: 주어진 sequence에서 기준에 맞는(True/False 중 True인 것들) 결과만 남김.
#### Python
```python
nums = [1, 2, 3]

print(list(filter(lambda x: x % 2 == 1, nums))) # [1, 3]
```
#### Rust
```rust
fn main() {
    let nums: Vec<i32> = vec![1, 2, 3];

    let filters: Vec<i32> = nums.into_iter().filter(|x| x % 2 == 1).collect(); // 엇 여기 filer에서는 into_iter()를 사용하네? 왜일까? 아래 설명. 
    println!("{:?}", filters); // [1, 3]
}
```
filter의 경우, **기존의 원소 값을 이동해서 새로운 벡터를 만들기 때문에** 문에 into_iter 메소드로 이터레이터를 만듬. 
> 기존의 원소 값을 이동해서 새로운 벡터를 만들기 때문에???? 이해 안 감. 