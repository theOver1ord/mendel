fn main() {
    let mut pop = String::new();
    let k = 3;
    let m = 2;
    let n = 1;
    for i in 0..k {
        pop.push('k');
    }
    for i in 0..m {
        pop.push('m');
    }
    for i in 0..n {
        pop.push('n');
    }
    println!("pop: {}", &pop[0..k+m]);
}