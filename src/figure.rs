use std::mem::swap;

use crate::{
    material::Material,
    math::{Ray, Vector3, EPSILON},
};

#[derive(Debug, Clone)]
pub enum FigureKind {
    Side { pos: [Vector3; 3], normal: Vector3, m: Material },
    //Порядок сторон:
    //Задняя верхняя левая, задняя верхняя правая, 
    //задняя нижняя правая, задняя нижняя левая
    //Передняя верхняя левая, Передняя верхняя правая, 
    //Передняя нижняя правая, Передняя нижняя левая
    //Нормали: верхняя, задняя, правая, передняя, левая, нижняя
    Cube { pos: [Vector3; 8], normals: [Vector3; 6], m: Material },
    Sphere { r: f32, pos: Vector3, m: Material },
}
impl FigureKind {
    //Вектор нормали смотрит по направлению взгляда на углы.
    pub fn new_side(top_left: &Vector3, top_right: &Vector3, down_left: &Vector3, m: Material) -> Self {
        let normal = Self::plane_normal(top_left, top_right, down_left);
        Self::Side {pos: [*top_left, *top_right, *down_left], normal, m }
    }
    pub fn new_cube_from_d(back_top_left: &Vector3, dw: &Vector3, dh: &Vector3, dd: &Vector3, m: Material) -> Self {
        Self::new_cube(back_top_left,
             &(back_top_left + dw),
              &(back_top_left + dh),
               &(back_top_left + dd), m)
    }
    pub fn new_cube(back_top_left: &Vector3, back_top_right: &Vector3, back_down_left: &Vector3, front_top_left: &Vector3, m: Material) -> Self {
        let h_v = back_down_left - back_top_left;
        let w_v = back_top_right - back_top_left;
        //let d_v = front_top_left - back_top_left;
        let front_top_right = front_top_left + &w_v;
        let front_down_left = front_top_left + &h_v;
        let back_down_right = back_down_left + &w_v;
        let front_down_right = front_down_left + h_v;
        let top_normal = -Self::plane_normal(back_top_left, back_top_right, front_top_left);
        let down_normal = -top_normal;
        let back_normal = Self::plane_normal(back_top_left, back_top_right, back_down_left);
        let front_normal = -back_normal;
        let left_normal = Self::plane_normal(front_top_left, back_top_left, &front_down_left);
        let right_normal = -left_normal;
        Self::Cube { pos: [*back_top_left, *back_top_right, back_down_right, *back_down_left,
            *front_top_left, front_top_right, front_down_right, front_down_left
            ], normals: [
                top_normal, back_normal, right_normal, front_normal, left_normal, down_normal
            ], m}
        
    }
    pub fn get_material(&self) -> &Material {
        match self {
            FigureKind::Side { m, .. } => m,
            FigureKind::Cube { m, .. } => m,
            FigureKind::Sphere { m, .. } => m,
        }
    }   
    pub fn intersect(&self, ray: &Ray) -> Option<Vector3> {
        match self {
            FigureKind::Side { pos, normal, .. } => Self::rectangle_intersect(ray, &pos[0], &pos[1], &pos[2], normal),
            FigureKind::Cube { pos, normals, .. } => Self::cube_intersect(ray, pos, normals).map(|x|x.0),
            FigureKind::Sphere { r, pos, .. } => Self::sphere_intersect(ray, *r, pos),
        }
    }
    //Точка пересечения и нормаль
    pub fn intersect_with_normal(&self, ray: &Ray) -> Option<(Vector3, Vector3)> {
        match self {
            FigureKind::Side { pos, normal, ..} => Self::rectangle_intersect(ray, &pos[0], &pos[1], &pos[2], normal).map(|x|(x, *normal)),
            FigureKind::Cube { pos, normals, .. } => Self::cube_intersect(ray, pos, normals).map(|x|(x.0, normals[x.1])),
            FigureKind::Sphere { r, pos, .. } => Self::sphere_intersect(ray, *r, pos).map(|x|
                (x, (&x - pos).normalize())
            ),
        }
    }
    pub fn plane_normal(pp1: &Vector3, pp2: &Vector3, pp3: &Vector3) -> Vector3 {
        let v1 = pp1 - pp2;
        let v2 = pp1 - pp3;
        v1.cross_product(&v2).normalize()
    }
    pub fn cube_intersect(r: &Ray, dots: &[Vector3; 8], normals: &[Vector3; 6]) -> Option<(Vector3, usize)> {
        let mut d = f32::MAX;
        let mut t = (Vector3::new(0.0, 0.0, 0.0), usize::MAX);
        if let Some(p) = Self::rectangle_intersect(r, &dots[0], &dots[1], &dots[4], &normals[0]) {
            let tmp = (p - r.pos).len_sq();
            if tmp < d {
                d = tmp;
                t = (p, 0)
            }
        }
        
        
        if let Some(p) = Self::rectangle_intersect(r, &dots[0], &dots[1], &dots[3], &normals[1]) {
            let tmp = (p - r.pos).len_sq();
            if tmp < d {
                d = tmp;
                t = (p, 1)
            }
        }
        if let Some(p) = Self::rectangle_intersect(r, &dots[1], &dots[5], &dots[2], &normals[2]) {
            let tmp = (p - r.pos).len_sq();
            if tmp < d {
                d = tmp;
                t = (p, 2)
            }
        }
        if let Some(p) = Self::rectangle_intersect(r, &dots[4], &dots[5], &dots[7], &normals[3]) {
            let tmp = (p - r.pos).len_sq();
            if tmp < d {
                d = tmp;
                t = (p, 3)
            }
        }
        if let Some(p) = Self::rectangle_intersect(r, &dots[4], &dots[0], &dots[7], &normals[4]) {
            let tmp = (p - r.pos).len_sq();
            if tmp < d {
                d = tmp;
                t = (p, 4)
            }
        }
        if let Some(p) = Self::rectangle_intersect(r, &dots[3], &dots[2], &dots[7], &normals[5]) {
            let tmp = (p - r.pos).len_sq();
            if tmp < d {
                t = (p, 5)
            }
        }
        if t.1 != usize::MAX {
            Some(t)
        } else {None}
    }
    pub fn sphere_intersect(r: &Ray, radius: f32, pos: &Vector3) -> Option<Vector3> {
        let l = pos - &r.pos;
        let tca = l.scalar_product(&r.dir);
        if tca < 0.0 {return None};
        let d2 = l.scalar_product(&l) - tca * tca;
        if (d2) > radius {return None;}
        let thc = (radius - d2).sqrt();
        let mut t0 = tca - thc;
        let mut t1 = tca + thc;
        if t0 > t1 {swap(&mut t0, &mut t1);}
        if t0 < 0.0 {
            t0 = t1;
            if t0 < 0.0 {return  None;}
        }
        Some(r.point_from_t(t0))
    }
    //Return t from ray equation r.pos + r.dir * t.
    pub fn plane_intersect(r: &Ray, pp1: &Vector3, normal: &Vector3) -> Option<Vector3> {
        let denom = normal.scalar_product(&r.dir);
        if denom.abs() > EPSILON {
            let p010 = pp1 - &r.pos;
            let t = p010.scalar_product(normal) / denom;
            if t <= EPSILON * 10.0 {
                None
            } else {
                Some(r.point_from_t(t))
            }
        } else {
            None
        }
    }

    pub fn rectangle_intersect(
        r: &Ray,
        top_left: &Vector3,
        top_right: &Vector3,
        down_left: &Vector3,
        normal: &Vector3
    ) -> Option<Vector3> {

        let b = top_left;
        let c = top_right;
        let e = down_left;
        let c_b = c - b;
        let e_b = e - b;
        //let pn = e_b.cross_product(&c_b).normalize();
        
        let a = Self::plane_intersect(r, top_left, normal)?;
        //https://math.stackexchange.com/questions/476608/how-to-check-if-point-is-within-a-rectangle-on-a-plane-in-3d-space
        let t1 = a.scalar_product(&c_b);
        let t2 = a.scalar_product(&e_b);
        if b.scalar_product(&c_b) - EPSILON <= t1
            && t1 <= c.scalar_product(&c_b) + EPSILON
            && b.scalar_product(&e_b) - EPSILON <= t2
            && t2 <= e.scalar_product(&e_b) + EPSILON
        {
            Some(a)
        } else {
            None
        }
    }
}
