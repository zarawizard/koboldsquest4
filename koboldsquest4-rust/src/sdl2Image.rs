struct sdl2Image {
	
	image : null, 
	x : 0,
	y : 0,
	w : 0,
	h : 0,
	len : 0,
	
}

fn make_sdl2Image(sdlimg,x,y,w,h,len)
{

	let imag = sdl2Image {sdlimg,x,y,w,h,len};
	return imag;

}

fn mod_sdl2Image_x(xx, &img)
{
	img.x = xx;
	return img;
}

fn mod_sdl2Image_y(yy, &img)
{
	img.y = yy;
	return img;
}

// put the modification methods of sdl2Image struct in a table
// https://doc.rust-lang.org/rust-by-example/custom_types/enum.html

// So, below, type Operations = mod_sdl2Image_funcs;
// let sdl2img = Operations::mod_x etc

enum mod_sdl2Image_funcs {
     mod_x,
     mod_y,
}

impl mod_sd2Image_funcs {
     fn run(&self, x: i64, y: i64) -> sdl2Image {
     	match self {
	      Self::mod_x => mod_sdl2Image_x,
	      Self::mod_y => mod_sdl2Image_y,
	}
      }
}
