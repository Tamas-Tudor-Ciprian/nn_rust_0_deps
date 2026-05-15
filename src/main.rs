use rand::Rng;
use std::f32::consts;
use plotters::prelude::*;

fn transpose_matrix(matrix: Vec<Vec<f32>>) -> Vec<Vec<f32>>{

	let mut matrix_T = vec![];

	for i in 0..matrix[0].len(){
		let mut row = vec![];
		for j in 0..matrix.len(){
			row.push(matrix[j][i]);
		}
		matrix_T.push(row);
	}
	matrix_T
}


fn sigmoid(input: f32) -> f32{

	1.0/(1.0 + (-input).exp())


}



fn prod(weights: Vec<Vec<f32>>, input: Vec<f32>) -> Vec<f32>{

	let mut output= vec![];	


	for i in 0..weights.len() {
		let mut sum :f32 = 0.0;
		for j in 0..weights[0].len(){	

			sum += weights[i][j] * input[j];

		}
		output.push(sum);
		}

	output

}

fn prod_v2v(gradient: Vec<f32>, input: Vec<f32>) -> Vec<Vec<f32>>{


	let mut weights_gradient = vec![];

	for i in 0..gradient.len(){
		
		let mut row = vec![];
		
		for j in 0..input.len(){
		
				row.push(input[j] * gradient[i]);
		
		}

		weights_gradient.push(row);

	}

	weights_gradient

}


fn add(input: Vec<f32>,bias: Vec<f32>) -> Vec<f32>{

	let mut output = vec![];

	for i in 0..input.len(){

		output.push(input[i] + bias[i]);
	}
	output
}


fn apply_lr(gradients: Vec<Connections>,lr : f32){

	for mut gradient in gradients{
		for i in 0..gradient.weights.len(){
			gradient.biases[i] *= lr;
			for j in 0..gradient.weights[0].len(){

				gradient.weights[i][j] *= lr;

			}

		}
	}



}

//this is so you can avergage the gradient batch
fn avg(gradient_batch: Vec<Vec<Connections>>)->Vec<Connections>{

	//we divide the sum of all values by this
	let n = gradient_batch.len();



	//now for the complicated part, we declare an copy of the first gradient and sum everything in it
	let mut sum = gradient_batch[0].clone();
	
	//now we iterate from the second and sum
	for i in 1..gradient_batch.len(){
		let current_grad = gradient_batch[i].clone();

		//now we go trough each connections element
		for j in 0..current_grad.len(){
			let current_w = current_grad[j].weights.clone();
			let current_b = current_grad[j].biases.clone();
			//the actual summing algo starts here
			for k in 0..current_w.len(){
				sum[j].biases[k] +=current_grad[j].biases[k];
				for l in 0..current_w[0].len(){
					sum[j].weights[k][l] += current_grad[j].weights[k][l];
				}
			}


		}



	}

	let to_apply = 1.0/(n as f32);

	for mut gradient in sum.iter_mut(){
		for i in 0..gradient.weights.len(){
			gradient.biases[i] *= to_apply;
			for j in 0..gradient.weights[0].len(){

				gradient.weights[i][j] *= to_apply;

			}

		}
	}

	sum


}


#[derive(Clone)]
struct Layer{
	activations: Vec<f32>,
}


impl Layer {
	fn first(input_data: Vec<f32>) -> Self{
		Self{
			activations : input_data,

		}
	}
	
	fn new(input: Layer, cons: Connections ) -> Self{
		let non_biased = prod(cons.weights, input.activations);
		let biased = add(non_biased,cons.biases);
		let squished = biased.iter().map(|x| sigmoid(*x)).collect();
		Self{
			activations: squished,
	
		}	
	}
}


#[derive(Clone)]
struct Connections{
	weights: Vec<Vec<f32>>,
	biases: Vec<f32>,
}




impl Connections{

	fn new(st_layer_len: usize,nd_layer_len: usize) -> Self {
		let mut rng = rand::thread_rng();
		
		let mut weights = 
		(0..nd_layer_len).map(|_| {
		(0..st_layer_len).map(|_| rng.gen::<f32>() * 4.0 - 2.0)//random in (-2,2)
		.collect()}).collect();
		
		let mut biases = 
		(0..nd_layer_len).map(|_| rng.gen::<f32>() * 4.0 - 2.0) //hope the biases don't have too big a range
		.collect();

		Self {
			weights: weights,
			biases: biases,
			}

		}

	fn from_con(w: Vec<Vec<f32>>, b: Vec<f32>)-> Self{
		Self{
			weights: w,
			biases: b,
		}
	}



	fn empty(st_layer_len: usize,nd_layer_len: usize) -> Self {
		let mut rng = rand::thread_rng();
		
		let mut weights = vec![vec![0.0;st_layer_len];nd_layer_len];
		
		let mut biases = vec![0.0;nd_layer_len];

		Self {
			weights: weights,
			biases: biases,
		}
	
	}
	


}



struct Network{
	layers: Vec<Layer>,
	cons: Vec<Connections>
}



impl Network{
	fn new(layers: Vec<usize>) -> Self{


		let mut cons = vec![];

		for i in 1..layers.len(){
		
			let layer_con = Connections::new(layers[i-1], layers[i]);
			cons.push(layer_con);
		}

		let empty_vec = vec![];
		

		Self{
			
			cons: cons,
			layers:empty_vec,

		}

	}

	/*
	it is to be mentioned that the first layer is to be the same size as the already declared
	input of this network
	*/
	pub fn pass(&mut self,input : Layer){
	
		self.layers.clear();
	
		let mut current_layer = input;

		//making sure the first layer is also in the network
		self.layers.push(current_layer.clone());

		for con in self.cons.clone(){
		
			let next_layer = Layer::new(current_layer.clone(), con.clone());

			current_layer = next_layer.clone();

			self.layers.push(next_layer);
		
		}
	}

	pub fn backprop_gradient(&mut self,y : Vec<f32>)->Vec<Connections>{
		

		let mut gradients: Vec<Connections> = vec![];


		//------------output layer--------------		


		//lets handle the output layer to kick things off
		
		let output = self.layers.last().unwrap().activations.clone();
		
		//dL/da_n = a_n - y 
		let output_error: Vec<_> = output.iter().zip(y.iter()).map(|(x,y)| x - y).collect();
		
		//dL/z_n = a * (1 - a) <- this is the sigmoid derivative
		let output_derivative: Vec<_> = output.iter().map(|x| x * (1.0 - x)).collect();
	
		//dL/a_n * dL/z_n <- chain rule derivative that will propagate
		let delta_output : Vec<_> = output_error.iter().zip(output_derivative.iter()).map(|(x,y)| x * y).collect();
	

		let mut last_delta = delta_output.clone();
	
		//=======================================layers loop=================================			
		for i in 2..=self.layers.len(){
			//------------hidden layer---------
		
			//now lets try and handle the first hidden layer aka a_n-1
			let hidden = self.layers[self.layers.len() - i].activations.clone();
		
			//this is the sigmoid derivative for this layer and will help propagate the error further
			let hidden_derivative : Vec<_> = hidden.iter().map(|x| x * (1.0 - x)).collect();
		
			//now to get the gradient we need to transpose the weights
			let w_out = self.cons[self.cons.len() - i + 1].weights.clone();


			let mut w_out_T = transpose_matrix(w_out);

			//now the past layer gradient times the transposed matrix will be the current gradient
			let hidden_error = prod(w_out_T,last_delta.clone());
		
			let delta_hidden :Vec<_> = hidden_error.iter().zip(hidden_derivative.iter()).map(|(x,y)| x*y).collect();	


			//=========weight and bias gradient============
			//now this is the whole purpose of this function

			// dL/dW = delta_n x a_n-1
			let w_grad = prod_v2v(last_delta.clone(), hidden.clone());

			//as per chain rule the bias gradient is simply the delta from the current layer

			let bias_grad = last_delta.clone();

			gradients.insert(0,Connections::from_con(w_grad,bias_grad));

			last_delta = delta_hidden;

		}


		gradients
			
		
	}

	//once you got the averaged gradient over a batch you can apply it with this
	pub fn apply_gradient(&mut self , grad: Vec<Connections>){
	
		let mut network = &mut self.cons;

		for  k in 0..grad.len(){

			let Connections { weights: w, biases: b } = &mut network[k];

			let w_grad = &grad[k].weights;
			let b_grad = &grad[k].biases;
			for i in 0..w.len(){
				b[i] -= b_grad[i];
				for j in 0..w[0].len(){

					w[i][j] -= w_grad[i][j];

				}

			}
		}

	}
	
}



fn main(){


	let mut rng = rand::thread_rng();

	//Lets init the layer structure
	let layers :Vec<usize> = vec![3,2,1];

	let input_activations :Vec<f32> = vec![2.5,1.3,0.2];

	let input_layer = Layer::first(input_activations);
	
	let mut net = Network::new(layers);

	net.pass(input_layer);

	/*think I will initially teach it to aproximate a sin to see if it can actually learn
	*/
	//will just use a lib for plotting because I don't have the patience to draw graphs in the cli myself

	//lets experiment

	let root = BitMapBackend::new("./training/data.png",(800,600)).into_drawing_area();
	root.fill(&WHITE).unwrap();

	let mut chart = ChartBuilder::on(&root)
	.caption("f(x) = sin(10x) + 1",("sans-serif",40))
	.margin(20)
	.set_all_label_area_size(40)
	.build_cartesian_2d(-10f32..10f32, 0f32..2f32).unwrap();
	
	chart.configure_mesh().draw();

	chart.draw_series(LineSeries::new(
		(-100..100).map(|x| x as f32 / 10.0).map(|x| (x,x .sin() + 1.0)),
		&RED,)).unwrap();

	//sin is a 2D func so 1 input 1 output
	let layers :Vec<usize> = vec![1,10,10,1];

	//this we will train
	let mut net = Network::new(layers);

	//lets do 1000 training iterations in batches of 10

	for _ in 0..100 {


		//so 10 random inputs in between -10 and 10
		
		let inputs : Vec<_> = 
		(0..20).map(|_| rng.gen::<f32>() * 20.0 - 10.0)
		.collect();

		let outputs : Vec<_> = inputs.iter().map( |x| x.sin() + 1.0).collect();


		//now that we got a batch of data we gotta get the gadients for them

		let mut gradients = vec![];

		for i in 0..inputs.len(){
			
			let input_activation = vec![inputs[i]];

			let input_layer = Layer::first(input_activation);

			net.pass(input_layer);
			
			let gradient = net.backprop_gradient(vec![outputs[i]]);


			gradients.push(gradient);
		}



	}
}
