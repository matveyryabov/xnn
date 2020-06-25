# xnn
_What?_
An extensible neural network library I am working on with fellow uw 25s.
_Why?_ 
There is currently a hole in terms of native, GPU accelerated, neural network libraries in the rust ecosystem. I thought it might be a good way to farm github points (whatever they might be) and develop a better understanding of both rust lang and the focused on ml concepts.
_How?_
Ummm,,,, brain? The goal is to get a library somewhere even slightly close to deeplearning4j in terms of types of neural networks supported. User provides input and model configuration either through a gui interfacing with a .json file or through the cl. The provided input and model conf is used to build a model which then learns to accomplish a certain task. 
TODO: 
+ Have ConvLayers and DenseLayers structs but implement their methods using a trait object.
+ After above is figured out,,, implement back prop.
+ Implement ~4 of top optimization algos. 
+ Implement CNN, RNN, and fully connected models. After that the goal is deeplearning4j level of support - just where to aim, not necessarily reach.
+ Allow for GPU acceleration. Currently this will be done through arrayfire but it might be useful to move to opencl or some other more low level library.
+ Take advantage of rust's auto doc features to have good docs.
+ Far off goal: build web/desktop interface to allow for easy use - like weka. 

