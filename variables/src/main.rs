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

    /*
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
    */

    /*
    let t = true;
    let f: bool = false; // 明示的型注釈
    */

    /*
    let c = 'z';
    let z = 'Ｚ';
    let heart_eyed_cat = '😻';
    */

    /*
    let tup: (i32, f64, u8) = (500, 6.3, 1);
    // let (x, y, z) = (500, 6.3, 1);
    // println!("The value of tup.2 is: {}", tup.2);
    let five_hundred = tup.0;
    let six_point_three = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_three is: {}", six_point_three);
    println!("The value of one is: {}", one);
    */

    let a = [1,2,3,4,5];
    /*
      let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
                  */
    /*
    let first = a[0];
    let second = a[1];
    println!("first and second: {} and {}", first, second);
    */
    let index = 10;
    let element = a[index];
    println!("The value of element is: {}", element);
}
