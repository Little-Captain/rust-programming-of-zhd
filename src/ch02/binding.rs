/// # 临时值
///
/// 位置表达式和值表达式
/// Rust 中的表达式一般可以分为位置表达式(Place Expression)和值表达式(Value Expression)
/// 在其他语言中, 一般叫作左值(LValue)和右值(RValue)
///
/// 位置表达式: 表示内存位置的表达式
/// 分类:
/// 1. 本地变量
/// 2. 静态变量
/// 3. 解引用(*expr)
/// 4. 数组索引(expr[expr])
/// 5. 字段引用(expr.field)
/// 6. 位置表达式组合
///
/// 通过位置表达式可以对某个数据单元的内存进行读写
/// 主要是进行写操作, 这是位置表达式可以被赋值的原因
///
/// 其他表达式都是值表达式
/// 值表达式一般只引用了某个存储单元地址中的数据
/// 它相当于数据值，只能进行读操作
///
/// 从语义角度来说, `位置表达式`代表了`持久性数据`, `值表达式`代表了`临时数据`
/// 位置表达式一般有持久的状态, 值表达式要么是字面量, 要么是表达式求值过程中创建的临时值
///
/// 表达式的求值过程在不同的上下文中会有不同的结果
/// 求值上下文也分为位置上下文(Place Context)和值上下文(Value Context)
///
/// 位置上下文
/// 分类:
/// 1. 赋值或者复合赋值语句左侧的操作数
/// 2. 一元引用表达式的独立操作数
/// 3. 包含隐式借用(引用)的操作数
/// 4. match 判别式或 let 绑定右侧在使用 ref 模式匹配的时候
///
/// 其余表达式都属于值上下文
/// 值表达式不能出现在位置上下文中
///
/// Basic usage:
///
/// ```
/// pub fn temp() -> i32 {
///     return 1;
/// }
/// let x = &temp();
/// temp() = *x;   // error[E0070]: invalid left-hand side expression
/// ```
pub fn temp() -> i32 {
    1
}

/// # 不变与可变
///
/// 通过 mut 关键字, 可以声明可变的位置表达式, 即可变绑定. 可变绑定可以正常修改和赋值
/// 从语义上来说, let 默认声明的不可变绑定只能对相应的存储单元进行读取,
/// 而 let mut 声明的可变绑定则是可以对相应的存储单元进行写入
///
/// Basic usage:
///
/// ```
/// pub fn immutable_and_mutable() {
///     let a = 1;  // 默认不可变
///     // a = 2; // immutable and error: cannot assign twice to immutable variable
///     let mut b = 2;  // 使用mut关键字声明可变绑定
///     b = 3; // mutable
/// }
/// immutable_and_mutable();
/// ```
pub fn immutable_and_mutable() {
    let a = 1;
    // a = 2;
    // ^ immutable and error
    let mut b = 2;
    b = 3;
    println!("{} {}", a, b);
}

/// # 所有权
/// 当位置表达式出现在值上下文中时, 该位置表达式将会把内存地址转移
/// 给另外一个位置表达式, 这其实是`所有权`的`转移`
///
/// 在语义上, 每个变量绑定实际上都拥有该存储单元的所有权,
/// 这种转移内存地址的行为就是所有权(OwnerShip)的转移,
/// 在 Rust 中称为移动(Move)语义, 那种不转移的情况实际上
/// 是一种复制(Copy)语义
///
/// Basic usage:
///
/// ```
/// pub fn ownership(){
///     let place1 = "hello";
///     //  ^^ 位置表达式 ^^  值表达式
///     //   ^ 位置上下文  ^  值上下文
///     let place2 = "hello".to_string();
///     let other = place1;    // Copy
///                  // ^^ 位置表达式出现在了值上下文中
///     println!("{:?}", place1);  // place1还可以继续使用
///     let other = place2;    // Move
///                  // ^^ 位置表达式出现在了值上下文中
///     println!("{:?}", place2); // place2不能再被使用，编译出错
/// }
/// ownership();
/// ```
pub fn ownership() {
    let place1 = "hello";
    let place2 = "hello".to_string();
    let other = place1;
    println!("{:?}", other);
    println!("{:?}", place1);
    let other = place2;
    println!("{:?}", other);
    // println!("{:?}", place2);
    // ^ Err : place2 value used here after move
}
/// # 引用
///
/// Rust 提供引用操作符(&), 可以直接获取表达式的存储单元地址, 即内存位置
/// 可以通过该内存位置对存储进行读取
///
/// Basic usage:
///
/// ```
/// pub fn reference() {
///     let a = [1,2,3];
///     let b = &a;
///     println!("{:p}", b);  // 0x7ffcbc067704
///     let mut c = vec![1,2,3];
///     let d = &mut c;
///     d.push(4);
///     println!("{:?}", d);
///     let e = &42;
///     assert_eq!(42, *e);
/// }
/// reference();
/// ```
pub fn reference() {
    let a = [1, 2, 3];
    let b = &a;
    println!("{:p} {:p}", &a, b); // 打印指针(内存地址)
                                  // 要获取可变引用, 必须先声明可变绑定
    let mut c = vec![1, 2, 3];
    let d = &mut c;
    d.push(4);
    println!("{:?}", d);
    let e = &42;
    assert_eq!(42, *e);
}

/// # 引用2
///
/// 从语义上来说, 不管是& a 还是&mut c, 都相当于对 a 和 c 所有权的借用,
/// 因为 a 和 c 还依旧保留它们的所有权, 所以`引用`也被称为`借用`
///
/// 值表达式在位置上下文中求值时会被创建临时值
/// println!("{:p}", &42);
pub fn reference2() {
    let mut _0: &i32;
    let mut _1: i32;
    _1 = 42i32;
    _0 = &_1;
    println!("{} {}", _0, _1);
    // 值表达式在位置上下文中求值时会被创建临时值
    println!("{:p}", &42);
}
