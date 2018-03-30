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
pub struct ModalGroup<'a> {
    pub move_type: &'a Option<GCode>,
    pub origin: &'a Coord,
    pub dest: Option<Coord>,
    pub speed: &'a Option<f32>,
    pub def_speed: &'a f32,
    pub unit: &'a Option<Unit>,
    pub reference: &'a Option<Referential>,
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

    pub fn update(&mut self, updater: &Coord) {
        if let Some(x) = updater.x {
            self.x = updater.x;;
        }
        if let Some(y) = updater.y {
            self.y = updater.y;;
        }
        if let Some(z) = updater.z {
            self.z = updater.z;;
        }
    }

    pub fn copy(&self) -> Coord {
        let mut copy = Coord::new();
        copy.x = match self.x {
            Some(a) => Some(a.clone()),
            None => None,
        };
        copy
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
