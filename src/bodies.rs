use macroquad::prelude::*;
pub struct Body { pub name: &'static str, pub position: Vec3, pub radius: f32, pub speed: f32 }
pub fn catalog() -> [Body;6] {
    [
        Body{name:"HELIOS",position:vec3(-7.,1.8,0.),radius:1.65,speed:0.12},
        Body{name:"IGNIS",position:vec3(-2.6,1.8,0.),radius:1.,speed:0.23},
        Body{name:"AURELION",position:vec3(3.1,1.8,0.),radius:1.35,speed:0.16},
        Body{name:"CRYON",position:vec3(-6.,-2.3,0.),radius:1.1,speed:0.19},
        Body{name:"NYX",position:vec3(-1.,-2.3,0.),radius:1.15,speed:0.14},
        Body{name:"VERDANTIS",position:vec3(4.7,-2.3,0.),radius:1.2,speed:0.21},
    ]
}
pub fn moon_position(parent: Vec3, time:f32)->Vec3 {
    parent+vec3((time*0.45).cos()*1.75,0.25,(time*0.45).sin()*1.75)
}
