use std::io;

fn main() {
   // let mut Za = 3355;
    let Zb = 2379;
    let mut amari: [i32; 1000] = [0; 1000];
    let mut kakeru: [i32; 1000] = [0; 1000];
    let mut kotae =0; 
    
    println!("最大公約数を求める整数を入力してください");

    // 数値の入力 https://magidropack.hatenablog.com/entry/2018/12/19/131919
    let mut number = String::new();
    io::stdin().read_line(&mut number).ok();
    let Za = number.trim().parse().ok().unwrap();
    match Za {
        _ => println!("{}を入力しました。",Za),
    }
                                        // end of 数値の入力
    
    kakeru[0] = Za;
    amari[0] = Zb;
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
