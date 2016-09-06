fn say_hello(){
	println!("HelloWorld");
}

fn add(a: i32,b:i32)-> i32 {
	return a+b;

}

fn main(){
	let c = add(1,2);
	say_hello();
}