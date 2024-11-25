use std::io::stdin;

fn main() {
    let mut lim= String::new();

    println!("Escolhe limite da sequência a gerar:");
    stdin().read_line(&mut lim).unwrap();

    let lim:u32 = lim.trim().parse().expect("Escolhe inteiro positivo");

    println!("Sequência: {:?}", fibonacci(lim));

    //Por recursão:
    print!("Sequência: ");
    fibonacci_rec(lim,1,0,0);
}

fn fibonacci(lim:u32) -> Vec<u32> {
    if lim==0{
        return vec![];
    }
    let mut v:Vec<u32> = vec![1];
    let mut n=1;
    let mut i=1;
    while i<lim{
        v.push(n);
        n=n+&v[v.len()-2];
        i+=1;
    }
    return v;
}

fn fibonacci_rec(lim:u32, n:u32, n_antigo:u32, i:u32) {
    if i<lim{
        print!("{n} ");
        fibonacci_rec(lim, n+n_antigo, n, i+1);
    }else{
        return;
    }
}
