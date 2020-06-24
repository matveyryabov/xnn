mod arch;
mod math;

extern crate arrayfire as af;
//Read the article on rustdoc
use af::*;


fn main() {
    println!("Create a 5-by-3 matrix of random floats on the GPU");
    //print(&a);
    //Find out why I need to set the constructor to be private/public and also find out whether struct shoudl be private (would be a good idea to look in book).
    //Goal: make the variables such as nodes_in_each_layer defined through a .json file or something like that. This would allow for a more seamless front end.
    let nodes_in_each_layer: Vec<u64> = vec![3, 5, 3];
    let test_array: Array<f32> = randn::<f32>(Dim4::new(&[4, 4, 1, 1])) * 3;
    print(&test_array);
    let test_array2: Array<f32> = math::sigmoid(test_array);
    print(&test_array2);

    let test = arch::DenseLayers::new(nodes_in_each_layer, 0.3);
}

//emu
//ndarray
//use cargo doc --open to open the pages after compilation
