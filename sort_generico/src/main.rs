fn main() {
    let mut arrayi32 = [10, 9, 8, 7, 5 ,6, -1];
    let mut arrayf64 =[10.0, 9.9, 10.2, 5.3, 4.0, -2.0];

    let mut arrayi32_2 = arrayi32.clone();
    let mut arrayf64_2 = arrayf64.clone();

    bubble_sort(&mut arrayi32);
    bubble_sort(&mut arrayf64);
    println!("---Bubble sort---");
    println!("Array i32: {:?}", arrayi32);
    println!("Array f64: {:?}", arrayf64);

    selection_sort(&mut arrayf64_2);
    selection_sort(&mut arrayi32_2);
    println!("---Selection sort---");
    println!("Array i32: {:?}", arrayi32_2);
    println!("Array f64: {:?}", arrayf64_2);
}
fn bubble_sort<T: PartialOrd>(array: &mut [T]) {
    let mut trocado = false;
    for i in 0..array.len()-1 {
        for j in 0..array.len()-i-1 {
            if array[j] > array[j+1] {
                array.swap(j, j+1);
                trocado = true;
            }
        }
        if !trocado {
            break;
        }
    }
}
fn selection_sort<T: PartialOrd>(array: &mut [T]) {
    for i in 0..array.len()-1 {
        let mut min = i;
        for j in i+1..array.len() {
            if array[j] < array[min] {
                min = j;
            }
        }
        array.swap(i, min);
    }
}