# data for graph

|source--------|destination---------|weight|
|--------------|--------------------|------|
| Beijing      | Shenyang           | 750  |
| Shenyang     | Qingdao            | 680  |
| Beijing      | Qingdao            | 800  |
| Beijing      | Xian               | 1140 |
| Beijing      | Zhengzhou          | 650  |
| Xian         | Zhengzhou          | 570  |
| Zhengzhou    | Qingdao            | 820  |
| Zhengzhou    | Wuhan              | 530  |
| Xian         | Chengdu            | 840  |
| Chengdu      | Chongqing          | 340  |
| Chongqing    | Wuhan              | 900  |
| Zhengzhou    | Changsanjiao       | 1200 |
| Qingdao      | Changsanjiao       | 960  |
| Wuhan        | Changsanjiao       | 680  |
| Zhusanjiao   | Chongqing          | 2500 |
| Zhusanjiao   | Wuhan              | 1380 |
| Zhusanjiao   | Changsanjiao       | 2600 |

# useful data structure impl in rust (compare to cpp)
这是一个非常好的问题，对于从 C++ 转向 Rust 的开发者来说至关重要。Rust 的标准库设计哲学与 C++ STL 有些不同：**Rust 倾向于提供核心、高效的底层数据结构，然后通过其 API 来实现各种抽象数据类型的功能，而不是像 C++ 那样提供大量的容器适配器。**

下面我将列出 C++ STL 中常见的数据结构，并说明它们在 Rust 中的对应实现和惯用法。

## 标准库 (`std::collections`) 中的核心数据结构

| 数据结构 (Data Structure) | C++ STL | Rust 等价实现及惯用法 |
| :--- | :--- | :--- |
| **栈 (Stack)** | `std::stack` | **`Vec<T>`** <br> Rust 的 `Vec` 提供了 `push()` 和 `pop()` 方法，完美实现了栈的 LIFO（后进先出）行为。由于其连续内存布局带来的高缓存效率，`Vec` 是在 Rust 中实现栈的**最高效、最常用**的方式。 |
| **队列 (Queue)** | `std::queue` | **`std::collections::VecDeque<T>`** <br> `VecDeque` 是一个基于环形缓冲区（Ring Buffer）实现的双端队列。使用 `push_back()` 入队，`pop_front()` 出队，这两个操作都是 O(1) 的。这是实现 FIFO（先进先出）队列的标准方式。 |
| **双端队列 (Deque)** | `std::deque` | **`std::collections::VecDeque<T>`** <br> 这是 `std::deque` 的直接对应。它在头部和尾部进行增删操作都非常高效。 |
| **动态数组 (Vector)** | `std::vector` | **`Vec<T>`** <br> 这是 `std::vector` 的直接对应，也是 Rust 中最常用的集合类型。它保证数据在内存中是连续存储的。 |
| **优先队列 (Priority Queue)** | `std::priority_queue` | **`std::collections::BinaryHeap<T>`** <br> `BinaryHeap` 是一个基于二叉堆实现的最大堆（Max-Heap）。`push()` 添加元素，`pop()` 移除并返回最大元素。如果你需要一个最小堆（Min-Heap），可以存储一个包装类型，并为其实现 `Ord` 和 `PartialOrd` trait 来反转排序逻辑。 |
| **有序 Map (Ordered Map)** | `std::map` | **`std::collections::BTreeMap<K, V>`** <br> `BTreeMap` 基于 B-Tree 实现。它的键是有序的。与 C++ 的 `std::map`（通常是红黑树）相比，B-Tree 在现代 CPU 架构下有更好的缓存性能，因此 `BTreeMap` 在很多场景下更快。 |
| **有序 Set (Ordered Set)** | `std::set` | **`std::collections::BTreeSet<T>`** <br> `BTreeSet` 是 `BTreeMap` 的 "键" 版本，用于存储有序且唯一的元素。 |
| **哈希 Map (Hash Map)** | `std::unordered_map` | **`std::collections::HashMap<K, V>`** <br> 这是 `unordered_map` 的直接对应。Rust 的 `HashMap` 在安全性和防范哈希冲突攻击方面做了很多设计，默认使用一种高质量的、非加密的哈希算法（如 aHash）。 |
| **哈希 Set (Hash Set)** | `std::unordered_set` | **`std::collections::HashSet<T>`** <br> `HashSet` 是 `HashMap` 的 "键" 版本，用于存储无序但唯一的元素。 |
| **链表 (Linked List)** | `std::list` | **`std::collections::LinkedList<T>`** <br> Rust 提供了双向链表。但**在 Rust 中，`LinkedList` 很少被使用**。因为 `Vec` 和 `VecDeque` 由于其优秀的缓存局部性，在绝大多数情况下都比链表快得多。除非你有非常特殊的需求（比如需要在列表的中间频繁地、非破坏性地分割和合并），否则应优先选择 `Vec` 或 `VecDeque`。 |

---

## 标准库之外：通过 `crates.io` 获取

Rust 的生态系统非常强大，很多在 C++ STL 中没有，或者需要自己实现的数据结构，在 Rust 中都有高质量的第三方库（称为 "crates"）提供。你可以通过 `cargo add <crate_name>` 来轻松地将它们添加到你的项目中。

**并查集 (Union-Find / Disjoint Set Union)**

*   **C++ STL**: 没有内置实现，需要自己手写。
*   **Rust**: 使用 **`petgraph`** 或 **`union-find`** crate。
    *   **`petgraph`**: 这是一个功能非常全面的图论库，它内部提供了并查集的功能 (`petgraph::unionfind::UnionFind`)。如果你的项目大量涉及图算法，这是一个很好的选择。
    *   **`union-find`**: 这是一个专门为并查集设计的轻量级库，如果你只需要并查集功能，它非常方便。

**如何使用一个 crate (以 `petgraph` 为例):**

1.  **添加到项目**: 在你的项目目录下打开终端，运行：
    ```sh
    cargo add petgraph
    ```

2.  **在代码中使用**:
    ```rust
    use petgraph::unionfind::UnionFind;

    fn main() {
        let mut uf = UnionFind::new(10); // 创建一个包含10个元素的并查集
        uf.union(0, 1); // 合并 0 和 1 所在的集合
        uf.union(2, 3);
        uf.union(0, 3); // 现在 0, 1, 2, 3 都在一个集合里

        println!("Are 1 and 2 in the same set? {}", uf.equiv(1, 2)); // 输出: true
        println!("Are 1 and 5 in the same set? {}", uf.equiv(1, 5)); // 输出: false
    }
    ```

### 总结

*   **核心思想**: Rust 提供了性能优异的基础集合（`Vec`, `VecDeque`, `HashMap`, `BTreeMap`），你可以灵活地使用它们的 API 来满足栈、队列等需求。
*   **性能**: Rust 标准库的数据结构在设计时就极度关注性能，特别是缓存效率（例如 `Vec` vs `LinkedList`，`BTreeMap` vs Red-Black Tree）。
*   **生态系统**: 对于更专业的数据结构（如图、并查集、Trie树等），Rust 强大的 `crates.io` 生态系统提供了大量高质量、经过测试的现成实现。这通常比在 C++ 中自己实现或寻找第三方库要方便和安全得多。
*   

# Ordering mechanism in Rust

好的，让我从最基础的概念开始层层解释：

## 1. **最底层：`Ordering` 枚举**

`Ordering` 是 Rust 标准库中的一个简单枚举，表示比较结果：

```rust
pub enum Ordering {
    Less,     // 小于
    Equal,    // 等于  
    Greater,  // 大于
}
```

**作用**：任何两个值的比较只能有这三种结果。

## 2. **第一层：`cmp` 方法**

`cmp` 是 `Ord` trait 的核心方法：

```rust
trait Ord {
    fn cmp(&self, other: &Self) -> Ordering;
}
```

**作用**：比较两个值，返回 `Ordering` 枚举中的一个变体。

### 基本用法示例：
```rust
let a = 5;
let b = 10;

// 整数的 cmp 方法
match a.cmp(&b) {
    Ordering::Less => println!("a < b"),
    Ordering::Equal => println!("a == b"), 
    Ordering::Greater => println!("a > b"),
}
// 输出: a < b
```

## 3. **第二层：`PartialOrd` trait**

`PartialOrd` 是 `Ord` 的"弱化版"，用于可能无法比较的情况：

```rust
trait PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
}
```

**关键区别**：
- `Ord::cmp()` → 一定返回 `Ordering`
- `PartialOrd::partial_cmp()` → 返回 `Option<Ordering>`（可能无法比较）

## 4. **第三层：运算符重载**

这些 trait 让比较运算符工作：

```rust
// 这些运算符依赖于对应的 trait：
a == b  // 需要 PartialEq
a != b  // 需要 PartialEq  
a < b   // 需要 PartialOrd
a <= b  // 需要 PartialOrd
a > b   // 需要 PartialOrd  
a >= b  // 需要 PartialOrd
```

## 5. **第四层：在你的代码中的具体应用**

现在看你的具体实现：

```rust
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)  // 注意顺序！
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))  // 委托给 cmp 方法
    }
}
```

### **执行流程示例**：
```rust
let state1 = State { weight: 10, from: 1, to: 2 };
let state2 = State { weight: 5, from: 1, to: 3 };

// 当调用 state1 < state2 时：
// 1. 调用 state1.partial_cmp(&state2)
// 2. partial_cmp 调用 self.cmp(other)  
// 3. cmp 执行: other.weight.cmp(&self.weight)
//    = 5.cmp(&10) = Ordering::Less
// 4. 结果: state1 < state2 = true
```

### **关键理解点**：
- `other.weight.cmp(&self.weight)` 中的顺序是**颠倒的**
- 这实际上创建了**按权重升序**的比较（小权重"大于"大权重）
- 这种模式常用于创建**最小堆**

## 6. **完整的关系链**：

```
运算符 (<, >, ==) 
    ↓ 调用
PartialOrd::partial_cmp / PartialEq::eq  
    ↓ 委托给  
Ord::cmp / Eq
    ↓ 返回  
Ordering (Less/Equal/Greater)
```

这样设计的好处是：你只需要实现核心的 `cmp` 逻辑，所有比较运算符和容器排序都能自动工作。