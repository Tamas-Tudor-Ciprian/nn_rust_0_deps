use rand::Rng;


struct Layer{
	activations: Vec<f32>,
}


impl Layer {
	fn first(input_data: Vec<f32>) -> Self{
		Self{
			activations : input_data,

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
		(0..nd_layer_len).map(|_| rng.get::<f64>() * 4.0 - 2.0)//random in (-2,2)
		.collect()}).collect();
		
		let mut biases = 
		(0..nd_later_len).map(|_| rng.get::<f64>() * 4.0 - 2.0) //hope the biases don't have too big a range
		.collect();

	Self {
		weights: weights,


	}
}



}



struct Network{}


fn main(){

	println!("A rusty begginging");



}
