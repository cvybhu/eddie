use super::super::bignum_u512::u512;
use super::{CurvePoint, CURVE_D_CONSTANT};

// Equality
impl std::cmp::PartialEq for CurvePoint {
    fn eq(&self, other: &CurvePoint) -> bool {
        return self.get_x() == other.get_x() && self.get_y() == other.get_y();
    }
}

impl std::cmp::Eq for CurvePoint {}


// Ordering
impl std::cmp::Ord for CurvePoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {

        let x_cmp = self.get_x().cmp(&other.get_x());
        if x_cmp != std::cmp::Ordering::Equal {
            return x_cmp;
        }

        return self.get_y().cmp(&other.get_y());
    }
}

impl std::cmp::PartialOrd for CurvePoint {
    fn partial_cmp(&self, other: &CurvePoint) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}


// Addition
impl std::ops::Add for &CurvePoint {
    type Output = CurvePoint;

    fn add(self, other: &CurvePoint) -> CurvePoint {
        
        // Addition using projective coordiantes
        let xx = &self.x * &other.x;                // C = X1 * X2
        let yy = &self.y * &other.y;                // D = Y1 * Y2
        let zz = &self.z * &other.z;                // A = Z1 * Z2
        let z4 = &zz * &zz;                         // B = A^2
        let dxxyy = &CURVE_D_CONSTANT * &xx * &yy;  // E = dC * D
        let z4_min_dxxyy = &z4 - &dxxyy;            // F = B - E
        let z4_plus_dxxyy = &z4 + &dxxyy;           // G = B + E

        let result = CurvePoint {
            x: &zz * &z4_min_dxxyy * ((&self.x + &self.y) * (&other.x + &other.y) - &xx - &yy),
            y: &zz * &z4_plus_dxxyy * (&yy + &xx),
            z: &z4_min_dxxyy * &z4_plus_dxxyy
        };

        //assert!(CurvePoint::is_on_curve(&result.x, &result.y, &result.z));

        return result;
    }
}

impl std::ops::AddAssign<&CurvePoint> for CurvePoint {
    fn add_assign(&mut self, other: &CurvePoint) {
        *self = &*self + other;
    }
}


impl CurvePoint {

    // Point doubling
    // point.doubled() = point + point
    // It's about twice as efficient to do it this way instead of the general addition operator
    pub fn doubled(&self) -> CurvePoint {
        let x2 = &self.x * &self.x;
        let y2 = &self.y * &self.y;
        let z2 = &self.z * &self.z;
        
        let x_plus_y = &self.x + &self.y;
        let x_plus_y_2 = &x_plus_y * &x_plus_y;

        let y2_min_x2 = &y2 - &x2;
        let j = &y2_min_x2 - (&z2 + &z2);

        let result = CurvePoint {
            x: (x_plus_y_2 - x2 - y2) * j,
            y: -y2_min_x2 * (x2 + y2),
            z: y2_min_x2 * j
        };

        return result;
    }
}

// Multiplication of point by scalar
impl std::ops::Mul<&CurvePoint> for u512 {
    type Output = CurvePoint;

    fn mul(mut self, point: &CurvePoint) -> CurvePoint {
        let mut result: CurvePoint = CurvePoint::identity();
        let mut cur_point_doubling: CurvePoint = *point;

        while self != u512::zero() {
            if self.get_bit(0) {
                result += &cur_point_doubling;
            }

            cur_point_doubling = cur_point_doubling.doubled();

            self >>= 1;
        }

        return result;
    }
}


// Negation
impl std::ops::Neg for &CurvePoint {
    type Output = CurvePoint;

    fn neg(self) -> CurvePoint {
        return CurvePoint { x: -self.x, y: self.y, z: self.z };
    }
}


// Version for other reference variations
impl std::ops::Add<CurvePoint> for CurvePoint {
    type Output = CurvePoint;

    fn add(self, other: CurvePoint) -> CurvePoint {
        return &self + &other;
    }
}

impl std::ops::Add<&CurvePoint> for CurvePoint {
    type Output = CurvePoint;

    fn add(self, other: &CurvePoint) -> CurvePoint {
        return &self + other;
    }
}

impl std::ops::Add<CurvePoint> for &CurvePoint {
    type Output = CurvePoint;

    fn add(self, other: CurvePoint) -> CurvePoint {
        return self + &other;
    }
}

impl std::ops::AddAssign<CurvePoint> for CurvePoint {
    fn add_assign(&mut self, other: CurvePoint) {
        *self += &other;
    }
}