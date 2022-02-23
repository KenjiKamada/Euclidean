use std::io;
use rand::Rng;

fn main() {
   // let mut z_a = 3355;
  
    let z_b = rand::thread_rng().gen_range(1, 1000);
//    let z_b = 2379;
    let mut amari: [i32; 1000] = [0; 1000];
    let mut kakeru: [i32; 1000] = [0; 1000];
    let mut kotae =0; 
    
    println!("{}との最大公約数を求めます。",z_b);

    println!("最大公約数を求める整数を入力してください");

    // 数値の入力 https://magidropack.hatenablog.com/entry/2018/12/19/131919
    let mut number = String::new();
    io::stdin().read_line(&mut number).ok();
    let z_a = number.trim().parse().ok().unwrap();
    match z_a {
        _ => println!("{}を入力しました。",z_a),
    }
                                        // end of 数値の入力
    println!("{}と{}の最大公約数を求めます。",z_a,z_b);
    
    // 大きい方をkakeruに入れる。
    if z_a > z_b{
        kakeru[0] = z_a;
        amari[0] = z_b;
    }
    else{
        kakeru[0] = z_b;
        amari[0] = z_a;
    }

    
    for n in 0..=1000 {
        amari[n+1] = kakeru[n] % amari[n];
        println!("余り={}",amari[n+1]);

        // 余りが無ければ止める。
        if amari[n+1]==0{
            kotae = amari[n];
            break;
        }
        else{
            kakeru[n+1] = amari[n];
        }

    }

    println!("最大公約数は...答え = {}",kotae);
}
