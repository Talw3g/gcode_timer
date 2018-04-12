use super::Cnc;

#[derive(Debug)]
pub struct Machine {
    pub move_type: Option<GCode>,
    pub pos: Coord,
    pub max_speed: (f32,f32,f32),
    pub speed: Option<f32>,
    pub unit: Option<Unit>,
    pub reference: Option<Referential>,
    pub tool_number: Option<u8>,
    pub status: Status,

}
impl Machine {
    pub fn new(config: Cnc) -> Machine {
        Machine {
            move_type: None,
            pos: Coord {
                x: Some(0.0),
                y: Some(0.0),
                z: Some(0.0),
                i: None,
                j: None,
                k: None,
            },
            max_speed: (
                config.speed_x,
                config.speed_y,
                config.speed_z,
            ),
            speed: None,
            unit: None,
            reference: None,
            tool_number: None,
            status: Status::Continue,
        }
    }
}



#[derive(Debug)]
pub struct ModalGroup<'a> {
    pub move_type: &'a Option<GCode>,
    pub origin: Coord,
    pub dest: Option<Coord>,
    pub speed: &'a Option<f32>,
    pub max_speed: &'a (f32,f32,f32),
    pub unit: &'a Option<Unit>,
    pub reference: &'a Option<Referential>,
}


#[derive(Debug, Copy)]
pub struct Tool {
    pub tool_number: Option<u8>,
    pub duration: f32,
    pub distance: f32,
}
impl Tool {
    pub fn new(tool_number: Option<u8>) -> Tool {
        Tool {
            tool_number,
            duration: 0.,
            distance: 0.,
        }
    }

    pub fn reset(&mut self, tool: Option<u8>) {
        self.tool_number = tool;
        self.duration = 0.;
        self.distance = 0.;
    }
}
impl Clone for Tool {
    fn clone(&self) -> Tool {*self}
}


#[derive(Debug, Copy)]
pub struct Coord {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
    pub i: Option<f32>,
    pub j: Option<f32>,
    pub k: Option<f32>,

}
impl Coord {
    pub fn new() -> Coord {
        Coord {
            x: None,
            y: None,
            z: None,
            i: None,
            j: None,
            k: None,
        }
    }

    pub fn update(&mut self, updater: &Coord) {
        if let Some(_) = updater.x {
            self.x = updater.x;;
        }
        if let Some(_) = updater.y {
            self.y = updater.y;;
        }
        if let Some(_) = updater.z {
            self.z = updater.z;;
        }
    }

    pub fn add(&mut self, adder: &Coord) {
        if let Some(ax) = adder.x {
            if let Some(x) =  self.x {
                self.x = Some(x + ax);
            }
        }
        if let Some(ay) = adder.y {
            if let Some(y) =  self.y {
                self.y = Some(y + ay);
            }
        }
        if let Some(az) = adder.z {
            if let Some(z) =  self.z {
                self.z = Some(z + az);
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        if let &Coord {x: None, y: None, z: None,
            i: None, j: None, k: None,} = self {
            return true
        }
        else {
            return false
        }
    }

    pub fn to_mm(&mut self) {
        if let Some(u) = self.x {
            self.x = Some(u*25.4);
        }
        if let Some(u) = self.y {
            self.y = Some(u*25.4);
        }
        if let Some(u) = self.z {
            self.z = Some(u*25.4);
        }
        if let Some(u) = self.i {
            self.i = Some(u*25.4);
        }
        if let Some(u) = self.j {
            self.j = Some(u*25.4);
        }
        if let Some(u) = self.k {
            self.k = Some(u*25.4);
        }
    }
}
impl Clone for Coord {
    fn clone(&self) -> Coord {*self}
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

#[derive(Debug)]
pub enum Status {
    Continue,
    EOP,
}
