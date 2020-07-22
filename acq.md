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

- **if语句**

  - 必须加大括号
  - 条件参数必须为bool型，不会自动转换

- **返回值**

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

- **函数**：

  ```rust
  fn foo(num: i32) { // 无返回值
      
  }
  fn foo(num: i32) -> i32 { // 有返回值
      1
  }
  ```

- **循环**：

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

- **tuple**：

  ```rust
  fn main() {
      let tup: (i32, f64, u8) = (500, 6.4, 1);
  
      let (x, y, z) = tup; // destruction
  
      println!("The value of y is: {}", y);
      
      let k = tup.0; // 从0开始编号
  }
  ```

- **数组**：

  ```rust
  let a = [1, 2, 3, 4, 5];
  let b = ["王么略", "baoyi"];
  
  let a: [i32; 5] = {1, 2, 3, 4, 5}; // 等价于int a[5];
  let a = [3; 5]; // int a[5]后统一初始化为3
  ```

  

- 特性：**ownership**

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
  
- **引用**

  **默认为常值引用**：We’re not allowed to modify something we have a reference to.

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

- **slice**

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




- **结构体**

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
      ..user1 // 这里！直接把user1的其他内容复制到user2
  };
  
  struct color(i32, i32, i32); // 从tuple生成struct
  ```

  