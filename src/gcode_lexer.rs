use super::errors::*;
use super::gcodes_def::*;
use super::lineparser::*;

impl Machine {
    pub fn line_depacker(&mut self, line: Vec<Codes>) -> Result<Option<ModalGroup>> {
        let mut dest = Coord::new();
        for item in line {
            match item {
                Codes::G(i) => {
                    let gcode = g_tokenizer(i)
                        .chain_err(|| "Error depacking G code")?;
                    println!("gcode: {:?}", gcode);
                    self.process_gcode(gcode);
                },
                Codes::M(i) => {
                },
                Codes::X(i) => {
                    if let Some(_) = dest.x {
                        bail!("Two X coordinate in the same modal group");
                    }
                    dest.x = Some(i);
                },
                Codes::Y(i) => {
                    if let Some(_) = dest.y {
                        bail!("Two Y coordinate in the same modal group");
                    }
                    dest.y = Some(i);
                },
                Codes::Z(i) => {
                    if let Some(_) = dest.z {
                        bail!("Two Z coordinate in the same modal group");
                    }
                    dest.z = Some(i);
                },
                Codes::I(i) => {
                    if let Some(_) = dest.i {
                        bail!("Two I coordinate in the same modal group");
                    }
                    dest.i = Some(i);
                },
                Codes::J(i) => {
                    if let Some(_) = dest.j {
                        bail!("Two J coordinate in the same modal group");
                    }
                    dest.j = Some(i);
                },
                Codes::F(i) => {
                },
            }
        }
        let modgroup = ModalGroup {
            move_type: &self.move_type,
            origin: &self.pos.copy(),
            dest: Some(dest),
            speed: &self.speed,
            def_speed: &self.def_speed,
            unit: &self.unit,
            reference: &self.reference,
        };
        match modgroup.dest {
            Some(ref dest) => {
                self.pos.update(&dest);
            },
            None => {},
        }
        Ok(Some(modgroup))
    }

    fn process_gcode(&mut self, gcode: GCode) {
        match gcode {
            GCode::MT(_) => self.move_type = Some(gcode),
            GCode::Flag(f) => {
                match f {
                    Flags::G20 => self.unit = Some(Unit::Inch),
                    Flags::G21 => self.unit = Some(Unit::MM),
                    Flags::G90 => self.reference = Some(Referential::Absolute),
                    Flags::G91 => self.reference = Some(Referential::Increment),
                }
            },
            GCode::Dump => {},
        }
    }


}

fn g_tokenizer(i: u8) -> Result<GCode> {
    match i {
        0 => Ok(GCode::MT(MoveTypes::G0)),
        1 => Ok(GCode::MT(MoveTypes::G1)),
        2 => Ok(GCode::MT(MoveTypes::G2)),
        3 => Ok(GCode::MT(MoveTypes::G3)),
        20 => Ok(GCode::Flag(Flags::G20)),
        21 => Ok(GCode::Flag(Flags::G21)),
        90 => Ok(GCode::Flag(Flags::G90)),
        91 => Ok(GCode::Flag(Flags::G91)),
        _ => Ok(GCode::Dump),
    }
}
