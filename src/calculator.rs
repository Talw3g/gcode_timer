use objects_def::*;
use super::errors::*;
use std::f32::consts::PI;
use warnings::*;

enum Direction {
    CW,
    CCW,
}

impl<'a> ModalGroup<'a> {
    pub fn get_stats(&'a self, warnlog: &mut Warnlog) -> Result<(f32,f32)> {
        let dist = self.get_distance()
            .chain_err(|| "Error computing travel distance")?;
        let time = self.get_duration(dist, warnlog)
            .chain_err(|| "Error computing travel duration")?;
        Ok((time,dist))
    }

    fn get_distance(&'a self) -> Result<f32> {
        if let Some(dest) = self.dest {
            if dest.is_empty() {
                return Ok(0.)
            }
        }
        else {
            return Ok(0.)
        }

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
            &MoveTypes::G0 | &MoveTypes::G1 => self.get_dist_line(),
            &MoveTypes::G2 => self.get_dist_arc(Direction::CW),
            &MoveTypes::G3 => self.get_dist_arc(Direction::CCW),
        }
    }

    fn get_duration(&'a self, dist: f32, warnlog: &mut Warnlog) -> Result<f32> {
        let move_type = match self.move_type {
            &Some(ref mt) => {
                match mt {
                    &GCode::MT(ref mt) => mt,
                    _ => return Ok(0.),
                }
            },
            &None => return Ok(0.),
        };

        let min_duration = self.get_durat_g0()
            .chain_err(|| "Error computing travel duration for G0 group")?;

        match move_type {
            &MoveTypes::G0 => Ok(min_duration),
            &MoveTypes::G1 => {
                match self.speed {
                    &Some(ref s) => {
                        let g1_duration = dist/s*60.;
                        let mut max_duration = g1_duration;
                        if g1_duration < min_duration {
                            max_duration = min_duration;
                            warnlog.warn(WarnType::TooFast);
                        }
                        return Ok(max_duration);
                    },
                    &None => bail!("No speed set for move of type: G1"),
                }
            },
            ref m => {
                match self.speed {
                    &Some(ref s) => {
                        let arc_duration = dist/s*60.;
                        let mut max_duration = arc_duration;
                        if arc_duration < min_duration {
                            max_duration = min_duration;
                            warnlog.warn(WarnType::TooFast);
                        }
                        return Ok(max_duration);
                    },
                    &None => bail!(format!("No speed set for move of type: {:?}", m)),
                }
            },
        }
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

    fn get_dist_arc(&self, dir: Direction) -> Result<f32> {
        let dest = match self.dest {
            Some(d) => d,
            None => return Ok(0.),
        };
        let reference = match self.reference {
            &Some(ref r) => r,
            &None => bail!("No referential set"),
        };
        let (cp,cd) = self.origin.to_rad_vec(&dest, reference)
            .chain_err(|| "Error getting radius vectors")?;
        let radius = cp.norm();
        let mut theta = ( cp.scalar_product(&cd)/radius.powi(2) ).acos();
        if theta.is_nan() {
            bail!(format!("theta is NaN ! :(\norigin: {:?}\ndest: {:?}\ncp: {:?}\ncd: {:?}\nradius:{}",self.origin,dest,cp,cd,radius));
        }

        match dir {
            Direction::CW if cp.cross_product(&cd).is_sign_positive() => theta = 2.*PI - theta,
            Direction::CCW if cp.cross_product(&cd).is_sign_negative() => theta = 2.*PI - theta,
            _ => {},
        }

        if theta == 0. { theta = 2.*PI; }

        let dist = radius * theta;
//        println!("radius: {}, scalar: {}, theta: {}", radius, cp.scalar_product(&cd), theta.to_degrees());
        Ok(dist)
    }
}
