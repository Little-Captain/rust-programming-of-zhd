fn main1() {
    use rust_programming_of_zhd::ch01::{fly_dyn, fly_static, title, Duck, Fly, Pig};
    title();
    let d = Duck;
    let p = Pig;
    println!("d fly: {}", d.fly());
    println!("p fly: {}", p.fly());

    println!("d static: {}", fly_static::<Duck>(d));
    println!("p dyn: {}", fly_dyn(&p));
}

fn main2_2() {
    use rust_programming_of_zhd::ch02::answer;
    answer();
}

fn main2_3() {
    use rust_programming_of_zhd::ch02::binding::temp;
    let x = &temp();
    println!("{}", x);
    // temp() = *x;
    // ^ error[E0070]: invalid left- hand side expression
}

fn main2_4() {
    use rust_programming_of_zhd::ch02::binding::immutable_and_mutable;
    immutable_and_mutable();
}

fn main2_5() {
    use rust_programming_of_zhd::ch02::binding::ownership;
    ownership();
}

fn main2_6() {
    use rust_programming_of_zhd::ch02::binding::reference;
    reference();
    use rust_programming_of_zhd::ch02::binding::reference2;
    reference2();
}

fn main2_8() {
    use rust_programming_of_zhd::ch02::function::fizz_buzz;
    assert_eq!(fizz_buzz(15), "fizzbuzz".to_string());
    assert_eq!(fizz_buzz(3), "fizz".to_string());
    assert_eq!(fizz_buzz(5), "buzz".to_string());
    assert_eq!(fizz_buzz(13), "13".to_string());
}

fn main2_9() {
    use rust_programming_of_zhd::ch02::function::lexical_scope;
    lexical_scope();
}

fn main2_10() {
    use rust_programming_of_zhd::ch02::function::{math, product, sum};
    let a = 2;
    let b = 3;
    println!("{} == {}", math(sum, a, b), 5);
    println!("{} == {}", math(product, a, b), 6);
}

fn main2_11() {
    use rust_programming_of_zhd::ch02::function::true_maker;
    println!("true_maker: {}", true_maker()());
}

fn main2_12() {
    use rust_programming_of_zhd::ch02::function::init_len;
    // 通过［O; N] 这种形式来初始化初始值为0、长度为N的数组
    // Rust 中固定长度的数组必须在编译期就知道长度, 否则会编译出错
    // 以函数 init_len 必须在编译期求值. 这就是 CTFE 的能力
    let arr = [0; init_len()];
    println!("{:?}", arr);
}

fn main2_13() {
    use rust_programming_of_zhd::ch02::function::closure;
    closure();
}

fn main2_14() {
    use rust_programming_of_zhd::ch02::function::closure_math;
    let a = 2;
    let b = 3;
    println!("{} == {}", closure_math(|| a + b), 5);
    println!("{} == {}", closure_math(|| a * b), 6);
}

fn main2_15() {
    use rust_programming_of_zhd::ch02::function::{two_times,two_times_dyn, two_times_impl};
    println!("{} == {}", two_times()(2), 4);
    println!("{} == {}", two_times_dyn()(2), 4);
    println!("{} == {}", two_times_impl()(2), 4);
}

fn main() {
    println!("------------------------");
    main1();

    println!("------------------------");
    main2_2();
    main2_3();
    main2_4();
    main2_5();
    main2_6();
    main2_8();
    main2_9();
    main2_10();
    main2_11();
    main2_12();
    main2_13();
    main2_14();
    main2_15();

    println!("------------------------");
}
