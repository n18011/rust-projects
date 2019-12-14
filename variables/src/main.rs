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
    /*
    let guess = "42".parse().expect("Not a number!");    // 数字ではありません！
    println!("guess is {}", guess);
    */
    /*
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
    */

    // 足し算
    let sum = 5 + 10;

    // 引き算
    let difference = 95.5 - 4.3;

    // 掛け算
    let product = 4 * 30;

    // 割り算
    let quotient = 56.7 / 32.2;

    // 余り
    let remainder = 43 % 5;
}
