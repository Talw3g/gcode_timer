use gcodes_def::*;
use super::errors::*;

impl<'a> ModalGroup<'a> {
    pub fn get_distance(&'a self) -> Result<f32> {
        let move_type = match self.move_type {
            &Some(ref mt) => {
                match mt {
                    &GCode::MT(ref mt) => mt,
                    _ => return Ok(0.),
                }
            },
            &None => return Ok(0.),
        };
        match move_type {
            &MoveTypes::G0 | &MoveTypes::G1 => {
                self.get_dist_line()
            },
            _ => return Ok(0.),
        }
    }


    pub fn get_duration(&'a self) -> Result<f32> {
        let move_type = match self.move_type {
            &Some(ref mt) => {
                match mt {
                    &GCode::MT(ref mt) => mt,
                    _ => return Ok(0.),
                }
            },
            &None => return Ok(0.),
        };
        match move_type {
            &MoveTypes::G0 => {
                let duration = self.get_durat_g0()
                    .chain_err(|| "error computing travel duration for g0 group")?;
                return Ok(duration)
            },
            &MoveTypes::G1 => {
                match self.speed {
                    &Some(ref s) => {
                        let dist = self.get_distance()
                            .chain_err(|| "Error computing distance")?;
                        let g1_duration = dist/s*60.;
                        let g0_duration = self.get_durat_g0()
                            .chain_err(|| "error computing travel duration for g0 group")?;
                        let max_duration = g1_duration.max(g0_duration);
                        return Ok(max_duration);
                    },
                    &None => bail!("No speed set for move of type: G1"),
                }
            },
            ref m => {
                match self.speed {
                    &Some(ref s) => {
                        let dist = self.get_distance()
                            .chain_err(|| "Error computing distance")?;
                        let g1_duration = dist/s*60.;
                        let g0_duration = self.get_durat_g0()
                            .chain_err(|| "error computing travel duration for g0 group")?;
                        let max_duration = g1_duration.max(g0_duration);
                        return Ok(max_duration);
                    },
                    &None => bail!(format!("No speed set for move of type: {:?}", m)),
                }
            },
        };
    }

    fn get_deltas(&self) -> Result<(f32,f32,f32)> {
        let dest = match self.dest {
            Some(d) => d,
            None => return Ok((0., 0., 0.)),
        };
        let delta_x = match dest.x {
            Some(x) => {
                match self.origin.x {
                    Some(ox) => {
                        match self.reference {
                            &Some(Referential::Absolute) => x - ox,
                            &Some(Referential::Increment) => x,
                            &None => bail!("No referential set"),
                        }
                    },
                    None => bail!("X origin not initialized"),
                }
            },
            None => 0.
        };
        let delta_y = match dest.y {
            Some(y) => {
                match self.origin.y {
                    Some(oy) => {
                        match self.reference {
                            &Some(Referential::Absolute) => y - oy,
                            &Some(Referential::Increment) => y,
                            &None => bail!("No referential set"),
                        }
                    },
                    None => bail!("y origin not initialized"),
                }
            },
            None => 0.
        };
        let delta_z = match dest.z {
            Some(z) => {
                match self.origin.z {
                    Some(oz) => {
                        match self.reference {
                            &Some(Referential::Absolute) => z - oz,
                            &Some(Referential::Increment) => z,
                            &None => bail!("No referential set"),
                        }
                    },
                    None => bail!("z origin not initialized"),
                }
            },
            None => 0.
        };
        Ok((delta_x, delta_y, delta_z))
    }

    fn get_dist_line(&self) -> Result<f32> {
        let (delta_x, delta_y, delta_z) = self.get_deltas()
            .chain_err(|| "Error computing deltas")?;
        let dist = (delta_x.powi(2) + delta_y.powi(2) + delta_z.powi(2)).sqrt();
        Ok(dist)
    }

    fn get_durat_g0(&self) -> Result<f32> {
        let (delta_x, delta_y, delta_z) = self.get_deltas()
            .chain_err(|| "Error computing deltas")?;
        let &(ms_x, ms_y, ms_z) = self.max_speed;

        let dura_x = delta_x.abs() / ms_x * 60.;
        let dura_y = delta_y.abs() / ms_y * 60.;
        let dura_z = delta_z.abs() / ms_z * 60.;
        let max_duration = dura_x.max(dura_y).max(dura_z);
        Ok(max_duration)
    }
}
