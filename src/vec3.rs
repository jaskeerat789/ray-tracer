use std::ops;
use crate::util::Lib;

#[derive(Debug,PartialEq,Default,Copy,Clone)]

pub struct Vec3{
    e:[f32;3]
}
pub type Point3 = Vec3;
pub type Color = Vec3;
impl Vec3{

    pub fn x(self)->f32{
        self.e[0]
    }

    pub fn y(self)->f32{
        self.e[1]
    }

    pub fn z(self)->f32{
        self.e[2]
    }

    pub fn new (e0:f32,e1:f32,e2:f32)-> Vec3{
        Vec3{e:[e0,e1,e2]}
    }

    pub fn length(self)->f32{
        self.squared_length().sqrt()
    }

    pub fn squared_length(self)->f32{
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn unit_vector(v:&Vec3)->Vec3{
        *v / v.length()
    }

    pub fn dot(u:&Vec3,v:&Vec3) -> f32{
        u.x() * v.x()
       +u.y() * v.y()
       +u.z() * v.z() 
    }

    pub fn cross (u:&Vec3, v:&Vec3) -> Vec3{
        Vec3::new(u.y()*v.z() - u.z()*v.y(),
                  u.z()*v.x() - u.x()*v.z(),
                  u.x()*v.y() - u.y()*v.y())
    }

    pub fn random() -> Vec3{
        Vec3{
            e:[Lib::random_double(),Lib::random_double(),Lib::random_double()]
        }
    }
    
    pub fn random_min_max(min: f32, max: f32) -> Vec3{
        Vec3{
            e:[Lib::random_min_max(min, max),Lib::random_min_max(min, max),Lib::random_min_max(min, max)]
        }
    }

    pub fn random_in_unit_sphere()-> Vec3 {
        loop
        {
            let p: Vec3 = Vec3::random_min_max(-1.0, 1.0);
            
            if p.squared_length() <= 1.0
            {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3{
        let a = Lib::random_min_max(0.0, 2.0 * std::f32::consts::PI);
        let z = Lib::random_min_max(-1.0, 1.0);
        let r = (1.0 - z*z).sqrt();
        Vec3{
            e:[r * a.cos(),r * a.sin(), z]
        }
    }

    pub fn random_in_hemisphere (normal: &Vec3) -> Vec3{
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        
        if Vec3::dot(&in_unit_sphere, normal) > 0.0
        {
            return in_unit_sphere
        }

        - in_unit_sphere
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
impl ops::Sub for Vec3{
    type Output = Self;
    fn sub(self,rhs: Vec3) -> Self::Output{
        Vec3{
            e:[
                self.x() - rhs.x(),
                self.y() - rhs.y(),
                self.z() - rhs.z()
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

impl ops::Mul<Vec3> for f32{
    type Output = Vec3;
    fn mul (self,rhs:Vec3)-> Self::Output{
        rhs * self
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
    fn test_vec_sub() {
        assert_eq!(Vec3::new(1.0f32,2.0f32,3.0f32)-Vec3::new(4.0f32,5.0f32,6.0f32),Vec3{e:[-3.0f32,-3.0f32,-3.0f32]})
    }

    #[test]
    fn test_vec_mul() {
        assert_eq!(Vec3::new(1.0f32,2.0f32,3.0f32) * 2f32,Vec3{e:[2.0f32,4.0f32,6.0f32]});
    }

    #[test]
    fn test_vec_mull_2() {
        assert_eq!(2f32 * Vec3::new(1.0f32,2.0f32,3.0f32),Vec3{e:[2.0f32,4.0f32,6.0f32]});
    }

    #[test]
    fn test_vec_div() {
        assert_eq!(Vec3::new(1.0f32,2.0f32,3.0f32) / 2f32, Vec3{e:[0.5f32,1.0f32,1.5f32]});
    }
}