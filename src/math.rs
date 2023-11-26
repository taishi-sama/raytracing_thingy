use std::{ops, fmt::Display};

pub const EPSILON: f32 = 1e-6;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub pos: Vector3,
    pub dir: Vector3,
}
impl Ray {
    pub fn reflect(&self, hit_point: &Vector3, normal: &Vector3) -> Self {
        let refl_dir = self.dir - normal.mult(2. * self.dir.scalar_product(normal));
        Self { pos: *hit_point, dir: refl_dir  }
    }
    pub fn new_normalize(pos: Vector3, dir: &Vector3) -> Self {
        Ray { pos, dir: dir.normalize() }
    }
    #[inline(always)]
    pub fn point_from_t(&self, t: f32) -> Vector3 {
        self.pos + self.dir.mult(t)
    }
    pub fn move_forward(&self, len: f32) -> Ray {
        Ray{ pos: self.pos + self.dir.mult(len), dir: self.dir }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Vector3{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vector3 {
    #[inline(always)]
    pub fn len_sq(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
    pub const fn new(x:f32, y:f32, z:f32) -> Self {
        Self { x, y, z }
    }
    #[inline(always)]
    pub fn len(&self) -> f32 {
        self.len_sq().sqrt()
    }
    #[inline(always)]
    pub fn normalize(&self) -> Self {
        let l = self.len(); 
        Vector3 { x: self.x / l, y: self.y / l, z: self.z / l}
    }
    #[inline(always)]
    pub fn cross_product(&self, other: &Self) -> Self {
        Vector3 { 
            x: self.y * other.z - self.z * other.y , 
            y: self.z * other.x - self.x * other.z, 
            z: self.x * other.y - self.y * other.x 
        }
    }
    #[inline(always)]
    pub fn scalar_product(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    #[inline(always)]
    pub fn mult(&self, n: f32) -> Self {
        Vector3 { x: self.x * n, y: self.y * n, z: self.z * n }
    }
    #[inline(always)]
    pub fn div(&self, n: f32) -> Self {
        Vector3 { x: self.x / n, y: self.y / n, z: self.z / n }
    }
    #[inline(always)]
    pub fn inverse(&self) -> Self {
        Vector3 { x: 1.0 / self.x , y: 1.0 / self.y, z: 1.0 / self.z }
    }
    #[inline(always)]
    pub fn mult_per_element(&self, other: &Self) -> Self {
        Self { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
    pub fn lerp(&self, other: &Self, portion: f32) -> Self {
        self + &(other - self).mult(portion)
    }
    pub fn rotate_x_axis(&self, angle: f32) -> Self {
        Self { 
            x: self.x, 
            y: self.y * angle.cos() - self.z * angle.sin(), 
            z: self.y * angle.sin() + self.z * angle.cos(), }
    }
    pub fn rotate_y_axis(&self, angle: f32) -> Self {
        Self { 
            x: self.x * angle.cos() + self.z * angle.sin(), 
            y: self.y, 
            z: -self.x * angle.sin() + self.z * angle.cos(), }
    }
    pub fn rotate_z_axis(&self, angle: f32) -> Self {
        Self { 
            x: self.x * angle.cos() - self.y * angle.sin(), 
            y: self.x * angle.sin() + self.y * angle.cos(),
            z: self.z,  }
    }
}
impl ops::Add for &Vector3 {
    type Output = Vector3;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}
impl ops::Sub for &Vector3 {
    type Output = Vector3;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl ops::Neg for &Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 {x: -self.x, y: -self.y, z: -self.z}
    }
}


impl ops::Add for Vector3 {
    type Output = Vector3;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}
impl ops::Sub for Vector3 {
    type Output = Vector3;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 {x: -self.x, y: -self.y, z: -self.z}
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let decimals = f.precision().unwrap_or(3);
        let t = format!("[x:{:.decimals$}, y:{:.decimals$}, z:{:.decimals$}]", self.x, self.y, self.z);
        f.pad_integral(true, "", &t)
    }
}