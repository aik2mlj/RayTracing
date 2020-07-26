# Rust

- `println!`: a macro

- 赋值语句：`let foo = dfsdf`

- 默认是常量，变量加mut：`let mut foo = bar`

- ```rust
  let x = 5;
  let y = 10;
  
  println!("x = {} and y = {}", x, y); // x = 5 and y = 10
  ```

- const 必须声明常量类型

### **if语句**

- 必须加大括号
- 条件参数必须为bool型，不会自动转换

### **返回值**

```rust
let y = {
    let x = 3;
    x + 1 // 这里没有分号！！！
}; // 这里有
```

“You can return early from a function by using the `return` keyword and specifying a value, but most functions return the last expression implicitly. ”

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

### **函数**

```rust
fn foo(num: i32) { // 无返回值
    
}
fn foo(num: i32) -> i32 { // 有返回值
    1
}
```

### **循环**

- loop：无限循环

  `break x + 1;` 这样子就把x + 1传出loop作为返回值（注意这里有分号）

- while：一般替换c++中的`for(int i = 1; i < 10; ++i)`

  ```rust
  while x <= 10 {
      ++x;
  }
  ```

- for：遍历数组等很方便

  ```rust
  a = [10, 20, 40, 70];
  for i in a.iter() {
      println!(i);
  }
  
  for i in 0..10 { // 左闭右开    
  }
  for i in (0..10).rev() { // 倒过来
  }
  ```

### **tuple**

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup; // destruction

    println!("The value of y is: {}", y);

    let k = tup.0; // 从0开始编号
}
```

### **数组**

```rust
let a = [1, 2, 3, 4, 5];
let b = ["王么略", "baoyi"];

let a: [i32; 5] = [1, 2, 3, 4, 5]; // 等价于int a[5]; 注意这里是[]而不是{}!
let a = [3; 5]; // int a[5]后统一初始化为3
```


### **ownership**

一个变量只能有一个主子

**Rust will never automatically create “deep” copies of your data.** 

例如：

```rust
let s1 = String::from("hello");
let s2 = s1; // 这里并没有实现深拷贝，而是将s1视为invalid，move到了s2

let s2 = s1.clone(); // 这才是深拷贝
```

自动实现深拷贝的仅有：（即 **Copy Trait**）

- integers、booleans、floats、char、simple tuple(no String etc.)

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    // 这里以后调用s将会编译失败

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

### **引用**

**默认为常值引用**：We’re not allowed to modify somethingwe have a reference to.

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 注意这里也要写&（和c++不一样），但是意义是引用而非指针
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
```

可变值引用：**在传参及函数参数列表中加上`mut`**

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s); // 这里
}

fn change(some_string: &mut String) { // 这里
    some_string.push_str(", world");
}
```

特性：rust仅允许同时出现一个可变值的引用，以防止“data race”（与c++不同）。同样，不允许同时出现一个变量的常值引用和变值引用。

引用的生命周期为：定义 -> 最后一次被使用

### **slice**

```rust
let ss = [1, 2, 3, 4, 5];
let s = &ss[0..3]; // s = [1, 2, 3] 左闭右开
```

```rust
let ss = String::from("hello world!");
let s = &ss[1..]; // 1到结尾
// ..前留空指从0开始，..后留空指到结尾
```

事实上，`let s = "hello world"`中，s是一个`&str`，即字符数组的常值引用


### **结构体**

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool, // 末尾可以留逗号
} // 定义（这里无分号！）

let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
}; // 初始化
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1 // 这里！直接把user1的其他内容复制到user2（无分号！）
};

struct color(i32, i32, i32); // 从tuple生成struct

impl User { // 类函数（“methods”）
    fn blahblah() -> i32 {
        ...
    }
}
```

### **enum**（真的好用）

```rust
enum Message {
    Quit, // 一般enum类的样子：啥也没有
    Move { x: i32, y: i32 }, // 关联一个无名结构体！
    Write(String), // 关联String类
    ChangeColor(i32, i32, i32), // 关联一个tuple
}
// 同时只能出现一种情况，Quit|Move|Write|ChangeColor四选一

初始化：
let m = Message::Write(String::from("hello"));
```

同样地，`enum`也可以定义method即类函数

- 特殊的enum：`Option`

  为了解决**空引用**的问题：

  ```rust
  enum Option<T> {
      Some(T),
      None,
  }
  ```

  在此，None就是”空引用“

### **match**

配合enum使用：

```rust

```



### **Trait**

实现了类似“基类函数”的作用，即可对其他的struct部署相同功能的method

定义：

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

部署(implement)

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle { // 这一行！
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

可以有默认部署：

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}
```

这样若对结构体重新部署则会override，否则只需：

```rust
impl Summary for NewsArticle {}
```

就可以使用默认部署（类似虚函数）

- 作为函数参数类型：

  ```rust
  pub fn notify(item: &impl Summary) {
      println!("Breaking news! {}", item.summarize());
  }
  ```

  这样子，任何impl过`Summary`这个trait的type都可以调用`notify`函数

  *事实上这是一个”语法糖“，完整的形式为：*

  ```rust
  pub fn notify<T: Summary>(item: &T) {
      println!("Breaking news! {}", item.summarize());
  }
  ```

- 作为函数返回值：

  ```rust
  fn returns_summarizable() -> impl Summary {
      Tweet {
          username: String::from("horse_ebooks"),
          content: String::from(
              "of course, as you probably already know, people",
          ),
          reply: false,
          retweet: false,
      }
  }
  ```

  返回一个部署了`Summary`的type

  注意：函数返回值还是只能有一个类型，不可以返回多种类型！

- `where`从句

  ```rust
  fn some_function<T, U>(t: &T, u: &U) -> i32
      where T: Display + Clone,
            U: Clone + Debug
  {...}
  // 等价于
  fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {...}
  ```

  这里`T: Display + Clone`就说明了`T`需要是一个部署了`Display`和`Clone`的类型

  `where`从句更简洁地说明了调用generic的限制

- 选择性部署trait
  ```rust
  struct Pair<T> {
      x: T,
      y: T,
  }
  
  impl<T> Pair<T> { // 无选择：所有类型都可以使用的method
      fn new(x: T, y: T) -> Self {
          Self { x, y }
      }
  }
  
  impl<T: Display + PartialOrd> Pair<T> { // 只有部署了Display和PartialOrd的类型才可以使用的method
      fn cmp_display(&self) {
          if self.x >= self.y {
              printmethodln!("The largest member is x = {}", self.x);
          } else {
              println!("The largest member is y = {}", self.y);
          }
      }
  }
  ```



# RayTracer

### shadow acne

由于精度问题导致，本该全被照亮的地方判定出现了阴影。

解决：略微调高`hit`的反应阈值，0.0 -> 0.001

对比：

![no_shadow_acne](C:\Users\ASUS\Desktop\ppca\RayTracing\some result\no_shadow_acne.png)![shadow_acne](C:\Users\ASUS\Desktop\ppca\RayTracing\some result\shadow_acne.png)