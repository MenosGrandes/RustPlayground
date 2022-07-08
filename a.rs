use rand::Rng;
const END_VALUE: i32= 10000;
fn create_random_array() -> [i32; 10] {
    let mut array: [i32; 10] = [0; 10];
    let mut rng = rand::thread_rng();
    for x in &mut array {
        let secret_number = rng.gen_range(1..= END_VALUE);
        *x = secret_number;
    }
    return array;
}
fn create_random_array_from_array(array: &mut [i32; 10]) {
    let mut rng = rand::thread_rng();
    for x in array {
        let secret_number = rng.gen_range(1..= END_VALUE);
        *x = secret_number;
    }
}
fn print_array(array: [i32; 10]) {
    for x in array {
        print!("{x} ");
    }
    println!("");
}
fn sort(array: &mut [i32; 10]) {
    for _x in 0..9 {
        let mut changed: bool = false;
        let end_point = 9 - _x;
        for i in 0..end_point {
            let first = array[i];
            let second = array[i + 1];
            if first > second {
                changed = true;
                array[i] = second;
                array[i + 1] = first;
            }
        }
        if changed == false {
            break;
        }
        print_array(*array);
    }
}

fn main() {
    let mut arr: [i32; 10] = [0; 10];
    create_random_array_from_array(&mut arr);
    print_array(arr);
    let mut arr_2 = create_random_array();
    print_array(arr_2);
    let the_same: bool = arr == arr;
    print!("Arrays are {the_same}");
    sort(&mut arr_2);
    println!("SORTED");
    print_array(arr_2);
}
