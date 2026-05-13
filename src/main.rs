use rand::Rng;
use std::f32::consts;


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
		
		let mut current_layer = input;

		//making sure the first layer is also in the network
		self.layers.push(current_layer.clone());

		for con in self.cons.clone(){
		
			let next_layer = Layer::new(current_layer.clone(), con.clone());

			current_layer = next_layer.clone();

			self.layers.push(next_layer);
		
		}
}

	//lets forget about batching for now and make it work for just one example
	pub fn back_prop(&mut self,y : Vec<f32>){
		
		//------------output layer--------------		


		//lets handle the output layer to kick things off
		
		let output = self.layers.last().unwrap().activations.clone();
		
		//dL/da_n = a_n - y 
		let output_error: Vec<_> = output.iter().zip(y.iter()).map(|(x,y)| x - y).collect();
		
		//dL/z_n = a * (1 - a) <- this is the sigmoid derivative
		let output_derivative: Vec<_> = output.iter().map(|x| x * (1.0 - x)).collect();
	
		
		//dL/a_n * dL/z_n <- chain rule derivative that will propagate
		let delta_output : Vec<_> = output_error.iter().zip(output_derivative.iter()).map(|(x,y)| x * y).collect();
		
		//------------first hidden layer---------
		
		//now lets try and handle the first hidden layer aka a_n-1
		let hidden = self.layers[self.layers.len() - 2].activations.clone();
		
		//this is the sigmoid derivative for this layer and will help propagate the error further
		let hidden_derivative : Vec<_> = hidden.iter().map(|x| x * (1.0 - x)).collect();
		
		//now to get the gradient we need to transpose the weights
		let w_out = self.cons.last().unwrap().weights.clone();


		let mut w_out_t = vec![];


		for i in 0..w_out[0].len(){
			let mut row = vec![];
			for j in 0..w_out.len(){
				
				row.push(w_out[j][i]);

			}
			w_out_t.push(row);

		}


		//now the past layer gradient times the transposed matrix will be the current gradient
		let hidden_error = prod(w_out_t,delta_output.clone());
		
		let delta_hidden :Vec<_> = hidden_error.iter().zip(hidden_derivative.iter()).map(|(x,y)| x*y).collect();	



		//=========weight and bias gradient============
		//now this is the whole purpose of this function

		// dL/dW = delta_n x a_n-1
		let w_grad = prod_v2v(delta_output.clone(), hidden.clone());

		//as per chain rule the bias gradient is simply the delta from the current layer

		let bias_grad = delta_output.clone();
			
		
}
	
	
	}



fn main(){

	//I'ma to a quick run of the foward pass to see hows it lookin


	//Lets init the layer structure
	let layers :Vec<usize> = vec![3,2,1];

	let input_activations :Vec<f32> = vec![2.5,1.3,0.2];

	let input_layer = Layer::first(input_activations);
	
	let mut net = Network::new(layers);

	net.pass(input_layer);

	let modified = net.layers;

	//this does indeed work brotha
	for layer in modified{
		
		for element in layer.activations{

			print!("| {} | ",element);

		}

		println!(" ");


	}

	


}
