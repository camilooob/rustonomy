
#[derive(Debug)]
enum GenderCategory {
	Male,
	Female,
}

#[derive(Debug)]
struct Humans {
	name:String,
	gender:GenderCategory,
	body:Physineeds,
	brainsafe: Safetyneeds,
	brainlove: Loveneeds,
	brainsocial: Esteemneeds,
	brainfly: Skyneeds,
}

#[derive(Debug)]
struct Physineeds {
	breath:u8,
	food:u8,
	hydration:u8,
	sleep:u8,
	pain:u8,
	temperature:u8,
	poop:u8,
	pee:u8,
	sex:u8,
	clean:u8,

}
#[derive(Debug)]
struct Safetyneeds {
	shelter:u8,
	salud:u8,
	seguridad:u8,

}
#[derive(Debug)]
struct Loveneeds {
	family:u8,
	friendship:u8,
	intimacy:u8,
	love:u8,
	social:u8

}
#[derive(Debug)]
struct Esteemneeds {
	prestige:u8,
	rol:String, //enum
	skill:String, //enum

}
#[derive(Debug)]
struct Skyneeds {
	goal:String,    // enum
	points:u8,     // Raking Humans

}


fn main() {
	
    let h1= Humans {
		name:String::from("Adan"),
		gender:GenderCategory::Male,
		body: Physineeds { 	breath:0,
			food:0,
			hydration:0,
			sleep:0,
			pain:0,
			temperature:0,
			poop:0,
			pee:0,
			sex:0,
			clean:0},
		brainsafe: Safetyneeds {shelter:0,
			salud:0,
			seguridad:0},
		brainlove: Loveneeds {	family:0,
			friendship:0,
			intimacy:0,
			love:0,
			social:0},
		brainsocial: Esteemneeds {	prestige:0,
			rol:String::from("None"), //enum
			skill:String::from("None")},
		brainfly: Skyneeds{	goal:String::from("None"),    // enum
			points:0, },
			};
	
	let h2= Humans {
		name:String::from("Eva"),
		gender:GenderCategory::Female,
		body: Physineeds { 	breath:0,
			food:0,
			hydration:0,
			sleep:0,
			pain:0,
			temperature:0,
			poop:0,
			pee:0,
			sex:0,
			clean:0},
		brainsafe: Safetyneeds {shelter:0,
			salud:0,
			seguridad:0},
		brainlove: Loveneeds {	family:0,
			friendship:0,
			intimacy:0,
			love:0,
			social:0},
		brainsocial: Esteemneeds {	prestige:0,
			rol:String::from("None"),
			skill:String::from("None")},
		brainfly: Skyneeds{goal:String::from("None"),
			points:0},
			};
	println!("{:?}",h1);
	println!("{:?}",h2);

}
