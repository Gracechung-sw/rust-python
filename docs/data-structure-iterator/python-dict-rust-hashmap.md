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

# 해시맵
키와 밸류를 묶어서 관리하는 자료형으로, **키에 대응하는 밸류를 빠르게 찾을 수 있는 장점**이 있습니다. 특히 **데이터를 인덱스로 관리하지 않는 경우에 유용**

### 해시맵 선언, 해시맵 값 참조. 
#### Python
파이썬에서는 해시맵을 딕셔너리로 구현
```python
songs = {
    "Toto": "Africa",
    "Post Malone": "Rockstar",
    "twenty one pilots": "Stressed Out",
}
print("----- Playlists -----")
if "Toto" in songs and "Africa" in songs.values():
    print("Toto's africa is the best song!")

songs["a-ha"] = "Take on Me"  # Insert
songs["Post Malone"] = "Happier"  # Update

for artist, title in songs.items():
    print(f"{artist} - {title}")
print("---------------------")

songs.pop("Post Malone")  # Delete
print(songs.get("Post Malone", "Post Malone is not in the playlist"))
```

#### Rust
HashMap 을 이용해 구현
```rust
use std::collections::HashMap;

fn main() {
    // Rust's HashMap does not keep the insertion order.
    let mut songs = HashMap::from([
        ("Toto", "Africa"),
        ("Post Malone", "Rockstar"),
        ("twenty one pilots", "Stressed Out"),
    ]);
    println!("----- Playlists -----");
    if songs.contains_key("Toto") && songs.values().any(|&val| val == "Africa") {
        println!("Toto's africa is the best song!");
    }

    songs.insert("a-ha", "Take on Me"); // Insert
    songs.entry("Post Malone").and_modify(|v| *v = "Happier"); // Update

    for (artist, title) in songs.iter() {
        println!("{} - {}", artist, title);
    }

    println!("---------------------");
    songs.remove("Post Malone"); // Delete
    println!(
        "{:?}",
        songs
            .get("Post Malone")
            .unwrap_or(&"Post Malone is not in the playlist")  // unwrap_or(&...) 는 앞의 코드가 에러를 발생시켰을 때 처리하는 방법으로, 자세한 문법은 에러 처리를 배울 때 다룸. 
    );
}
```