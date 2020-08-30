use std::ops;

#[derive(Debug,PartialEq,Copy,Clone)]

pub struct Vec3{
    e:[f32;3]
}

impl Vec3{
    pub fn new (e0:f32,e1:f32,e2:f32)-> Vec3{
        Vec3{e:[e0,e1,e2]}
    }

    pub fn length(self)->f32{
        (
            self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
        ).sqrt()
    }

    pub fn unit_vector(v:&Vec3)->Vec3{
        *v / v.length()
    }

    pub fn x(self)->f32{
        self.e[0]
    }

    pub fn y(self)->f32{
        self.e[1]
    }

    pub fn z(self)->f32{
        self.e[2]
    }
}

impl ops::Add for Vec3{
    type Output = Self;
    fn add(self,rhs:Vec3)-> Self::Output{
        Vec3{
            e:[
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ]
        }
    }
}
impl ops::Neg for Vec3{
    type Output = Self;
    fn neg(self)->Self::Output{
        Vec3{
            e:[-self.e[0],-self.e[1],-self.e[2]],
        }
    }
}

impl ops::Mul<f32> for Vec3{
    type Output = Self;
    fn mul(self,rhs:f32)->Self::Output{
        Vec3{
            e:[self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs]
        }
    }
}

impl ops::Div<f32> for Vec3{
    type Output = Self;
    fn div(self,rhs:f32)->Self::Output{
        Vec3{
            e:[self.e[0]/rhs,self.e[1]/rhs,self.e[2]/rhs]
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_vec3() {
        assert_eq!(Vec3::new(1.0f32,2.0f32,3.0f32),Vec3{e:[1.0f32,2.0f32,3.0f32]})
    }
    #[test]
    fn test_vec_add() {
        assert_eq!(Vec3::new(1.0f32,2.0f32,3.0f32)+Vec3::new(4.0f32,5.0f32,6.0f32),Vec3{e:[5.0f32,7.0f32,9.0f32]})
    }

    #[test]
    fn test_vec_mul() {
        assert_eq!(Vec3::new(1.0f32,2.0f32,3.0f32) * 2f32,Vec3{e:[2.0f32,4.0f32,6.0f32]});
    }
    #[test]
    fn test_vec_div() {
        assert_eq!(Vec3::new(1.0f32,2.0f32,3.0f32) / 2f32, Vec3{e:[0.5f32,1.0f32,1.5f32]});
    }
}