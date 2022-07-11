use rand::Rng;
#[allow(dead_code)]
fn print_array(array: [i32; 10]) {
    for elem in array {
        print!(" {elem} ");
    }
    println!();
}
fn create_random_array() -> [i32; 10] {
    let mut array: [i32; 10] = [0; 10];
    let mut rng = rand::thread_rng();
    for elem in array.iter_mut() {
        *elem = rng.gen_range(1..=100);
    }
    array
}
fn bubble_sort(array: &mut [i32; 10]) {
    for j in 0..array.len() - 1 {
        for i in 0..array.len() - j - 1 {
            let first_elem = array[i];
            let second_elem = array[i + 1];
            if first_elem > second_elem {
                array[i] = second_elem;
                array[i + 1] = first_elem;
            }
        }
    }
}
fn insert_sort(array: &mut [i32; 10]) {
    let mut insertion_vector: Vec<i32> = Vec::new();
    for elem in array.iter_mut() {
        insertion_vector.push(*elem);
        let mut i: usize = insertion_vector.len() - 1; // zwraca ilosc elementow
        while i > 0 {
            if insertion_vector[i - 1] > insertion_vector[i] {
                insertion_vector.swap(i, i - 1);
            }
            i -= 1;
        }

    }
    for i in 0..array.len()
    {
        array[i].clone_from(&insertion_vector[i]);
    }

}

fn main() {
    let mut random_array = create_random_array();
    let mut random_array_bubble_sort = random_array.clone();
    let mut random_array_insert_sort = random_array.clone();
    random_array.sort();
    {
        bubble_sort(&mut random_array_bubble_sort);
        let is_same = random_array == random_array_bubble_sort;
        assert_eq!(true, is_same, "BUBBLE SORT NOT WORKING");
    }
    {
        insert_sort(&mut random_array_insert_sort);
        let is_same = random_array == random_array_insert_sort;
        assert_eq!(true, is_same, "INSERT SORT NOT WORKING");
    }
}
