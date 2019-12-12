# 変数と可変性


- rustは変数は標準で不変(推奨している)
    これは安全性や並行性の利点を享受するため


- 可変にもできる


```rust:src/main.rc
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);     // xの値は{}です
    x = 6;
    println!("The value of x is: {}", x);
}
```


`cargo run`を実行するとコンパイルエラー


エラー原因は不変変数xに2回代入できない


値が変わらないという前提のもと処理を行っているものがあると、別のタイミングでその値が変更されると最初の処理が思惑通りに動かない可能性があるため。



可変にしたい時は`mut`を変数宣言時につける。
`mut`宣言することでコードの別の部分が値を変える可能性があることを示す。


こうすると
```rust:src/main.rc
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

```
結果は
```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
     Running `target/debug/variables`
The value of x is: 5   (xの値は5です)
The value of x is: 6

```


- 大きなデータ構造を扱う場合
    可変にして変更できるようにする方が、いちいちインスタンスをコピーして新しくメモリ割り当てされたインスタンスを返すよりも速くなる
- 小さなデータ構造を扱う場合
    新規インスタンスを生成して、もっと関数型っぽいコードを書く方が通して考えやすくなる


## 変数と定数(constants)の違い
- 定数にはmutキーワードは使えない
    - 定数は標準で不変であるだけでなく、常に不変。
    - letの代わりに、constで宣言し、値の型は必ず注釈しなければならない。
    - 定数はどんなスコープでも定義できる。
- 定数は定数式にしかセットできない
    関数呼び出し結果や、実行時に評価される値にはセットできない。


定数の命名規則は、 全て大文字でアンダースコアで単語区切りする
```rust

#![allow(unused_variables)]
fn main() {
const MAX_POINTS: u32 = 100_000;
}
```
定義されたスコープ内でずっと有効。プログラムのいろんなところで使用される可能性のあるアプリケーション空間の値を定義するのに有益な選択肢


## シャドーイング
前に定義した変数と同じ名前の変数を新しく宣言でき、 新しい変数は、前の変数を覆い隠す。
letキーワードの使用を繰り返す


```rust:src/main.rc
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

```
let x =を繰り返すことでxを覆い隠し、xの最終的な値は12になる。
```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/variables`
The value of x is: 12

```


変数をmutにするのとは違う。
letを使うことで、 値にちょっとした加工は行えるが、その加工が終わったら、変数は不変になる


実効的には新しい変数を生成していることになるため、 値の型を変えつつ、同じ変数名を使いまわせる
```rust:src/main.rc

#![allow(unused_variables)]
fn main() {
let spaces = "   ";
let spaces = spaces.len();
}
```
最初のspaces変数は文字列型であり、2番目のspaces変数は、 最初の変数と同じ名前になった変数だが、数値型になる。このため、spacesという名前を再利用できる


これはエラー
```rust:src/main.rc
let mut spaces = "   ";
spaces = spaces.len();
```
```
error[E0308]: mismatched types          (型が合いません)
 --> src/main.rs:3:14
  |
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected &str, found usize
  |                           (&str型を予期しましたが、usizeが見つかりました)
  |
  = note: expected type `&str`
             found type `usize`

```
変数の型を可変にすることは許されていない
