fn main() {
    format2();
    let a = Empty {};
    println!("{:?}", a)
}

fn format1() {
    let ty = "23".to_string();
    let c = &format!(
        "const parameter `{}` is part of concrete type but not \
                      used in parameter list for the `impl Trait` type alias",
        ty
    );
    println!("{c}")
}

fn format2() {
    let str1 = "hand code str";
    let smart1 = str1.to_string();
    let str1 = "wq";
    let unsmart1 = &smart1;
    let out2 = format!("format2 : {}", smart1);
    let out = format!("format2 : {}", &str1);

    println!("{}\n,{}\n,{}\n,{}\n", out, out2, unsmart1, smart1,);
}

#[derive(Debug)]
struct Empty {}
