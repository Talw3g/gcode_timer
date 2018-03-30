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

    fn get_dist_line(&self) -> Result<f32> {
        let dest = match self.dest {
            Some(d) => d,
            None => return Ok(0.),
        };
        let delta_x = match dest.x {
            Some(x) => {
                match self.origin.x {
                    Some(ox) => x - ox,
                    None => bail!("X origin not initialized"),
                }
            },
            None => 0.
        };
        let delta_y = match dest.y {
            Some(y) => {
                match self.origin.y {
                    Some(oy) => y - oy,
                    None => bail!("y origin not initialized"),
                }
            },
            None => 0.
        };
        let delta_z = match dest.z {
            Some(z) => {
                match self.origin.z {
                    Some(oz) => z - oz,
                    None => bail!("z origin not initialized"),
                }
            },
            None => 0.
        };
        let dist = (delta_x.powi(2) + delta_y.powi(2) + delta_z.powi(2)).sqrt();
        Ok(dist)
    }
}
