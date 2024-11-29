use itertools::Itertools;

fn main() {
    let dup_vetor = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9], vec![1,2,3]];

    let vetor = dup_vetor.into_iter()
        .flatten()
        .filter(|&x| x % 2 == 0 || x%3 == 0)
        .unique()
        .collect::<Vec<_>>();

    println!("{:?}", vetor);
}
