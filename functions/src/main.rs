 /*関数の動作法
  */
 fn main() {
     /*
    println!("Hello, world!");

    another_function();
    */

    // another_function(5);
    // another_function(5, 6);
    // let y = 6; // これは 文
    // let x = (let y = 6); // error:letは文なため値を返さない.よって変数に代入できない。
    /*
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {}", y);
    */
     /*
    let x = five();
    println!("The value of x is {}", x);
    */
    let x = plus_one(5);
    println!("The value of x is {}", x);

}

/*
fn plus_one(x:i32) ->i32 {
    x + 1
}
*/
fn plus_one(x:i32) ->i32 {
    x + 1;
}

fn five() -> i32 {
    5
}

/*
fn another_function() {
    println!("Another function.");  // 別の関数
}
*/
/*
fn another_function(x:i32) {
    println!("the value of x is {}", x);
}
*/
/*
fn another_function(x: i32, y: i32) {
    println!("the value of x is {}", x);
    println!("the value of y is {}", y);
}
*/
