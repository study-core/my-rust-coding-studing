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
   宏转换（将具有不完整参数的调用转换为规范形式）

   宏中的匹配规则

   宏中使用字面量匹配的哦

   几种指示符：

   ident: 标识符，用来表示函数或变量名
   expr: 表达式
   block: 代码块，用花括号包起来的多个语句
   pat: 模式，普通模式匹配（非宏本身的模式）中的模式，例如 Some(t), (3, 'a', _)
   path: 路径，注意这里不是操作系统中的文件路径，而是用双冒号分隔的限定名(qualified name)，如 std::cmp::PartialOrd
   tt: 单个语法树
   ty: 类型，语义层面的类型，如 i32, char
   item: 条目，
   meta: 元条目
   stmt: 单条语句，如 let a = 42;


   */


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
}

/*
定义一个宏 (导出的宏，可以被任意地方引用)
一定要有：
#[macro_export] 注解哦
*/
/*#[macro_export]
macro_rules! gavin {
    ( $( $x:expr ),* ) => {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    };
}*/

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