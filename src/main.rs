#[derive(Debug)]
enum GenderCategory {
	Male,
	Female,
}

#[derive(Debug)]
struct Humans {
	name:String,
	gender:GenderCategory
	
}


fn main() {
	
    let h1= Humans {
		name:String::from("Adan"),
		gender:GenderCategory::Male
	};
	
	let h2= Humans {
		name:String::from("Eva"),
		gender:GenderCategory::Female
	};
	println!("{:?}",h1);
	println!("{:?}",h2);

}
