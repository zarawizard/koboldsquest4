struct Entity {
	x: i64,
	y: i64,

	leftimage : sdl2Image,
	rightimage : sdl2Image,
	upimage : sdl2Image,
	downimage : sdl2Image,
}

fn make_entity(x,y,limg,rimg,uimg,dimg)
{

	let ent = Entity {x,y,limg,rimg,uimg,dimg};
	return ent;

}

fn mod_entity_x(xx, &ent)
{
	ent.x = xx;
}

fn mod_entity_y(yy, &ent)
{
	ent.y = yy;
}

// put the modification methods of Entity struct in a table
// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html

// So, below, type Operations = mod_entity_funcs;
// let entity = Operations::mod_x etc

enum mod_entity_funcs {
     mod_x,
     mod_y,
}

impl mod_entity_funcs {
     fn run(&self, x: i64, y: i64) -> Entity {
     	match self {
	      Self::mod_x => mod_entity_x,
	      Self::mod_y => mod_entity_y,
	}
      }
}
