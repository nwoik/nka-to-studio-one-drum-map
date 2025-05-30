#[derive(Debug)]
pub struct PitchPosition {
    name: String,
    position:Vec<i32>,
    color:String
}

pub fn create_pitch_position(name:String, position:Vec<i32>, color:String) -> PitchPosition {
    return PitchPosition { name: (name), position: (position), color: (color) }
}