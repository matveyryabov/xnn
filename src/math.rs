//This is the file responsible for the matrix math and such
//use std::convert::TryInto;
extern crate arrayfire as af;
use af::*;

//TO DO
// Implement or figure out how to do with array fire:
// EW addition, EW multiplication, transpose array, scale array, dot product, convolution, max pooling, global pooling

//maybe rename method feed_forward to something like output and have it instead have the feedforward function here. Although memory safety might be dealt with better if its there
pub fn sigmoid(input_array: Array<f32>) -> Array<f32> {
    let output_array = 1 / (1 + (1/exp(&input_array)));
    output_array
}

//pub fn dsigmoid(input_array: Array<f32>) -> Array<f32> {

    
//}
