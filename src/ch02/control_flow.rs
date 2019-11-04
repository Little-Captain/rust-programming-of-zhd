/// # if表达式
///
/// Basic usage:
///
/// ```
/// fn if_expr() {
///     let n = 13;
///     let big_n = if n < 10 && n > -10 {
///         10 * n
///     } else {
///         n / 2
///     };
///     assert_eq!(big_n, 6);
/// }
/// if_expr();
/// ```
pub fn if_expr() {
    let n = 13;
    let big_n = if (n < 10 && n > -10) { 10 * n } else { n / 2 };
    println!("{} == {}", big_n, 6)
}

/// # while表达式
///
/// Basic usage:
///
/// ```
/// fn while_fizzbuzz() {
///     let mut n = 1;
///     while n < 101 {
///         if n % 15 == 0 {
///             println!("fizzbuzz");
///         } else if n % 3 == 0 {
///             println!("fizz");
///         } else if n % 5 == 0 {
///             println!("buzz");
///         } else {
///             println!("{}", n);
///         }
///         n += 1;
///     }
/// }
/// while_fizzbuzz();
/// ```
pub fn while_fizzbuzz() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}

/// # while表达式
///
/// Basic usage:
///
/// ```
/// fn loop_fizzbuzz() {
///     let mut n = 1;
///     loop {
///         if n > 101 { break; }
///         if n % 15 == 0 {
///             println!("fizzbuzz");
///         } else if n % 3 == 0 {
///             println!("fizz");
///         } else if n % 5 == 0 {
///             println!("buzz");
///         } else {
///             println!("{}", n);
///         }
///         n += 1;
///     }
/// }
/// loop_fizzbuzz();
/// ```
pub fn loop_fizzbuzz() {
    let mut n = 1;
    loop {
        if n >= 101 {
            break;
        }
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}

/// # for表达式
///
/// Basic usage:
///
/// ```
/// fn for_fizzbuzz() {
///     for n in 1..101 {
///         if n % 15 == 0 {
///             println!("fizzbuzz");
///         } else if n % 3 == 0 {
///             println!("fizz");
///         } else if n % 5 == 0 {
///             println!("buzz");
///         } else {
///             println!("{}", n);
///         }
///     }
/// }
/// for_fizzbuzz();
/// ```
pub fn for_fizzbuzz() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

/// # while true
///
/// 当需要使用无限循环的时候, 请务必使用 loop 循环, 避免使用 while true 循环
///
/// Basic usage:
///
/// ```
/// fn while_true(x: i32) -> i32 {
///     while true {  // error[E0308]: mismatched types，expected type `i32` found type `()`
///         return x+1;
///     }
/// }
/// let y = while_true(5);
/// assert_eq!(y, 6);
/// ```
pub fn while_true(x: i32) -> i32 {
    // 错误提示称 while true 循环块返回的是单元值,
    // 而函数 while true 返回值是 i32, 所以不匹配
    // 因为 Rust 编译器在对 while 循环做流分析(Flow Sensitive)的时候,
    // 不会检查循环条件, 编译器会认为 while 循环条件可真可假, 所以循环体
    // 里的表达式也会被忽略, 此时编译器只知道 while true 循环返回的是单元值,
    // 而函数返回的是 i32, 其他情况一概不知. 这一切都是因为 CTFE 功能的限制,
    // while 条件表达式无法作为编译器常量来使用. 只有等将来 CTFE 功能完善了,
    // 才可以正常使用. 同理, if true 在只有一条分支的情况下, 也会发生类似情况
    // while true {
    //     return x + 1;
    // }
    // x
    loop {
        return x + 1;
    }
}

/// # if true
///
/// Basic usage:
///
/// ```
/// fn if_true(x: i32) -> i32 {
///     if true {  // error[E0308]: mismatched types，expected type `i32` found type `()`
///         return x+1;
///     }
/// }
/// let y = if_true(5);
/// assert_eq!(y, 6);
/// ```
pub fn if_true(x: i32) -> i32 {
    // error[E0308]: mismatched types，expected type `i32` found type `()`
    if true {
        return x + 1;
    }
    x
}

/// # match 匹配
///
/// 在 Rust 语言中, match 分支使用了模式匹配(Pattern Matching)技术
/// 模式匹配在数据结构字符串中经常出现, 比如在某个字符串中找出与该子串相同的所有子串
/// 在编程语言中, 模式匹配用于判断类型或值是否存在可以匹配的模式
/// 模式匹配在很多函数式语言中己经被广泛应用
///
/// 在 Rust 语言中, match 分支左边就是模式, 右边就是执行代码
/// 模式匹配同时也是一个表达式, 和 if 表达式类似, 所有分支必须返回同一个类型
/// 但是左侧的模式可以是不同的. 下面代码中使用的模式分别是单个值、范围、多个值和通配符
/// 其中值得注意的是, 在代码中, 使用操作符 @ 可以将模式中的值绑定给一个变量
/// 供分支右侧的代码使用, 这类匹配叫绑定模式(Binding Mode)
/// match 表达式必须穷尽每一种可能, 所以一般情况下, 会使用通配符来处理剩余的情况
///
/// Basic usage:
///
/// ```
/// fn match_expr() {
///     let number = 42;
///     match number {
///         0 => println!("Origin"),  // 匹配数字
///         1...3 => println!("All"), // 匹配范围
///         5 | 7 | 13 => println!("Bad Luck"), // 匹配相同的分支
///         n @ 42 => println!("Answer is {}", n), // 使用@可以创建绑定n，分支右侧表达式中可用
///         _ => println!("Common"),  // 下划线为通用匹配
///     }
/// }
/// match_expr();
/// ```
pub fn match_expr(number: i32) {
    match number {
        0 => println!("Origin"),
        1..=3 => println!("All"),
        5 | 7 | 13 => println!("Bad Luck"),
        n @ 42 => println!("Answer is {}", n),
        _ => println!("Common"),
    }
}

// # match 匹配布尔值
///
/// Basic usage:
///
/// ```
/// fn match_bool() {
///     let boolean = true;
///     let binary = match boolean {
///         false => 0,
///         true => 1,
///     };
///     assert_eq!(binary, 1);
/// }
/// match_bool();
/// ```

// # if let
///
/// Basic usage:
///
/// ```
/// fn if_let_bool() {
///     let boolean = true;
///     let mut binary = 0;
///     if let true = boolean {
///         binary = 1;
///     }
///     assert_eq!(binary, 1);
/// }
/// if_let_bool();
/// ```

// # while let
///
/// Basic usage:
///
/// ```
/// fn while_let_pop() {
///     let mut v = vec![1,2,3,4,5];
///     while let Some(x) = v.pop() {
///         println!("{}", x);
///     }
/// }
/// while_let_pop();
/// ```

// # while match
///
/// Basic usage:
///
/// ```
/// fn loop_match_pop() {
///     let mut v = vec![1,2,3,4,5];
///     loop {
///         match v.pop() {
///             Some(x) => println!("{}", x),
///             None => break,
///         }
///     }
/// }
/// loop_match_pop();
/// ```
fn loop_match_pop() {}
