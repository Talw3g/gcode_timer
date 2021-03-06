use super::errors::*;
use super::objects_def::*;
use super::lineparser::*;

impl Machine {
    pub fn line_depacker(&mut self, line: Vec<Codes>) -> Result<(ModalGroup, &Option<u8>)> {
        let mut dest = Coord::new();
        let mut speed = None;
        for item in line {
            match item {
                Codes::G(i) => {
                    let gcode = g_tokenizer(i)
                        .chain_err(|| "Error depacking G code")?;
                    self.process_gcode(gcode);
                },
                Codes::M(i) => {
                    if i == 30 {
                        self.status = Status::EOP;
                    }
                },
                Codes::T(i) => {
                    self.tool_number = Some(i);
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
                Codes::K(i) => {
                    if let Some(_) = dest.k {
                        bail!("Two K coordinate in the same modal group");
                    }
                    dest.k = Some(i);
                },
                Codes::F(i) => speed = Some(i),
            }
        }

        let (dest, speed) = self.convert_units(dest,speed)
            .chain_err(|| "Error converting units")?;

        if let Some(s) = speed {
            self.speed = Some(s);
        }

        let modgroup = ModalGroup {
            move_type: &self.move_type,
            origin: self.pos.clone(),
            dest: Some(dest),
            speed: &self.speed,
            max_speed: &self.max_speed,
            unit: &self.unit,
            reference: &self.reference,
        };
        match modgroup.dest {
            Some(ref dest) => {
                self.pos.update(&dest);
            },
            None => {},
        }
        Ok((modgroup, &self.tool_number))
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

    fn convert_units(&self, mut dest: Coord, mut speed: Option<f32>) -> Result<(Coord, Option<f32>)> {
        match self.unit {
            Some(Unit::Inch) => {
                dest.to_mm();
                if let Some(u) = speed {
                    speed = Some(u*25.4);
                }
            },
            Some(Unit::MM) => {},
            None => {
                if !dest.is_empty() || !speed.is_none() {
                    bail!("Coordinates set without unit.");
                }
            },
        }
        Ok((dest, speed))
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
