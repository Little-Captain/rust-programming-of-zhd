fn main1() {
    use rust_programming_of_zhd::ch01::{title, Duck, Pig, Fly, fly_static, fly_dyn};

    title();

    let d = Duck;
    let p = Pig;
    println!("d fly: {}", d.fly());
    println!("p fly: {}", p.fly());

    println!("d static: {}", fly_static::<Duck>(d));
    println!("p dyn: {}", fly_dyn(&p));
}

fn main() {
    // main1();
}