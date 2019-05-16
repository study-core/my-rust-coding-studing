// 定义一个 counter 结构体
struct Counter {
    count: u32,
}

// 定义 counter 结构体的 关联函数
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

/*
关于 Iterator trait 的定义
trait Iterator {
    // 关联类型 Item
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 此处省略了方法的默认实现
}
*/

// counter 实现 Iterator trait
impl Iterator for Counter {
    // 将 关联类型 Item 设置为 u32
    // 意味着迭代器会返回 u32 值集合
    type Item = u32;

    // 实现 next 方法
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}


#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    // 测试 自定义迭代器只能遍历5次
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
// 获取 Counter 实例产生的值，将这些值与另一个 Counter 实例在省略了第一个值之后产生的值配对，
// 将每一对值相乘，只保留那些可以被三整除的结果，然后将所有保留的结果相加
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        // zip 在任一输入迭代器返回 None 时也返回 None
        // 所以这里的zip 只产生四对值；理论上第五对值 (5, None) 从未被产生。
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}