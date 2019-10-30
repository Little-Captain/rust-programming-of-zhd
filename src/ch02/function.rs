/// # 函数定义
///
/// Basic usage:
///
/// ```
/// pub fn fizz_buzz(num: i32) -> String {
///     if num % 15 == 0 {
///         return "fizzbuzz".to_string();
///     } else if num % 3 == 0 {
///         return "fizz".to_string();
///     } else if num % 5 == 0 {
///         return "buzz".to_string();
///     } else {
///         return num.to_string();
///     }
/// }
/// assert_eq!(fizz_buzz(15), "fizzbuzz".to_string());
/// assert_eq!(fizz_buzz(3), "fizz".to_string());
/// assert_eq!(fizz_buzz(5), "buzz".to_string());
/// assert_eq!(fizz_buzz(13), "13".to_string());
/// ```
pub fn fizz_buzz(num: i32) -> String {
    if num % 15 == 0 {
        "fizzbuzz".to_string()
    } else if num % 3 == 0 {
        "fizz".to_string()
    } else if num % 5 == 0 {
        "buzz".to_string()
    } else {
        num.to_string()
    }
}

/// # 词法作用域
///
/// Rust 语言的作用域是静态作用域, 即词法作用域(Lexical Scope)
/// 由一对花括号({})来开辟作用域, 其作用域在词法分析阶段就巳经确定了, 不会动态改变
///
/// Basic usage:
///
/// ```
/// pub fn lexical_scope(){
///     let v = "hello world!";
///     assert_eq!(v, "hello world!");
///     let v = "hello Rust!";
///     assert_eq!(v, "hello Rust!");
///     {
///         let v = "hello World!";
///         assert_eq!(v, "hello World!");
///     }
///     assert_eq!(v, "hello Rust!");
/// }
/// lexical_scope();
/// ```
pub fn lexical_scope() {
    let v = "hello world!";
    assert_eq!(v, "hello world!");
    // 连续用 let 定义同名变量的做法叫变量遮蔽(Variable Shadow)
    let v = "hello Rust!";
    assert_eq!(v, "hello Rust!");
    // 在词法作用域内部使用花括号开辟新的词法作用域后, 两个作用域是相互独立的
    // 在不同的词法作用域内声明的变量绑定, 拥有不同的生命周期(LifeTime)
    // 变量绑定的生命周期总是遵循这样的规律:
    // 从使用 let 声明创建变量绑定开始，到超出词法作用域的范围时结束
    {
        let v = "hello World!";
        assert_eq!(v, "hello World!");
    }
    assert_eq!(v, "hello Rust!");
}

/// # 函数指针： 函数作为参数
///
/// 在 Rust 中, 函数为一等公民. 这意味着, 函数自身就可以作为函数的参数和返回值使用
/// 函数指针类型: fn(xxx) -> xxx
/// 可以直接使用函数的名字来作为函数指针
///
/// Basic usage:
///
/// ```
/// pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32{
///     op(a, b)
/// }
/// fn sum(a: i32, b: i32) -> i32 {
///     a + b
/// }
/// fn product(a: i32, b: i32) -> i32 {
///     a * b
/// }
///
/// let a = 2;
/// let b = 3;
/// assert_eq!(math(sum, a, b), 5);
/// assert_eq!(math(product, a, b), 6);
/// ```
pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}

pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub fn product(a: i32, b: i32) -> i32 {
    a * b
}

/// # 函数指针 : 函数作为返回值
///
/// Basic usage:
///
/// ```
/// fn is_true() -> bool { true }
/// fn true_maker() -> fn() -> bool { is_true }
/// assert_eq!(true_maker()(), true);
/// ```
fn is_true() -> bool {
    true
}

pub fn true_maker() -> fn() -> bool {
    is_true
}

/// # CTFE: const fn
///
/// Rust 编译器也可以像 C＋＋或 D 语言那样,
/// 拥有编译时函数执行(Compile-Time Function Execution, CTFE)的能力
/// 使用 const fn 定义的函数, 必须可以确定值, 不能存在歧义
/// 与 fn 定义函数的区别在于, const fn 可以强制编译器在编译期执行函数
/// 其中关键字 const 一般用于定义全局常量
///
/// Rust 中的 CTFE 是由 miri 来执行的
/// miri 是一个 MIR 解释器, 目前己经被集成到了 Rust 编译器 rustc 中
/// Rust 编译器目前可以支持的常量表达式有: 字面量、元组、数组、字段结构体、
/// 枚举、只包含单行代码的块表达式、范围等
/// Rust 想要拥有完善的 CTFE 支持, 还有很多工作要做
///
/// Basic usage:
///
/// ```
/// // #![feature(const_fn)]
/// const fn init_len() -> usize {
///     return 5;
/// }
/// [0; init_len()];
/// ```
pub const fn init_len() -> usize {
    5
}

/// # 闭包
///
/// 闭包和函数有一个重要的区别, 那就是闭包可以捕获外部变量, 而函数不可以
///
/// Basic usage:
///
/// ```
/// pub fn closure(){
///     let out = 42;
///     // 普通函数
///     // fn  add(i: i32, j: i32) -> i32 { i + j + out}
///     fn  add(i: i32, j: i32) -> i32 { i + j }
///     // 定义类型标注的闭包
///     let closure_annotated = |i: i32, j: i32| -> i32 { i + j + out};
///     // 如果没有类型标注则由编译器推断类型
///     let closure_inferred  = |i, j| i + j + out;
///     let i = 1;
///     let j = 2;
///     assert_eq!(3, add(i, j));
///     assert_eq!(45, closure_annotated(i, j));
///     assert_eq!(45, closure_inferred(i, j));
/// }
/// closure();
/// ```
pub fn closure() {
    let out = 42;
    // 普通函数: 不能捕获上下文环境
    // fn add(i: i32, j: i32) -> i32 { i + j + out }
    fn add(i: i32, j: i32) -> i32 {
        i + j
    }
    let closure_annotated = |i: i32, j: i32| -> i32 { i + j + out };
    let closure_inferred = |i: i32, j: i32| i + j + out;
    let i = 1;
    let j = 2;
    println!("{} == {}", add(i, j), 3);
    println!("{} == {}", closure_annotated(i, j), 45);
    println!("{} == {}", closure_inferred(i, j), 45);
}

/// # 闭包: 作为参数
///
/// 闭包可以作为函数参数和返回值, 但使用起来略有区别
/// Rust 中闭包实际上就是由一个匿名结构体和 trait 来组合实现的
///
/// Basic usage:
///
/// ```
/// pub fn math<F: Fn() -> i32>(op: F) -> i32 {
///     op()
/// }
/// let a = 2;
/// let b = 3;
/// assert_eq!(math(|| a + b), 5);
/// assert_eq!(math(|| a * b), 6);
/// ```
pub fn closure_math<F: Fn() -> i32>(op: F) -> i32 {
    // 在函数内部, 通过在后面添加一对圆括号来调用传入的闭包
    op()
}

/// # 闭包: 作为返回值（动态分发）
///
/// Basic usage:
///
/// ```
/// fn two_times() -> Box<Fn(i32) -> i32> {
///     let i = 2;
///     Box::new(move |j| j * i)
/// }
/// let result = two_times();
/// assert_eq!(result(2), 4);
/// ```
pub fn two_times() -> Box<Fn(i32) -> i32> {
    let i = 2;
    Box::new(move |j| j * i)
}

/// # 闭包: 作为返回值（动态分发）Rust 2018
///
/// Basic usage:
///
/// ```
/// fn two_times_dyn() -> Box<dyn Fn(i32) -> i32> {
///     let i = 2;
///     Box::new(move |j| j * i)
/// }
/// let result = two_times_dyn();
/// assert_eq!(result(2), 4);
/// ```
pub fn two_times_dyn() -> Box<dyn Fn(i32) -> i32> {
    let i = 2;
    Box::new(move |j| j * i)
}

/// # 闭包: 作为返回值（静态分发）
///
/// Basic usage:
///
/// ```
/// fn two_times_impl() -> impl Fn(i32) -> i32{
///     let i = 2;
///     move |j| j * i
/// }
/// let result = two_times_impl();
/// assert_eq!(result(2), 4);
/// ```
/// 使用 impl Fn(i32) -> i32 作为函数的返回值
/// 表示实现 Fn(i32) -> 132 的类型
/// 在函数定义时并不知道具体的返回类型, 但是在函数调用时, 编译器会推断出来
/// 这个过程也是零成本抽象的, 一切都发生在编译期
pub fn two_times_impl() -> impl Fn(i32) -> i32 {
    let i = 2;
    // 需要注意的是, 在这里返回闭包时使用了 move 关键字
    // 这是因为在一般情况下, 闭包默认会按引用捕获变量
    // 如果将此闭包返回, 则引用也会跟着返回
    // 但是在整个函数调用完毕之后, 函数内的本地变量 i 就会被销毁
    // 那么随闭包返回的变量 i 的引用, 也将成为悬垂指针
    // Rust 是注重内存安全的语言, 绝对不会让这种事情发生
    // 所以如果不使用 move 关键字, 编译器会报错
    // 使用 move 关键字, 将捕获变量 i 的所有权转移到闭包中
    // 就不会按引用进行捕获变量, 这样闭包才可以安全地返回
    move |j| j * i
}
