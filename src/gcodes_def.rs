#[derive(Debug)]
pub struct Machine {
    pub move_type: Option<GCode>,
    pub pos: Coord,
    pub def_speed: f32,
    pub speed: Option<f32>,
    pub unit: Option<Unit>,
    pub reference: Option<Referential>,

}
impl Machine {
    pub fn new(def_speed: f32) -> Machine {
        Machine {
            move_type: None,
            pos: Coord {
                x: Some(0.0),
                y: Some(0.0),
                z: Some(0.0),
                i: None,
                j: None,
            },
            def_speed,
            speed: None,
            unit: None,
            reference: None,
        }
    }
}



#[derive(Debug)]
pub struct ModalGroup {
    pub move_type: &Option<GCode>,
    pub origin: &Coord,
    pub dest: &Option<Coord>,
    pub speed: &Option<f32>,
    pub def_speed: &f32,
    pub unit: &Option<Unit>,
    pub reference: &Option<Referential>,
}
//impl ModalGroup {
//    pub fn new() -> ModalGroup {
//        ModalGroup {
//            move_type: None,
//            origin: None,
//            dest: None,
//            speed: None,
//            def_speed: None,
//            unit: None,
//        }
//    }
//}


#[derive(Debug)]
pub struct Coord {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
    pub i: Option<f32>,
    pub j: Option<f32>,

}
impl Coord {
    pub fn new() -> Coord {
        Coord {
            x: None,
            y: None,
            z: None,
            i: None,
            j: None,
        }
    }
}



#[derive(Debug)]
pub enum GCode {
    MT(MoveTypes),
    Flag(Flags),
    Dump,
}

#[derive(Debug)]
pub enum MoveTypes {
    G0,
    G1,
    G2,
    G3,
}

#[derive(Debug)]
pub enum Flags {
    G20,
    G21,
    G90,
    G91,
}

#[derive(Debug)]
pub enum Unit {
    MM,
    Inch,
}

#[derive(Debug)]
pub enum Referential {
    Absolute,
    Increment,
}
