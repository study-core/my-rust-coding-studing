//use std::io;

/*
fn main() {
//    println!("Guess the number!");
//
//    println!("Please input your guess.");
//
//    let mut guess = String::new();
//
//    io::stdin().read_line(&mut guess)
//        .expect("Failed to read line");
//
//    println!("You guessed: {}", guess);


//    let s = "ssss";
//    let s1 = String::from("hello");

    let mut s = String::from("hello world");

    let word = first_word(&s); // word 的值为 5

    s.clear(); // 这清空了字符串，使其等于 ""


}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}*/



/*
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);


    *//*
    结构体
    *//*
    struct  User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }


    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };


    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };


    *//*
    元组结构体 [注意： 不是元组哦]
    *//*
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

*/




/*
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}


*/

/*
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);

//    println!("rect1 is  {:#?}", rect1);
}*/



/*

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );



    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}



enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
*/


/*fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);


    // v.get(i)  返回一个 Option<&T>
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}*/



/*
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

*/


/*
use std::collections::HashMap;

fn main() {
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:#?}", scores)
}*/




/*
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {

        // or_insert 方法事实上会返回这个键的值的一个可变引用（&mut V
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
*/


/*use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}*/


/*
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
}
*/



/*use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}*/

/*
use std::io;
use std::io::Read;
use std::fs::File;


fn main() {
    let f =  read_username_from_file();

    match f {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("{:?}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::from("this is success ！!");

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}*/


/*
use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    let f =  read_username_from_file();

    match f {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("{:?}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {

    // 如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。如果值是 Err，Err 中的值将作为整个函数的返回值，就好像使用了 return 关键字一样，这样错误值就被传播给了调用者

    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}*/



/*
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}*/










/*
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

// 生命周期泛型
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}*/





/*
use std::fmt::Display;

fn main() {

}

// 同时存在, 指定泛型类型参数、trait bounds 和生命周期的语法
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}*/


/*
*//*
声明 带有 闭包泛型 和 option 泛型的结构体
*//*
use std::thread;
use std::time::Duration;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}


impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {

        // 每次调用该方法时，都先判断当前Cacher 实例的 value字段的值
        match self.value {
            Some(v) => v,
            None => {

                // 调用 闭包，并将闭包返回值 赋值给 v
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}


fn generate_workout(intensity: u32, random_number: u32) {

    // 根据一个 闭包入参，实例化 Cacher 实例
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}*/



/*
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}*/


/*
fn main() {
    let x = vec![1, 2, 3];


    // 使用了 move 关键字，这时候 x 的所有权，由 main转移到 闭包中，则下一行 println! 就会报错，以为x的所有权已经不在main中了
    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}*/


/*fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];


    // 将原来集合中各个元素都 +1
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    println!("原来的集合 {:?}, \n后来的集合 {:?}", v1, v2)
}*/



/*#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // 过滤收集 只有满足 s.size == shoe_size 条件的 元素
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}
fn main() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];
    let v = shoes_in_my_size(shoes, 10);
//    assert_eq!(
//        in_my_size,
//        vec![
//            Shoe { size: 10, style: String::from("sneaker") },
//            Shoe { size: 10, style: String::from("boot") },
//        ]
//    );
    println!("返回的集合为: {:#?}", v)
}*/



/*
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}*/


/*

// 循环引用，内存泄漏
use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}


fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    // 查看 a的引用 这里是 1个
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    // 把 a 指向 b
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // 再次查看a的引用 这里是 2
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // 而查看b的引用这里是 1
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // 单个的 模式匹配
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // 取消如下行的注释来观察引用循环；
    // 这会导致栈溢出
     println!("a next item = {:?}", a.tail());

    //!
    //! 可以看到将 a 修改为指向 b 之后，a 和 b 中都有的 Rc<List> 实例的引用计数为 2。
    //! 在 main 的结尾，Rust 会尝试首先丢弃 b，这会使 a 和 b 中 Rc 实例的引用计数减 1
    //!
    //! 然而，因为 a 仍然引用 b 中的 Rc<List>，Rc<List> 的引用计数是 1 而不是 0，
    //! 所以 Rc<List> 在堆上的内存不会被丢弃。其内存会因为引用计数为 1 而永远停留.
    //!
    //! 如图:
    //!
    //!
    //!
}
*/



/*
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..=5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
*/


/*
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // 通过在闭包之前增加 move 关键字，我们强制闭包获取其使用的值的所有权，而不是任由 Rust 推断它应该借用值
    // 保证在闭包内还能用到 v 的所有权
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });


    handle.join().unwrap();

    // 所以在闭包外面已经拿不到 v 的所有权了
    // move 关键字，通过告诉 Rust 将 v 的所有权移动到新建线程，我们向 Rust 保证主线程不会再使用 v
    println!("{:?}", v)
}*/


/*
use std::thread;
use std::sync::mpsc;

fn main() {

    // 创建一个 通道，发送端名： tx， 接受端名： rx
    let (tx, rx) = mpsc::channel();

    // 通过move 将tx 所有权移到 线程的 闭包中
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}*/

/*

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // 一个字符串集合
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
*/

/*
//! 多个生产者
//!
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {

    let (tx, rx) = mpsc::channel();


    // 这里可以通过 通道 克隆来做到获取多个同一个通道的 发送端
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
*/


/*
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
       *//*
       // 这里是获取 m.lock 的所有权，而不是 m的所有权，所以在 {} 外面还是可以 println! m的
       let mut num = m.lock().unwrap();
        *num = 6;
        *//*

        // 先转移所有权
        let m2 = m;
        let mut num = m2.lock().unwrap();
        *num = 6;
        println!("m2 = {:?}", m2);
//        println!("m = {:?}", m);

    }

//    println!("m = {:?}", m);
}
*/


/*
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        // 注意，这里的 num 表示返回 lock 的引用计数
        // TODO 注意，这里的 counter 实例被 move 到闭包里面了
        // 所以外面已经失去它的 所有权
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
*/


/*
//! 使用了原子行的 Rc<T>
//! Arc<T> 的API 和Rc<T> 一致
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {

    // 使用智能指针，Arc<T> 具备原子性的Rc<T> 创建 Mutex<T>
    let counter = Arc::new(Mutex::new(0));


    let mut handles = vec![];

    for _ in 0..10 {

        // 在进入 线程闭包前 通过Arc来clone一个mutex的引用
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {

            // 返回一个引用计数句柄  LockResult<MutexGuard<T>>
            let mut num = counter.lock().unwrap();

            // 给 句柄加 1 (使用解引用符号 * 可以获取智能指针的原始值)
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 这里输出 counter的引用句柄数 counter.lock 返回一个智能指针
    println!("Result: {}", *counter.lock().unwrap());
}*/


/*
//! 面向对象编程 #################
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}


impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        // 从末尾弹掉一个元素
        let result = self.list.pop();
        match result {
            // 返回 Some(value)
            Some(value) => {
                self.update_average();
                Some(value)
            },

            // 返回 None
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        // 累加所有元素的值
        let total: i32 = self.list.iter().sum();
        // 求平均值
        self.average = total as f64 / self.list.len() as f64;
    }
}


fn main() {

    let mut a  = AveragedCollection{
        list: vec![],
        // average: 0.0,
        average: 0 as f64,
    };

    a.add(45);
    a.add(78);

    let av =  a.average();
    println!("{}", av);
    a.remove();
    a.remove();
    let p = a.remove();

    match p {
        Some(v) => println!("移除掉了：{}", v),

        None => println!("nil~"),
    }

}*/






/*
pub trait Draw {
    fn draw(&self);
}


pub struct Screen {
    // TODO 注意: dyn Draw 这个啥语法啊
    // 类似于 类型泛型 !
    pub components: Vec<Box<dyn Draw>>,
}


impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}



*//*
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}*//*


pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 实际绘制按钮的代码
    }
}*/

/*

fn main() {

//    let x;
//    let y = 1_i32;
//    x = 2_i32;
//
//    println!("{:?}, {:?}", x, y) // 2, 1

    // 全局的定义必须加上 类型
    static mut X : i32 = 21; // 全局变量 必须在定义时 加上 mut


    // 在使用 全局变量时，必须用 unsafe 关键字
    unsafe {
        X = 65;
        println!("{}", X)
    }

0
}
*/

/*
// pub use的使用

use crate::outer::GetName;

mod outer {

    pub use self::inner::GetName; // 这里是将自己的内部元素导出给外面的人用

    mod inner {

        pub fn GetName () -> String {
            let s = String::from("我是inner的函数，但是我要被导出到outer外面去");
            s
        }
    }
}

// 在这里使用 GetName（）
fn call_getName () {
    println!("{}", GetName())
}

fn main() {
    call_getName()
}*/

/*

*//*
   宏转换（将具有不完整参数的调用转换为规范形式）

   宏中的匹配规则

   宏中使用字面量匹配的哦

   几种指示符：

   ident: 标识符，用来表示函数或变量名; 例如：x; foo.
   expr: 表达式;例如：2 + 2; if true then { 1 } else { 2 }; f(42).
   block: 代码块，用花括号包起来的多个语句;例如： { log(error, "hi"); return 12; }.
   pat: 模式，普通模式匹配（非宏本身的模式）中的模式，例如 Some(t), (3, 'a', _), _.
   path: 路径，注意这里不是操作系统中的文件路径，而是用双冒号分隔的限定名(qualified name)，如 std::cmp::PartialOrd
   tt: 单个语法树 (标记树 token tree)
   ty: 类型，语义层面的类型，如 i32, char
   item: 一个项目。例如： fn foo() { }; struct Bar;.
   meta: 一个 "元项目", 在属性中建立的. 例如： cfg(target_os = "windows").
   stmt: 单条语句，如 let a = 42;


还有其他关于元变量后下一个标记的附加规则：

变量 expr 后必须加下面中的一个： => , ;
变量 ty 和 path 后必须加下面中的一个： => , : = > as
变量 pat 后必须加下面中的一个：=> , =
其它变量后可能要加其它符号。

    TODO 宏 和 函数的 区别
    函数不能接受任意多个参数，其次函数是不能操作语法单元的，
    即把语法元素作为参数进行操作，从而生成代码，
    例如 mod, crate 这些是 Rust 内置的关键词，
    是不可能直接用函数去操作这些的，而宏就有这个能力。

    相比函数，宏是用来生成代码的，在调用宏的地方，
    编译器会先将宏进行展开，生成代码，然后再编译展开后的代码。

    定义：

    TODO： macro_rules! macro_name { macro_body }
    即：
    macro_rules! macro_name {
        pattern => do_something
    }
    其中 pattern 和 do_something 都是用配对的括号括起来的，
    括号可以是圆括号、方括号、花括号中的任意一种。
    匹配可以有多个分支，每个分支以分号结束。

    注意： 需要说明的是这里的括号和宏里面其它地方一样都可以是三种括号中的任意一种，
    因为括号在这里仅仅是用来标记一个模式的开始和结束，大部分情况重复的模式是用逗号或分号分隔的，
    所以你会经常看到 $(...),*, $(...);*, $(...),+, $(...);+ 这样的用来表示重复。
   *//*


// 宏模式所匹配的是 Rust 代码结构而不是值
fn main() {
    // 使用自定义宏
//    gavin![1, 2, 5];
    gavin![];

    foo!{y => 3};

    // foo!(z => 3); // 错误的

    let a: &[i32] = o_O!(10; [1, 2, 3]; 20; [4, 5, 6]);

    assert_eq!(a, [11, 12, 13, 24, 25, 26]);


    let state: &str = "reticulating splines";
    log!(state);


    BOO!(x);
    println!("{}", x);

    func!();
    my();



    // 递归宏
    use std::fmt::Write;
    let mut out = String::new();

    write_html!(&mut out,
        html[
        head[title["Macros guide"]]
        body[h1["Macros are the best!"]]
        ]);

    assert_eq!(out,
               "<html><head><title>Macros guide</title></head>\
 <body><h1>Macros are the best!</h1></body></html>");



    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2 , 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));



    print_result!(1u32 + 1);

    // 回想一下，代码块也是表达式！
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });

    myFunc();
    bar();


    // 测试 `add_assign`、`mul_assign` 和 `sub_assign`
    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);


    calculate! {
        eval 1 + 2 // 看到了吧，`eval` 可并不是 Rust 的关键字！
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }


    calculate2! { // 妈妈快看，可变参数的 `calculate!`！
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }


    let v = vec!{1, 2};
    println!("{}", v[0])

}

*//*
定义一个宏 (导出的宏，可以被任意地方引用)
一定要有：
#[macro_export] 注解哦
*//*
#[macro_export]
macro_rules! gavin_vec {

    // 【1】表示入参的表达式有0到多个
    ( $( $x:expr ),* ) => {

        // 先调用 函数生成一个 Vec的变量
        let mut temp_vec = Vec::new();
        // 表示 该语句有0到多个，具体个数 和【1】一致
        $(
            temp_vec.push($x);
        )*
        // 返回 temp_vec 变量
        temp_vec
    };
}

// 一个打印宏
#[macro_export]
macro_rules! gavin {
    () => {
      println!("I am gavin macro_rules!!")
    };
}

// 一个表达式字面量匹配宏
#[macro_export]
macro_rules! foo {
    (x => $e:expr) => (println!("mode X: {}", $e));
    (y => $e:expr) => (println!("mode Y: {}", $e));
}

// 一个逐级相加的宏
#[macro_export]
macro_rules! o_O {
    (
        $(
            $x:expr; [ $( $y:expr ),* ]
        );*
    ) => {
            &[ $($( $x + $y ),*),* ]
    }
}

// 定义一个 日志宏
#[macro_export]
macro_rules! log {
    ($msg:expr) => {{
        // 在这里调用了 get_log_state()
        let state: i32 = get_log_state();
        if state > 0 {
            // 将结果打印出来
            println!("log({}): {}", state, $msg);
        }
    }};
}

fn get_log_state () -> i32 {
    let s  = 4;
    s
}

// 内部赋值 宏
#[macro_export]
macro_rules! BOO {
    // 内部赋值 宏
    ($v:ident) => (let $v = 3);
}

// 定义函数 宏
#[macro_export]
macro_rules! func {
    () => (fn my() { println!("我是一个func~")});
}

// 递归宏
#[macro_export]
macro_rules! write_html {
    ($w:expr, ) => (());

    ($w:expr, $e:tt) => (write!($w, "{}", $e));

    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{

        write!($w, "<{}>", stringify!($tag));
        write_html!($w, $($inner)*);
        write!($w, "</{}>", stringify!($tag));
        write_html!($w, $($rest)*);
    }};
}

*//*
递归查找最小值 宏
*//*
#[macro_export]
macro_rules! find_min {
    // 如果只有一个元素，则直接返回
    ($x:expr) => ($x);

    // 如果有 多个元素则
    ($x:expr, $($y:expr),+) => (
    // 用第一个和后面的递归算出来最小的来算出最小的，并返回
    std::cmp::min($x, find_min!($($y),+))
    )
}


macro_rules! create_function {
    // 此宏接受一个 `ident` 指示符表示的参数，并创建一个名为 `$func_name` 的函数。
    // `ident` 指示符用于变量名或函数名
    ($func_name:ident) => (
        fn $func_name() {
            // `stringify!` 宏把 `ident` 转换成字符串。
            println!("You called {:?}()",
                     stringify!($func_name))
        }
    )
}

// 借助上述宏来创建名为 `foo` 和 `bar` 的函数。
create_function!(myFunc);
create_function!(bar);


#[macro_export]
macro_rules! print_result {
    // 此宏接受一个 `expr` 类型的表达式，并将它作为字符串，连同其结果一起
    // 打印出来。
    // `expr` 指示符表示表达式。
    ($expression:expr) => (
        // `stringify!` 把表达式*原样*转换成一个字符串。
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression)
    )
}










//#[macro_export]
macro_rules! assert_equal_len {
    // `tt`（token tree，标记树）指示符表示运算符和标记。
    ($a:ident, $b: ident, $func:ident, $op:tt) => (
        assert!($a.len() == $b.len(),
                "{:?}: dimension mismatch: {:?} {:?} {:?}",
                stringify!($func),
                ($a.len(),),
                stringify!($op),
                ($b.len(),));
    )
}

use std::ops::{Add, Mul, Sub};
macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => (
        fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
                // *x = x.$method(*y);
            }
        }
    )
}

// 实现 `add_assign`、`mul_assign` 和 `sub_assign` 等函数。
op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

mod test {
    use std::iter;

    // 定义宏
    #[macro_export]
    macro_rules! test {
        ($func: ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    super::$func(&mut x, &y);

                    assert_eq!(x, z);
                }
            }
        }
    }

    // 测试 `add_assign`、`mul_assign` 和 `sub_assign`
    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);
}


// 小型计算机 宏
#[macro_export]
macro_rules! calculate {
    (eval $e:expr) => {{
        {
            let val: usize = $e; // 强制类型为整型
            println!("{} = {}", stringify!{$e}, val);
        }
    }};
}


#[macro_export]
macro_rules! calculate2 {
    // 单个 `eval` 的模式
    (eval $e:expr) => {{
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    }};

    // 递归地拆解多重的 `eval`
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate2! { eval $e }
        calculate2! { $(eval $es),+ }
    }};
}


pub fn increment(x: u32) -> u32 {
    x + 1
}

#[macro_export]
macro_rules! inc_a {
    // 这里表明，该宏只能在当前 crate (库内)使用
    ($x:expr) => ( ::increment($x) )
}

#[macro_export]
macro_rules! inc_b {
    // 这里表明，该宏只能在外部 crate (库外)使用
    ($x:expr) => ( ::mylib::increment($x) )
}


#[macro_export]
macro_rules! inc {
    // TODO 这个表示  库内外 都可任意使用的 宏
    // 这个函数名字会展开为::increment或::mylib::increment。
    // 为了保证这个系统简单和正确，#[macro_use] extern crate ...【应只出现在你包装箱的根中】，而不是在mod中
    ($x:expr) => ( $crate::increment($x) )
}*/



/*
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // 函数体
            println!("{:#?}", 12)
        }
    }
}

mod performance_group {

    *//*
    pub use 的使用
    在这里通过 pub use 将 sound::instrument 引入 performance_group
    并将 instrument 作为 performance_group 的部分引出去，给别人能用
    *//*
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

fn main() {
    performance_group::clarinet_trio();

    // 因为在 performance_group 使用了 pub use 所以这里可以把原先定义在 sound 中的 instrument
    // 当做 performance_group 中的 instrument 重新引出
    performance_group::instrument::clarinet();
}*/


//fn main() {
//    /*let v = vec![1, 2, 3, 4, 5];
//
//    let third: &i32 = &v[2];
//    println!("The third element is {}", third);
//
//    let val = v.get(2);
//
//    match val {
//        Some(third) => println!("The third element is {}", third),
//        None => println!("There is no third element."),
//    }
//
//    println!("{}", third);
//    println!("{}", third);
//    println!("{}", third)*/
//
//
//
//   /*
//    // 下面这个是错的～～ v.push(6)
//    let mut v = vec![1, 2, 3, 4, 5];
//
//    let first = &v[0];
//
//    v.push(6);
//
//    println!("The first element is: {}", first);*/
//
//
//
//    let s1 = "xxx";
//
//    let s2 = "aaa";
//
//    let s3 = s1.to_owned() + s2;
//
//    println!("s1:{}, s2:{}, s3:{}", s1, s2, s3);
//
//    // String 是一个 Vec<u8> 的封装
//    let len = String::from("Hola").len(); // 4 因为 Hola 为 这里每一个字母的 UTF-8 编码都占用一个字节
//    println!("{}", len);
//
//    let len = String::from("Здравствуйте").len(); // 24 这是使用 UTF-8 编码 “Здравствуйте” 所需要的字节数，这是因为每个 Unicode 标量值需要两个字节存储
//    println!("{}", len);
//
//
//    // 使用 索引操作 字符串 是一个坏点子
//    let hello = "Здравствуйте";
//
////    let s = &hello[0..3]; // 出错
//
//    let s = &hello[0..4]; // 输出 Зд
//    println!("{}", s);
//
//
//    for c in "नमस्ते".chars() { // 注意这里返回 6个 char ： न म ् स् ् ते
//        println!("{}", c);
//    }
//
//
//    for b in "नमस्ते".bytes() { // 返回原始字节
//        println!("{}", b);
//    }
//}


//fn main() {
//
//    use std::collections::HashMap;
//
//    /*let mut scores = HashMap::new();
//
//    scores.insert(String::from("Blue"), 10);
//    scores.insert(String::from("Yellow"), 50);
//
//    let team_name = String::from("Blue");
//    let score = scores.get(&team_name);
//    let myscore = match score {
//        Some(val) => println!("{}", val),
//        None =>  println!("It is  empty～～")
//    }*/
//
//
//
//    /*let mut scores = HashMap::new();
//
//    scores.insert(String::from("Blue"), 10);
//    scores.insert(String::from("Yellow"), 50);
//
////    for (key, value) in &scores {
//    for m in &scores {
//        println!("{:?}", m); // ("Yellow", 50)  ("Blue", 10)
//    }*/
//
//
//   /* // 没有就插入
//    let mut scores = HashMap::new();
//    scores.insert(String::from("Blue"), 10);
//
//    er = scores.entry(String::from("Yellow")).or_insert(50);
//    scores.entry(String::from("Blue")).or_insert(50); // 检查某个特定的键是否有值，如果没有就插入一个值
//
//    println!("{:?}", scores);
//
//    match er {
//
//    }*/
//
//
//    /*// 更新旧值
//    let text = "hello world wonderful world";
//
//    let mut map = HashMap::new();
//
//    for word in text.split_whitespace() {
//        let count = map.entry(word).or_insert(0);
//        *count += 1;
//    }
//
//    println!("{:?}", map); // {"hello": 1, "wonderful": 1, "world": 2}*/
//}


//use std::panic;
//fn main() {
//
//
//   /* // 读文件
//    use std::io;
//    use std::io::Read;
//    use std::fs::File;
//
//    fn read_username_from_file() -> Result<String, io::Error> {
//        let f = File::open("hello.txt");
//
//        // 注意了 match 是有返回值的
//        let mut f = match f {
//            Ok(file) => file,
//            Err(e) => return Err(e),
//        };
//
//        let mut s = String::new();
//
//        match f.read_to_string(&mut s) {
//            Ok(_) => Ok(s),
//            Err(e) =>{
//                println!("err:{}", e.to_string());
//                Err(e)
//            },
//        }
//    }*/
//
//   /* // 用 ? 号 代替 Result  和上面的 match 是一样的
//    use std::io;
//    use std::io::Read;
//    use std::fs::File;
//
//    fn read_username_from_file() -> Result<String, io::Error> {
//        // 这里的 ? 是一个泛指， 如果是 OK 则返回 ok的值给 f变量，程序就往下走, 等价于 TODO:  let mut f = file
//        // 如果是 Err 则直接结束 read_username_from_file() 函数的调用, 等价于 TODO:  return Err(e)
//        // 并返回Err 给函数的调用者
//        let mut f = File::open("hello.txt")?;
//        let mut s = String::new();
//        f.read_to_string(&mut s)?;
//        Ok(s)
//    }*/
//
//
//   /* // 链式操作
//    use std::io;
//    use std::io::Read;
//    use std::fs::File;
//
//    fn read_username_from_file() -> Result<String, io::Error> {
//        let mut s = String::new();
//
//        // 链式操作
//        File::open("hello.txt")?.read_to_string(&mut s)?;
//        println!("{}", s);
//        Ok(s)
//    }*/
//
//
//
//    /*fn catch() {
//        let nums: [i32; 5] = [1, 2, 3, 4, 5];
//        let numsPtr = &nums;
//        panic::catch_unwind(|| {
//            println!("{}", numsPtr[9]);
//        });
//    }
//
//    catch()*/
//}



//mod traittest;
//use traittest::Summary; // 注意不能直接写 use 某个mod哦， use traittest;
//
//
//pub struct NewsArticle {
//    pub headline: String,
//    pub location: String,
//    pub author: String,
//    pub content: String,
//}
//
//impl Summary for NewsArticle {
//    fn summarize(&self) -> String {
//        format!("{}, by {} ({})", self.headline, self.author, self.location)
//    }
//}
//
//pub struct Tweet {
//    pub username: String,
//    pub content: String,
//    pub reply: bool,
//    pub retweet: bool,
//}
//
//impl Summary for Tweet {
//    fn summarize(&self) -> String {
//        format!("{}: {}", self.username, self.content)
//    }
//
//
//}
//
//
//fn main() {
//    let tweet = Tweet {
//        username: String::from("Gavin"),
//        content: String::from("of course, as you probably already know, people"),
//        reply: false,
//        retweet: false,
//    };
//
//    println!("1 new tweet: {}", tweet.summarize());
//
////    notify(tweet);
//    notify2(tweet);
//
//
//    let p = Pair::new(48_i8, 47_i8);
//
//
//}
//
//// trait 入参
//pub fn notify(item: impl Summary) {
//    println!("Breaking news! {}", item.summarize());
//}
//
//// 或者 使用 trait bonds 来表示: trait 入参
//pub fn notify2<T: Summary>(item: T) {
//    println!("notify2 news! {}", item.summarize());
//}
///*
//有些签名中使用 trait bonds 太多了，这时候需要用到 where
//
//fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
//
//等价于
//
//fn some_function<T, U>(t: T, u: U) -> i32
//    where T: Display + Clone,
//          U: Clone + Debug
//{}
//
//TODO 贼返回值中使用trait 类型， 只能返回单一的 trait实现类型
//
//TODO 下面这个就有问题
//
//fn returns_summarizable(switch: bool) -> impl Summary {
//    if switch {
//        NewsArticle {
//            headline: String::from("Penguins win the Stanley Cup Championship!"),
//            location: String::from("Pittsburgh, PA, USA"),
//            author: String::from("Iceburgh"),
//            content: String::from("The Pittsburgh Penguins once again are the best
//            hockey team in the NHL."),
//        }
//    } else {
//        Tweet {
//            username: String::from("horse_ebooks"),
//            content: String::from("of course, as you probably already know, people"),
//            reply: false,
//            retweet: false,
//        }
//    }
//}
//
//TODO 而这样是 OK 的
//
//fn returns_summarizable() -> impl Summary {
//    Tweet {
//        username: String::from("horse_ebooks"),
//        content: String::from("of course, as you probably already know, people"),
//        reply: false,
//        retweet: false,
//    }
//}
//
//*/



/*
// TODO 使用 trait bound 有条件地实现方法
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// TODO  new 【函数】的实现
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

// TODO cmp_display 【方法】的实现
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}*/


// 生命周期
//fn main() {
//
////    /*
////    变量 x 并没有 “存在的足够久”.
////    其原因是 x 在到达第 7 行内部作用域结束时就离开了作用域.
////    不过 r 在外部作用域仍是有效的;
////    作用域越大我们就说它 “存在的越久”.
////    如果 Rust 允许这段代码工作,r 将会引用在 x 离开作用域时被释放的内存,
////    这时尝试对 r 做任何操作都不能正常工作.
////
////    */
////    {
////        let r;                              // ---------+-- 'a
////                                                 //          |
////        {                                        //          |
////            let x = 5;                       // -+-- 'b  |
////            r = &x;  // 借来的 x 价值不够长         //  |       |
////        }                                        // -+       |
////                                                 //          |
////        println!("r: {}", r);                    //          |
////    }                                            // ---------+
////
////
//
//
//    fn longest(x: &str, y: &str) -> &str {
//        if x.len() > y.len() {
//            x
//        } else {
//            y
//        }
//    }
//    /*
//    因为 Rust 并不知道将要返回的引用是指向 x 或 y.事实上我们也不知道,
//    因为函数体中 if 块返回一个 x 的引用而 else 块返回一个 y 的引用！
//    编译器 分不清 x 和 y 的生命周期是否一致.
//    TODO 因为它不知道 x 和 y 的生命周期是如何与返回值的生命周期相关联的.
//
//    TODO 解决方案：
//    增加泛型生命周期参数来定义引用间的关系以便借用检查器可以进行分析.
//
//    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//        if x.len() > y.len() {
//            x
//        } else {
//            y
//        }
//    }
//
//    */
//    let s = longest("s", "4");
//    println!("{}", s)
//
//    /*
//    &i32            // 引用
//    &'a i32         // 带有显式生命周期的引用
//    &'a mut i32     // 带有显式生命周期的可变引用
//    */
//
//
//    /*
//    TODO 省略  生命周期参数  原则
//
//    todo 第一条规则适用于输入生命周期，后两条规则适用于输出生命周期。
//    如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误。
//
//    这些规则适用于 fn 定义，以及 impl 块。
//
//    todo 第一条规则是每一个是引用的参数都有它自己的生命周期参数。换句话说就是，有一个引用参数的函数有一个生命周期参数：fn foo<'a>(x: &'a i32)，有两个引用参数的函数有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推。
//
//    todo 第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。
//
//    todo 第三条规则是如果方法有多个输入生命周期参数，不过其中之一因为方法的缘故为 &self 或 &mut self，那么 self 的生命周期被赋给所有输出生命周期参数。这使得方法更容易读写，因为只需更少的符号。
//
//    */
//
//
//
//    // 静态生命周期 'static
//    let s: & 'static str = "I have a static lifetime.";
//
//
//}


//
//pub struct Guess {
//    value: i32,
//}
//
//impl Guess {
//    pub fn new(value: i32) -> Guess {
//        if value < 1 || value > 100 {
//            panic!("Guess value must be between 1 and 100, got {}.", value);
//        }
//
//        Guess {
//            value
//        }
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    #[should_panic]
//    fn greater_than_100() {
//        Guess::new(200);
//    }
//}


//use std::thread;
//use std::time::Duration;
//
//fn generate_workout(intensity: u32, random_number: u32) {
//
//    /*
//    todo 闭包
//
//    let expensive_closure = |num: u32| -> u32 {
//        println!("calculating slowly...");
//        thread::sleep(Duration::from_secs(2));
//        num
//    };
//
//    或者
//
//    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
//    let add_one_v2 = |x: u32| -> u32 { x + 1 };
//    let add_one_v3 = |x|             { x + 1 };
//    let add_one_v4 = |x|               x + 1  ;
//
//    TODO 闭包中不显示 类型的话，使用类型推断，则在使用了第一次后，该闭包的类型就确定了
//    */
//    // TODO 闭包不【不要求】像 fn 函数那样在参数和返回值上注明类型.
//    // TODO 函数中需要类型注解是因为他们是暴露给用户的显式接口的一部分.
//    // 闭包通常很短并只与对应相对任意的场景较小的上下文中
//    let expensive_closure = |num| {
//        println!("calculating slowly...");
//        thread::sleep(Duration::from_secs(2));
//        num
//    };
//
//    if intensity < 25 {
//        println!(
//            "Today, do {} pushups!",
//            expensive_closure(intensity)
//        );
//        println!(
//            "Next, do {} situps!",
//            expensive_closure(intensity)
//        );
//    } else {
//        if random_number == 3 {
//            println!("Take a break today! Remember to stay hydrated!");
//        } else {
//            println!(
//                "Today, run for {} minutes!",
//                expensive_closure(intensity)
//            );
//        }
//    }
//}
//
//
//fn generate_workout2(intensity: u32, random_number: u32) {
//    let mut expensive_result = Cacher::new(|num| {
//        println!("calculating slowly...");
//        thread::sleep(Duration::from_secs(2));
//        num
//    });
//
//    if intensity < 25 {
//        println!(
//            "Today, do {} pushups!",
//            expensive_result.value(intensity)
//        );
//        println!(
//            "Next, do {} situps!",
//            expensive_result.value(intensity)
//        );
//    } else {
//        if random_number == 3 {
//            println!("Take a break today! Remember to stay hydrated!");
//        } else {
//            println!(
//                "Today, run for {} minutes!",
//                expensive_result.value(intensity)
//            );
//        }
//    }
//}
//
//
//// Fn 系列 trait 由标准库提供。
//// 所有的闭包都实现了 trait Fn、FnMut 或 FnOnce 中的一个
//struct Cacher<T>
//    where T: Fn(u32) -> u32
//{
//    calculation: T,  // 这是一个闭包 泛型 T
//    value: Option<u32>,
//}
//
//impl<T> Cacher<T>
//    where T: Fn(u32) -> u32
//{
//    // 入参一个 闭包
//    fn new(calculation: T) -> Cacher<T> {
//        Cacher {
//            calculation,
//            value: None,
//        }
//    }
//
//    fn value(&mut self, arg: u32) -> u32 {
//        match self.value {
//            Some(v) => v,
//            None => {
//                let v = (self.calculation)(arg);
//                self.value = Some(v);
//                v
//            },
//        }
//    }
//}
//
//#[test]
//fn call_with_different_values() {
//    let mut c = Cacher::new(|a| a);
//
//    let v1 = c.value(1);
//    let v2 = c.value(2);
//
//    assert_eq!(v2, 2);
//}
//
////fn main() {
////    let x = 4;
////
////    let equal_to_x = |z| z == x;
////
////    let y = 4;
////
////    assert!(equal_to_x(y));
////}
//
//fn main() {
//    let x = vec![1, 2, 3];
//
//    let equal_to_x = move |z| z == x; // 将 x 所有权移入闭包了
//
//    println!("can't use x here: {:?}", x); // 这里将获取到一个 悬垂引用
//
//    let y = vec![1, 2, 3];
//
//    assert!(equal_to_x(y));
//}












/*
TODO 智能指针
*/


// TODO  Box
// box 允许你将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针
/*
Box 多用于下面的场景
当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候

TODO Box默认实现了 Deref trait 可以被解指针操作
*/

//enum List {
//    Cons(i32, List),
//    Nil,
//}


/*
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

//fn main() {
//
//    // 这样纸会出现 无限引用导致没存溢出
////    let list = Cons(1, Cons(2, Cons(3, Nil)));
//
//    // TODO 下面这个才是真正的做法
////    let list = Cons(1,
////                    Box::new(Cons(2,
////                                  Box::new(Cons(3,
////                                                Box::new(Nil))))));
//
//
////    let x = 5;
////    let y = &x;
////
////    assert_eq!(5, x);
////    assert_eq!(5, *y);
//
//
////
////    let x = 5;
////    let y = Box::new(x);
////
////    assert_eq!(5, x);
////    assert_eq!(5, *y);
//
//
//
////    let x = 5;
////    let y = MyBox::new(x);
////
////    assert_eq!(5, x);
////    assert_eq!(5, *y); // 对于实现了 Deref trait 的智能指针 (包含自定义的) 使用 todo *y == *(y.deref())
//
//
//}


*//*
*//**//*
TODO 定义自己的智能指针
【注意】 // 自定义这个没实现 Deref trait 的，不可以被解指针

*//**//*
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}


use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}*//*





use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
    }

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}*/


//struct CustomSmartPointer {
//    data: String,
//}
//
//impl Drop for CustomSmartPointer {
//    fn drop(&mut self) {
//        println!("Dropping CustomSmartPointer!");
//    }
//}
//
//fn main() {
//    let c = CustomSmartPointer { data: String::from("some data") };
//    println!("CustomSmartPointer created.");
//
//    // 提前把 c 清理掉 注意这里不用 c.drop() 哦
//    drop(c);
//    println!("CustomSmartPointer dropped before the end of main.");
//}


//enum List {
//    Cons(i32, Box<List>),
//    Nil,
//}
//
//use List::{Cons, Nil};
//fn main() {
//    let a = Cons(5,
//                 Box::new(Cons(10,
//                               Box::new(Nil))));
//    let b = Cons(3, Box::new(a));
//    let c = Cons(4, Box::new(a)); // 这个会报错，因为所有权已经被转移
//}


//enum List {
//    Cons(i32, Rc<List>),
//    Nil,
//}
//
//use List::{Cons, Nil};
//use std::rc::Rc;
//
//// 这个是 ok 的 Rc::clone 只是 clone 引用，而不是 clone 数据
//fn main() {
//    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//    let b = Cons(3, Rc::clone(&a));
//    let c = Cons(4, Rc::clone(&a));
//}


// 用 RefCell<T> 来修改不可变的 数据
/*
TODO 对于引用和 Box<T>，借用规则的不可变性作用于编译时。
TODO 对于 RefCell<T>，这些不可变性作用于 运行时

TODO  Rc<T>，RefCell<T> 只能用于单线程场景

Box Rc  RefCell 的选择场景


Rc<T> 允许相同数据有多个所有者；TODO Box<T> 和 RefCell<T> 有单一所有者。
Box<T> 允许在编译时执行【不可变】或【可变】借用检查； TODO Rc<T>仅允许在编译时执行 【不可变】借用检查；
RefCell<T> 允许在运行时执行【不可变】或【可变】借用检查

*/

//// 下面的写法是会报错的
//fn main() {
//    let x = 5;
//    let y = &mut x; // 不允许将 不可变的 变成可变的
//}


//// 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
//// TODO RefCell<T> 的一个常见用法是与 Rc<T> 结合
//#[derive(Debug)]
//enum List {
//    Cons(Rc<RefCell<i32>>, Rc<List>),
//    Nil,
//}
//
//use List::{Cons, Nil};
//use std::rc::Rc;
//use std::cell::RefCell;
//
//fn main() {
//    let value = Rc::new(RefCell::new(5));
//
//    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
//
//    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
//    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
//
//    *value.borrow_mut() += 10;
//
//    println!("a after = {:?}", a);
//    println!("b after = {:?}", b);
//    println!("c after = {:?}", c);
//}


//
//
//// TODO  人为制造 内存溢出  或者 避免引用循环：将 Rc<T> 变为 Weak<T>  （没试过这个Weak）
//fn main() {
//    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
//
//    println!("a initial rc count = {}", Rc::strong_count(&a));
//    println!("a next item = {:?}", a.tail());
//
//    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
//
//    println!("a rc count after b creation = {}", Rc::strong_count(&a));
//    println!("b initial rc count = {}", Rc::strong_count(&b));
//    println!("b next item = {:?}", b.tail());
//
//    if let Some(link) = a.tail() {
//        *link.borrow_mut() = Rc::clone(&b);
//    }
//
//    println!("b rc count after changing a = {}", Rc::strong_count(&b));
//    println!("a rc count after changing a = {}", Rc::strong_count(&a));
//
//    // 取消如下行的注释来观察引用循环；
//    // 这会导致栈溢出
//    // println!("a next item = {:?}", a.tail());
//
//}
//use std::rc::Rc;
//use std::cell::RefCell;
//use List::{Cons, Nil};
//
//#[derive(Debug)]
//enum List {
//    Cons(i32, RefCell<Rc<List>>),
//    Nil,
//}
//
//impl List {
//    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//        match self {
//            Cons(_, item) => Some(item),
//            Nil => None,
//        }
//    }
//}


//// TODO 用了 Weak
//use std::rc::{Rc, Weak};
//use std::cell::RefCell;
//
//#[derive(Debug)]
//struct Node {
//    value: i32,
//    parent: RefCell<Weak<Node>>,
//    children: RefCell<Vec<Rc<Node>>>,
//}
//
//fn main() {
//    let leaf = Rc::new(Node {
//        value: 3,
//        parent: RefCell::new(Weak::new()),
//        children: RefCell::new(vec![]),
//    });
//
//    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
//
//    let branch = Rc::new(Node {
//        value: 5,
//        parent: RefCell::new(Weak::new()),
//        children: RefCell::new(vec![Rc::clone(&leaf)]),
//    });
//
//    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
//
//    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
//}


//// TODO 又
//use std::rc::{Rc, Weak};
//use std::cell::RefCell;
//
//#[derive(Debug)]
//struct Node {
//    value: i32,
//    parent: RefCell<Weak<Node>>,
//    children: RefCell<Vec<Rc<Node>>>,
//}
//
//fn main() {
//    let leaf = Rc::new(Node {
//        value: 3,
//        parent: RefCell::new(Weak::new()),
//        children: RefCell::new(vec![]),
//    });
//
//    println!(
//        "leaf strong = {}, weak = {}",
//        Rc::strong_count(&leaf),
//        Rc::weak_count(&leaf),
//    );
//
//    {
//        let branch = Rc::new(Node {
//            value: 5,
//            parent: RefCell::new(Weak::new()),
//            children: RefCell::new(vec![Rc::clone(&leaf)]),
//        });
//
//        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
//
//        println!(
//            "branch strong = {}, weak = {}",
//            Rc::strong_count(&branch),
//            Rc::weak_count(&branch),
//        );
//
//        println!(
//            "leaf strong = {}, weak = {}",
//            Rc::strong_count(&leaf),
//            Rc::weak_count(&leaf),
//        );
//    }
//
//    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
//    println!(
//        "leaf strong = {}, weak = {}",
//        Rc::strong_count(&leaf),
//        Rc::weak_count(&leaf),
//    );
//}




















/*

// 并发 (无畏并发)

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // TODO 使用 join 等待所有线程结束
    // 类似与 go 中的  wg.Wait()
    handle.join().unwrap();
}*/



/*
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

//    let handle = thread::spawn( || { // 这样纸写的话  v 的所有权是没有被移动到闭包中来的。 TODO Rust 不知道这个新建线程会执行多久，所以无法知晓 v 的引用是否一直有效
    let handle = thread::spawn(move || { // 用 move 就可以表示将 v 移到 闭包中来
        println!("Here's a vector: {:?}", v);
    });

    // 如果不用 move的话，因为可能，我这里就把 v 给 drop 掉了
    // drop(v);

    handle.join().unwrap();
}*/


/*
// 在线程之间传递数据
use std::thread;
use std::sync::mpsc;

fn main() {
    // 声明一个 chan
    let (tx, rx) = mpsc::channel();


    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}*/


/*
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {}", val); // 这一行 会报错，因为val 已经被发送到 其他线程了(所有权已经被转移了)
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}*/


/*
// 使用 for 来接受 chan中的信息， TODO 类似 go 中的 for-rage 操作 chan
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}*/

//
//// TODO 通过克隆发送者来创建多个生产者
//use std::thread;
//use std::sync::mpsc;
//use std::time::Duration;
//
//fn main() {
//
//    let (tx, rx) = mpsc::channel();
//
//    let tx1 = mpsc::Sender::clone(&tx); // TODO clone 了chan的发送端
//
//
//    thread::spawn(move || {
//        let vals = vec![
//            String::from("hi"),
//            String::from("from"),
//            String::from("the"),
//            String::from("thread"),
//        ];
//
//        for val in vals {
//            tx1.send(val).unwrap();
//            thread::sleep(Duration::from_secs(1));
//        }
//    });
//
//    thread::spawn(move || {
//        let vals = vec![
//            String::from("more"),
//            String::from("messages"),
//            String::from("for"),
//            String::from("you"),
//        ];
//
//        for val in vals {
//            tx.send(val).unwrap();
//            thread::sleep(Duration::from_secs(1));
//        }
//    });
//
////    for received in rx {
////        println!("Got: {}", received);
////    }
//
//
//   /* // clone 多个接受者 TODO 这样子是不行的
//    let rx1 = mpsc::Receiver::clone(&rx);
//
//    thread::spawn(move ||{
//        for received in rx {
//            println!("Rev1: {}", received);
//            thread::sleep(Duration::from_secs(1));
//        }
//    });
//
//    thread::spawn(move ||{
//        for received in rx1 {
//            println!("Rev2: {}", received);
//            thread::sleep(Duration::from_secs(1));
//        }
//    });*/
//
//
//}



/*
// 使用 锁
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        *//*
        Mutex<T> 是一个智能指针。更准确的说，lock 调用 返回 一个叫做 MutexGuard 的智能指针。
        这个智能指针实现了 Deref 来指向其内部数据；其也提供了一个 Drop 实现当 MutexGuard 离开作用域时自动释放锁
        *//*
        let mut num = m.lock().expect("error~");
        *num = 6;
    }

    println!("m = {:?}", m);
}

*/

/*
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {

        // TODO 注意这里，表示第一个 thread 就已经拿到了 counter 的所有权
        // TODO 所以，后续的 thread 再拿就会报错了
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}


*/


/*
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    // TODO 这个也是一样，counter 在这个 thread 中被移进 闭包，
    // TODO 则Q其他 thread 就不能使用 counter 了
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
    });
    handles.push(handle);

    let handle2 = thread::spawn(move || {
        let mut num2 = counter.lock().unwrap();

        *num2 += 1;
    });
    handles.push(handle2);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}*/



/*
// 使用 技术引用 来包装 mutex 从而做到了一在 多个 thread 中获取 mutex
// TODO 注意： 不幸的是，Rc<T> 并不能安全的在线程间共享
// TODO 在多线程间使用  计数引用的话，需要用 Arc<T> 一个类似 Rc<T> 并可以安全的用于并发环境的类型
// TODO 因为 Rc 不是线程安全的， 可能存在 多个线程 同时修改 Rc 的引用计数，导致 计数不准


use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}*/

//
//use std::sync::{Mutex, Arc};
//use std::thread;
//
//fn main() {
//    let counter = Arc::new(Mutex::new(0));
//    let mut handles = vec![];
//
//    for i in 0..10 {
//        let counter = Arc::clone(&counter);
//        let handle = thread::spawn(move || {
//            let mut num = counter.lock().unwrap();
//
//
//            println!("这是一个线程:{}", i)
////            *num += 1;
//        });
//        handles.push(handle);
//    }
//
//    for handle in handles {
//        handle.join().unwrap();
//    }
//
//    println!("Result: {}", *counter.lock().unwrap());
//}


//// 面向对象编程
//
//pub struct AveragedCollection {
//    list: Vec<i32>,
//    average: f64,
//}
//
//impl AveragedCollection {
//
//    pub fn add(&mut self, value: i32) {
//        self.list.push(value);
//        self.update_average();
//    }
//
//    pub fn remove(&mut self) -> Option<i32> {
//        let result = self.list.pop();
//        match result {
//            Some(value) => {
//                self.update_average();
//                Some(value)
//            }
//            None => None,
//        }
//    }
//
//    pub fn average(&self) -> f64 {
//        self.average
//    }
//
//    fn update_average(&mut self) {
//        let total: i32 = self.list.iter().sum();
//        self.average = total as f64 / self.list.len() as f64;
//    }
//}
//
//
//
//pub trait Draw {
//    fn draw(&self);
//}
//
//
//pub struct Screen {
//    pub components: Vec<Box<dyn Draw>>,
//}
//
//
//impl Screen {
//
//    pub fn run(&self) {
//        for component in self.components.iter() {
//            component.draw();
//        }
//    }
//}
//
////impl<T> Screen<T>
////    where T: Draw {
////    pub fn run(&self) {
////        for component in self.components.iter() {
////            component.draw();
////        }
////    }
////}
//
//
//
//
//pub struct Button {
//    pub width: u32,
//    pub height: u32,
//    pub label: String,
//}
//
//impl Draw for Button {
//    fn draw(&self) {
//        // 实际绘制按钮的代码
//    }
//}
//
//
//struct SelectBox {
//    width: u32,
//    height: u32,
//    options: Vec<String>,
//}
//
//impl Draw for SelectBox {
//    fn draw(&self) {
//        // code to actually draw a select box
//    }
//}
//
////
////fn main() {
////    let screen = Screen {
////        components: vec![
////            Box::new(SelectBox {
////                width: 75,
////                height: 10,
////                options: vec![
////                    String::from("Yes"),
////                    String::from("Maybe"),
////                    String::from("No")
////                ],
////            }),
////            Box::new(Button {
////                width: 50,
////                height: 10,
////                label: String::from("OK"),
////            }),
////        ],
////    };
////
////    screen.run();
////}
//
//
//fn main() {
//    let screen = Screen {
//        components: vec![
//            Box::new(String::from("Hi")),
//        ],
//    };
//
//    screen.run();
//}


//fn main() {
//    let v = vec!['a', 'b', 'c'];
//
//    // for in
//    for (index, value) in v.iter().enumerate() {
//        println!("{} is at index {}", value, index);
//    }
//}


//fn main() {
//
//    // TODO 字面量匹配
//    let x = 1;
//
//    match x {
//        1 => println!("one"), // 匹配
//        2 => println!("two"),
//        3 => println!("three"),
//        _ => println!("anything"),
//    }
//
//
//    // TODO 命名匹配
//    let x = Some(5);
//    let y = 10;
//
//    match x {
//        Some(50) => println!("Got 50"),
//        Some(y) => println!("Matched, y = {:?}", y),  // 匹配
//        _ => println!("Default case, x = {:?}", x),
//    }
//
//    println!("at the end: x = {:?}, y = {:?}", x, y);
//
//
//    // TODO 多模式匹配
//    let x = 1;
//
//    match x {
//        1 | 2 => println!("one or two"),  // 匹配
//        3 => println!("three"),
//        _ => println!("anything"),
//    }
//
//
//    // TODO  范围匹配
//    let x = 5;
//
//    match x {
//        1...5 => println!("one through five"),   // 匹配
//        _ => println!("something else"),
//    }
//
//    let x = 'c';
//
//    match x {
//        'a'...'j' => println!("early ASCII letter"),
//        'k'...'z' => println!("late ASCII letter"),
//        _ => println!("something else"),
//    }
//
//
//    // 解构匹配
//    struct Point {
//        x: i32,
//        y: i32,
//    }
//
//    let p = Point { x: 0, y: 7 };
//
//    match p {
//        Point { x, y: 0 } => println!("On the x axis at {}", x),
//        Point { x: 0, y } => println!("On the y axis at {}", y), // 匹配
//        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
//
//    }
//
//
//
//    // 枚举匹配
//    enum Message {
//        Quit,
//        Move { x: i32, y: i32 },
//        Write(String),
//        ChangeColor(i32, i32, i32),
//    }
//
//
//    let msg = Message::ChangeColor(0, 160, 255);
//
//    match msg {
//        Message::Quit => {
//            println!("The Quit variant has no data to destructure.")
//        },
//        Message::Move { x, y } => {
//            println!(
//                "Move in the x direction {} and in the y direction {}",
//                x,
//                y
//            );
//        }
//        Message::Write(text) => println!("Text message: {}", text),
//        Message::ChangeColor(r, g, b) => {
//            println!(
//                "Change the color to red {}, green {}, and blue {}",
//                r,
//                g,
//                b
//            )
//        }
//    }
//
//
//    let points = vec![
//        Point { x: 0, y: 0 },
//        Point { x: 1, y: 5 },
//        Point { x: 10, y: -3 },
//    ];
//
//    let sum_of_squares: i32 = points
//        .iter()
//        // 如果没有在 &Point { x, y } 中包含 & 则会得到一个类型不匹配错误，因为这样 iter 会遍历 vector 中项的引用而不是值本身
//        .map(|&Point { x, y }| x * x + y * y)
//        .sum();
//
//
//
//
//
//    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
//
//
//    // 使用 _ 部分匹配
//    let mut setting_value = Some(5);
//    let new_setting_value = Some(10);
//
//    match (setting_value, new_setting_value) {
//        (Some(_), Some(_)) => {
//            println!("Can't overwrite an existing customized value");
//        }
//        _ => {
//            setting_value = new_setting_value;
//        }
//    }
//
//    println!("setting is {:?}", setting_value);
//
//
//    let numbers = (2, 4, 8, 16, 32);
//
//    match numbers {
//        (first, _, third, _, fifth) => {
//            println!("Some numbers: {}, {}, {}", first, third, fifth)
//        },
//    }
//
//
//
//
//    // 在名字前以一个下划线开头来忽略未使用的变量
//    let _x = 5;
//    let y = 10;
//
//    // 当然你也可以是用 这变量
//    let s = Some(String::from("Hello!"));
////
////    if let Some(_s) = s {
////        println!("found a string");
////    }
//
//    // 而这样下面的 println 就不会报错
//    if let Some(_) = s {
//        println!("found a string");
//    }
//
//    println!("{:?}", s);
//
//
//    // 使用  .. 图略剩下的值
//    struct Point2 {
//        x: i32,
//        y: i32,
//        z: i32,
//    }
//
//    let origin = Point2 { x: 0, y: 0, z: 0 };
//
//    match origin {
//        Point2 { x, .. } => println!("x is {}", x),
//    }
//
//    // 或者
//    let numbers = (2, 4, 8, 16, 32);
//
//    match numbers {
//        (first, .., last) => {
//            println!("Some numbers: {}, {}", first, last);
//        },
//    }
//
//
//    // TODO 模式匹配 守卫
//    let num = Some(4);
//
//    match num {
//        Some(x) if x < 5 => println!("less than five: {}", x), // 匹配
//        Some(x) => println!("{}", x),
//        None => (),
//    }
//
//
//
//    let x = 4;
//    let y = false;
//
//    match x {
//        4 | 5 | 6 if y => println!("yes"),
//        _ => println!("no"), // 匹配
//    }
//
//
//
//
//    // TODO     @ 绑定
//
//    // 允许我们在创建一个存放值的变量的同时测试其值是否匹配模式
//    enum Message2 {
//        Hello { id: i32 },
//    }
//
//    let msg = Message2::Hello { id: 5 };
//
//    match msg {
//        Message2::Hello { id: id_variable @ 3...7 } => {  // 匹配
//            println!("Found an id in range: {}", id_variable)
//        },
//        Message2::Hello { id: 10...12 } => {
//            println!("Found an id in another range")
//        },
//        Message2::Hello { id } => {
//            println!("Found some other id: {}", id)
//        },
//    }
//
//}


//// todo 解构并分解值
//struct Point {
//    x: i32,
//    y: i32,
//}
//
//fn main() {
//    let p = Point { x: 0, y: 7 };
//
//    // TODO 注意看这里哦
//    let Point { x: a, y: b } = p;
//    assert_eq!(0, a);
//    assert_eq!(7, b);
//}














































//// 使用外部函数 （如：调用 C）
//
//extern "C" {
//    fn abs(input: i32) -> i32;
//}
//
//fn main() {
//    unsafe {
//        println!("Absolute value of -3 according to C: {}", abs(-3));
//    }
//}
//
//
//// 返回闭包
//fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
//    Box::new(|x| x + 1)
//}
//
//// 无返回类型
//fn bar() -> ! {
//    print!("forever ");
//
//    loop { // 最后一个有着 ! 类型的表达式是 loop
//        print!("and ever ");
//    }
//
//
//
//
//    let guess = "3";
//    loop { // 最后一个有着 ! 类型的表达式是 loop
//        let guess: u32 = match guess.trim().parse() {
//            Ok(num) => num,
//            Err(_) => continue,
//        };
//        break; // 不能结束 loop 否则报错
//    }
//}


/*
fn main() {
    let x = 1.0_f32/0.0;   // inf: 非0数/0 == inf

    let y = 0.0f32/0.0;    // NaN: 0/0 == NaN

    println!["{}, {}", x, y];


    let  arr = [1, 2, 4, 8];

    // TODO  将 enum 的 variant 当做函数使用
    let v: Vec<Option<&i32>> = arr.iter().map(Some).collect();

    println!("{:#?}", v);


   *//* let v = loop{}; // 因为这个 loop 永远执行不完，所以 v的类型为"!" 即： 发散类型
    println!("{}", v);  // 则这句打印永远不会被执行*//*


    let me = Chef;

    // TODO 完整函数调用
    <Cook>::start(&me);     // 或者 <Chef as Cook>::start(&me);
    <Wash>::start(&me);     // 或者 <Chef as Wash>::start(&me);
    <Chef as Cook>::start(&me);
    <Chef as Wash>::start(&me);




    let a : isize = 120;
    println!("{}", a)


}


trait Cook {
    fn start(&self);
}

trait Wash {
    fn start(&self);
}



struct Chef;

impl Cook for Chef {
    fn start (&self) {
        println!("Cook::start")
    }
}

impl Wash for Chef {
    fn start (&self) {
        println!("Wash::start")
    }
}
*/



//fn main() {
//    let arr : [i32; 5] = [1, 2, 3, 4, 5];
//    let addr : &[i32; 5] = &arr;
//    println!("addr of arr: {:p}", addr);
//    raw_slice(addr as &[i32])
//
//}
//
//
//fn raw_slice(arr: &[i32]) {
//    // 打印胖指针的内存
//    unsafe {
//        let (val1, val2) : (usize, usize) = std::mem::transmute(arr);
//        println!("Value in raw pointer:");
//        println!("addr: {:x}", val1);
//        println!("size: {:x}", val2);
//    }
//}


//fn main() {
////    let r = 1..10; // Range<i32> 类型， 而不是 元组哦， 没有索引
////    for v in r {
////        println!("{:?}", v)
////    }
//
//    use std::iter::Iterator;
//    let r = (1i32..13).rev().map(|i| i*10);
//
//    for v in r {
//        println!("{:?}", v)
//    }
//
//    let r = 1..; // (1, +∞)
//    let r = .. 13; // (-∞, 13)
//    let r = ..; // (-∞, +∞)
//
//
//
//
//    // 边界检查
//
//    let v = [10i32, 20, 30, 40, 50];
//
//    let index : usize = std::env::args().nth(1).map(|x|x.parse().unwrap_or(0)).unwrap_or(0);
//    println!("{:?}", v[index]);
//
//}


//fn main() {
//    // 变量绑定  @
//    let x = 1;
//
//    match x {
//        // 表示e来匹配 [1,...,5] 中的任意元素
//        e@1..=5 => println!("{}", e),
//        _ => println!("anythings")
//    }
//
//
//    // 如果@和|一起用，需要在每个变量上都加上 @
//
//    let x = 5;
//    match x {
//        e @ 1..=5 | e@8..=10 => println!("{}", e),
//        _ => println!("anythings")
//    }
//
//
//
//    // 利用编译错误查看 类型
//    fn type_id (_ : ()) {}
//
//    let ref x = 5_i32;
//
////    type_id(x); // 这里就会给我们打印出来 `x`的真实类型： &i32
//
//
//    let ref x = 5_i32;
//    print_type_name(&x) // 第二种查看类型的 方法
//}
//
//
//fn print_type_name<T> (_args:  &T) {
//    unsafe {
//        println!("{}", std::intrinsics::type_name::<T>())
//    }
//}


//fn main() {
//    let mut x : Option<String> = Some("hello".into());
//
//    match x {
//        Some(i) => i.push_str("world"), // cannot borrow as mutable
//        None => println!("None"),
//    }
//
//    println!("{:?}", x);
//}


//fn main() {
//    let x = T{
//        item1: 'A',
//        item2: false,
//    };
//
//    test(x);
//}
//
//struct T {
//    item1: char,
//    item2: bool,
//}
//
//
//// TODO 当一个函数接受一个结构体参数的时候， 可以直接在参数中做 模式解构
//fn test (T{item1: args1, item2: args2} : T){
//    println!("{}, {}", args1, args2)
//}


//fn main() {
//    let x = 1_i32;
//
//    let mut y : u32 = 1;
//
//    let raw = &mut y as *mut u32 as *mut i32 as *mut i64;
//
//    unsafe {
//        *raw = -1;
//    }
//
//    println!("{:X}, {:X}", x, y)
//}



fn raw_to_ref<'a>(p : *const i32) -> & 'a i32 {
    unsafe{
        &*p
    }
}

fn main() {
    let p : &i32 = raw_to_ref(std::ptr::null::<i32>());
    println!("{}", p) // interrupted by signal 11: SIGSEGV
}



