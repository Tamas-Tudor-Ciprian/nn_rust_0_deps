use rand::Rng;


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






fn add(input: Vec<f32>,bias: Vec<f32>) -> Vec<f32>{

	let mut output = vec![];


	for i in 0..input.len(){

		output.push(input[i] + bias[0]);

}



	output



}






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

	Self{
		activations: vec![],//placeholder for now
	
	}	



}


}





struct Connections{
	weights: Vec<Vec<f32>>,
	biases: Vec<f32>,
}




impl Connections{

	fn new(st_layer_len: usize,nd_layer_len: usize) -> Self {
		let mut rng = rand::thread_rng();
		
		let mut weights = 
		(0..st_layer_len).map(|_| {
		(0..nd_layer_len).map(|_| rng.gen::<f32>() * 4.0 - 2.0)//random in (-2,2)
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



struct Network{}


fn main(){

	println!("A rusty begginging");



}
