/* 変数と可変性
fn main() {
    /*
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
    */
    let mut spaces = "   ";
    spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}
*/

/* データ型
 */
fn main (){
    let guess: u32 = "42".parse().expect("Not a number!");    // 数字ではありません！
    println!("guess is {}", guess);
}
