//! توابع کوچک برای نمایش benchmark در فصل «عملکرد و بهینه‌سازی».

use rayon::prelude::*;

/// ساخت بردار بدون پیش‌تخصیص.
pub fn build_no_capacity(n: usize) -> Vec<u64> {
    let mut v = Vec::new();
    for i in 0..n as u64 {
        v.push(i * 2);
    }
    v
}

/// ساخت بردار با `with_capacity`.
pub fn build_with_capacity(n: usize) -> Vec<u64> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n as u64 {
        v.push(i * 2);
    }
    v
}

/// جمع با حلقه‌ی اندیس‌دار (bounds check در هر دسترسی، مگر اینکه بهینه‌ساز حذفش کند).
#[allow(clippy::needless_range_loop)] // عمداً اندیس‌دار؛ موضوع مقایسه است
pub fn sum_indexed(data: &[u64]) -> u64 {
    let mut total = 0u64;
    for i in 0..data.len() {
        total = total.wrapping_add(data[i]);
    }
    total
}

/// جمع با iterator.
pub fn sum_iter(data: &[u64]) -> u64 {
    data.iter().fold(0u64, |a, &b| a.wrapping_add(b))
}

/// جمع مربع‌های اعداد زوج، به‌صورت زنجیره‌ی iterator.
pub fn sum_even_squares_iter(data: &[u64]) -> u64 {
    data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x.wrapping_mul(x))
        .fold(0u64, |a, b| a.wrapping_add(b))
}

/// همان محاسبه با حلقه‌ی دستی.
pub fn sum_even_squares_loop(data: &[u64]) -> u64 {
    let mut total = 0u64;
    for &x in data {
        if x % 2 == 0 {
            total = total.wrapping_add(x.wrapping_mul(x));
        }
    }
    total
}

/// همان محاسبه به‌صورت موازی با rayon.
pub fn sum_even_squares_par(data: &[u64]) -> u64 {
    data.par_iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x.wrapping_mul(x))
        .reduce(|| 0u64, |a, b| a.wrapping_add(b))
}

/// AoS: هر ذره یک struct است.
#[derive(Clone, Copy)]
pub struct ParticleAos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub mass: f32,
    pub id: u64,
    pub name_hash: [u64; 4],
}

/// SoA: هر فیلد در بردار جدا.
pub struct ParticlesSoa {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub z: Vec<f32>,
    pub mass: Vec<f32>,
    pub id: Vec<u64>,
    pub name_hash: Vec<[u64; 4]>,
}

pub fn make_aos(n: usize) -> Vec<ParticleAos> {
    (0..n)
        .map(|i| ParticleAos {
            x: i as f32,
            y: 0.0,
            z: 0.0,
            mass: 1.0 + (i % 7) as f32,
            id: i as u64,
            name_hash: [0; 4],
        })
        .collect()
}

pub fn make_soa(n: usize) -> ParticlesSoa {
    ParticlesSoa {
        x: (0..n).map(|i| i as f32).collect(),
        y: vec![0.0; n],
        z: vec![0.0; n],
        mass: (0..n).map(|i| 1.0 + (i % 7) as f32).collect(),
        id: (0..n as u64).collect(),
        name_hash: vec![[0; 4]; n],
    }
}

/// جمع جرم‌ها در AoS: برای هر ذره کل struct (۵۶ بایت) از حافظه‌ی کش عبور می‌کند.
pub fn total_mass_aos(p: &[ParticleAos]) -> f32 {
    p.iter().map(|p| p.mass).sum()
}

/// جمع جرم‌ها در SoA: فقط آرایه‌ی `mass` خوانده می‌شود.
pub fn total_mass_soa(p: &ParticlesSoa) -> f32 {
    p.mass.iter().sum()
}

/// Dispatch ایستا.
pub trait Shape {
    fn area(&self) -> f64;
}
pub struct Square(pub f64);
pub struct Circle(pub f64);
impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.0 * self.0
    }
}

/// همان شکل‌ها به‌صورت enum (ارسال با match، بدون heap و بدون vtable).
pub enum AnyShape {
    Square(f64),
    Circle(f64),
}

impl Shape for AnyShape {
    fn area(&self) -> f64 {
        match self {
            AnyShape::Square(s) => s * s,
            AnyShape::Circle(r) => std::f64::consts::PI * r * r,
        }
    }
}

pub fn total_area_dyn(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

pub fn total_area_generic<S: Shape>(shapes: &[S]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builders_agree() {
        assert_eq!(build_no_capacity(100), build_with_capacity(100));
    }

    #[test]
    fn sums_agree() {
        let v = build_with_capacity(1000);
        assert_eq!(sum_indexed(&v), sum_iter(&v));
        let a = sum_even_squares_iter(&v);
        assert_eq!(a, sum_even_squares_loop(&v));
        assert_eq!(a, sum_even_squares_par(&v));
    }

    #[test]
    fn layouts_agree() {
        let a = make_aos(1000);
        let s = make_soa(1000);
        assert_eq!(total_mass_aos(&a), total_mass_soa(&s));
        assert_eq!(std::mem::size_of::<ParticleAos>(), 56);
    }

    #[test]
    fn mixed_dispatch_agree() {
        let boxed: Vec<Box<dyn Shape>> = (0..10)
            .map(|i| -> Box<dyn Shape> {
                if i % 2 == 0 { Box::new(Square(i as f64)) } else { Box::new(Circle(i as f64)) }
            })
            .collect();
        let en: Vec<AnyShape> = (0..10)
            .map(|i| if i % 2 == 0 { AnyShape::Square(i as f64) } else { AnyShape::Circle(i as f64) })
            .collect();
        assert_eq!(total_area_dyn(&boxed), total_area_generic(&en));
    }

    #[test]
    fn dispatch_agree() {
        let boxed: Vec<Box<dyn Shape>> = (0..10)
            .map(|i| Box::new(Square(i as f64)) as Box<dyn Shape>)
            .collect();
        let plain: Vec<Square> = (0..10).map(|i| Square(i as f64)).collect();
        assert_eq!(total_area_dyn(&boxed), total_area_generic(&plain));
    }
}
