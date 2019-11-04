//! 第二章：语言精要
//!

/// # Examples
///
/// Basic usage:
///
/// ```
/// pub fn title(){
///   println!("第2章：{}", "语言精要");
/// }
/// title();
/// ```
pub fn title() {
    println!("第2章: {}", "语言精要");
}

/// # answer
///
/// Basic usage:
///
/// 一切皆表达式
/// 表达式: 求值表达式, 返回`求值结果`
/// 语句: 带分号的特殊表达式, 返回`()`
/// ```
/// // extern crate std;      // 声明语句
/// // use std::prelude::v1::*;  // 声明语句
///
/// pub fn answer() -> (){
///     let a = 40;  // 声明语句
///     let b = 2;   // 声明语句
///     assert_eq!(sum(a, b), 42); // 宏语句
/// }
///
/// pub fn sum(a: i32, b: i32) -> i32 {
///     a + b  // 表达式
/// }
///
/// answer(); // 表达式语句
/// ```
pub fn answer() -> () {
    let a = 40;
    let b = 2;
    assert_eq!(sum(a, b), 42)
}

pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub mod binding;
pub mod control_flow;
pub mod function;
