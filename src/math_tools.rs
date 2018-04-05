use objects_def::*;
use super::errors::*;

impl Coord {

    pub fn to_rad_vec(&self, dest: &Coord, reference: &Referential) -> Result<(Coord,Coord)> {
        let &dest = dest;

        let x0 = match self.x {
            Some(u) => u,
            None => bail!("Current X position not set"),
        };
        let y0 = match self.y {
            Some(u) => u,
            None => bail!("Current Y position not set"),
        };
        let z0 = match self.z {
            Some(u) => u,
            None => bail!("Current Z position not set"),
        };

        let (x1,y1,z1,i,j,k) = match reference {
            &Referential::Absolute => {
                let x1 = dest.x.unwrap_or(x0);
                let y1 = dest.y.unwrap_or(y0);
                let z1 = dest.z.unwrap_or(z0);
                let i = dest.i.unwrap_or(0.);
                let j = dest.j.unwrap_or(0.);
                let k = dest.k.unwrap_or(0.);
                (x1,y1,z1,i,j,k)
            },
            &Referential::Increment => {
                let x1 = x0 + dest.x.unwrap_or(0.);
                let y1 = y0 + dest.y.unwrap_or(0.);
                let z1 = z0 + dest.z.unwrap_or(0.);
                let i = dest.i.unwrap_or(0.);
                let j = dest.j.unwrap_or(0.);
                let k = dest.k.unwrap_or(0.);
                (x1,y1,z1,i,j,k)
            },
        };


        let cp = Coord {
            x: Some(-i),
            y: Some(-j),
            z: Some(-k),
            i: None,
            j: None,
            k: None,
        };
        let cd = Coord {
            x: Some(x1 - x0 - i),
            y: Some(y1 - y0 - j),
            z: Some(z1 - z0 - k),
            i: None,
            j: None,
            k: None,
        };
        Ok((cp,cd))
    }

    pub fn norm(&self) -> f32 {
        let x = self.x.unwrap_or(0.);
        let y = self.y.unwrap_or(0.);
        let z = self.z.unwrap_or(0.);

        let norm = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();

        norm
    }

    pub fn cross_product(&self, v2: &Coord) -> f32 {
        let x1 = self.x.unwrap_or(0.);
        let y1 = self.y.unwrap_or(0.);
        let z1 = self.z.unwrap_or(0.);

        let x2 = v2.x.unwrap_or(0.);
        let y2 = v2.y.unwrap_or(0.);
        let z2 = v2.z.unwrap_or(0.);

        let cross_prod = ((y1*z2) - (z1*y2)) + ((z1*x2) - (x1*z2)) + ((x1*y2) - (y1*x2));

        cross_prod
    }

    pub fn scalar_product(&self, v2: &Coord) -> f32 {
        let x1 = self.x.unwrap_or(0.);
        let y1 = self.y.unwrap_or(0.);
        let z1 = self.z.unwrap_or(0.);
        //println!("x1:{}, y1:{}, z1:{}", x1,y1,z1);

        let x2 = v2.x.unwrap_or(0.);
        let y2 = v2.y.unwrap_or(0.);
        let z2 = v2.z.unwrap_or(0.);
        //println!("x2:{}, y2:{}, z2:{}", x2,y2,z2);

        let scalar_prod = x1*x2 + y1*y2 + z1*z2;

        scalar_prod
    }
}

