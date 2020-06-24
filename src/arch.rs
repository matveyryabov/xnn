//The file with the architecture
//Read the structure article in Rust documentation to figure out how the documentation for each file should be written

//fix this
use std;

mod math;

//use std::convert::TryInto;
extern crate arrayfire as af;
use af::*;

//make a struct called model/nn/something and make it be overarching over dense layers and convlayers
// If not handled by model struct, make a output layer struct
//Research and find out whether having structs inside structs is better than having everything inside one struct so to speak
pub struct DenseLayers {
    nodes_in_each_layer: Vec<u64>,
    lr: f32,
    weights: Vec<Array<f32>>,
    biases: Vec<Array<f32>>,
    //Maybe in the future if I implement diff optimization algos have option for that. Also weight/biases initialization method selection could also go here.

}

struct ConvLayers {
    num_filters: Vec<u64>,
    //When having many declarations of the same type, isn't there a way to shorten?
    vertical_stride: u32,
    horizonal_stride: u32,
    kernel_width: u32,
    kernel_height: u32, 
}

//find out whether its possible to somehow group constructors? I mean c has header files right? orrrrr maybe it's possible to put constructors inside the overarching struct
impl ConvLayers {
    //fn new() -> ConvLayers {
      //  ConvLayers{}
    //}
    fn pool() {

    }
    fn conv() {

    }
}

impl DenseLayers {
    pub fn new(nodes_in_each_layer: Vec<u64>, lr: f32) -> DenseLayers {
        //Initialize weights
        let num_layers = nodes_in_each_layer.len();
        let mut weights = vec![];
        for i in 0..num_layers-1 {
            let weights_dims = Dim4::new(&[nodes_in_each_layer[i], nodes_in_each_layer[i+1], 1, 1]);
            //If the non arrayfire vector is made up of arrays which are stored on the GPU, doesn't it just act as a pointer of sorts?
            weights.push(randu::<f32>(weights_dims)); 
        }

        //Initialize biases
        let mut biases = vec![];
        for i in 0..num_layers-1 {
            let biases_dims = Dim4::new(&[nodes_in_each_layer[i], 1, 1, 1]);
            //See in weight initialization for loop for my concern
            biases.push(randu::<f32>(biases_dims));
        }

        DenseLayers{nodes_in_each_layer, weights, biases, lr}
    }

    fn train() {

    }

    fn feed_forward(layer_input: Array<f32>, layer_weights: Array<f32>, layer_biases: Array<f32>) -> Array<f32> {
        let mut layer_output: Array<f32> = matmul(&layer_weights, &layer_input, MatProp::NONE, MatProp::NONE);
		    layer_output = add(&layer_input, &layer_biases, false);
        layer_output = sigmoid(&layer_output);
        layer_output
    }

    fn back_prop(layer_output: Array<f32>, layer_target: Array<f32>, layer_weights: Array<f32>, layer_biases: Array<f32>) {

    }
}

