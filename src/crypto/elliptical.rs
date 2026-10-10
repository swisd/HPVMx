use core::ops::{Add, Mul, Neg, Sub};
use core::fmt;


pub trait FieldElement:
    Sized
    + Clone
    + Copy
    + Eq
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Neg<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
    fn is_zero(&self) -> bool;
    fn invert(&self) -> Option<Self>;

    fn add_field(self, rhs: Self) -> Self {
        <Self as Add<Self>>::add(self, rhs)
    }

    fn sub_field(self, rhs: Self) -> Self {
        <Self as Sub<Self>>::sub(self, rhs)
    }

    fn mul_field(self, rhs: Self) -> Self {
        <Self as Mul<Self>>::mul(self, rhs)
    }

    fn neg_field(self) -> Self {
        <Self as Neg>::neg(self)
    }

    fn double(&self) -> Self {
        self.add_field(*self)
    }
}


pub trait CurveConfig: Clone + Copy {
    type Field: FieldElement;

    const A: Self::Field;
    const B: Self::Field;

    fn is_on_curve(x: &Self::Field, y: &Self::Field) -> bool {
        let y_sq = y.mul_field(*y);
        let x_sq = x.mul_field(*x);
        let x_cu = x_sq.mul_field(*x);
        let ax = Self::A.mul_field(*x);
        let rhs = x_cu.add_field(ax).add_field(Self::B);
        y_sq == rhs
    }
}


pub enum Point<C: CurveConfig> {
    Coordinate { x: C::Field, y: C::Field },
    Identity,
}

impl<C: CurveConfig> Copy for Point<C> {}

impl<C: CurveConfig> Clone for Point<C> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<C: CurveConfig> fmt::Debug for Point<C>
where
    C::Field: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Point::Coordinate { x, y } => f.debug_tuple("Coordinate").field(x).field(y).finish(),
            Point::Identity => f.write_str("Identity"),
        }
    }
}

impl<C: CurveConfig> PartialEq for Point<C> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Point::Coordinate { x: x1, y: y1 },
                Point::Coordinate { x: x2, y: y2 },
            ) => x1 == x2 && y1 == y2,
            (Point::Identity, Point::Identity) => true,
            _ => false,
        }
    }
}

impl<C: CurveConfig> Eq for Point<C> {}

impl<C: CurveConfig> Point<C> {
    pub fn new(x: C::Field, y: C::Field) -> Option<Self> {
        if C::is_on_curve(&x, &y) {
            Some(Point::Coordinate { x, y })
        } else {
            None
        }
    }
}

impl<C: CurveConfig> Add for Point<C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Point::Identity, p) => p,
            (p, Point::Identity) => p,
            (Point::Coordinate { x: x1, y: y1 }, Point::Coordinate { x: x2, y: y2 }) => {
                if x1 == x2 {
                    if y1 == y2.neg_field() || y1.is_zero() {
                        Point::Identity
                    } else {
                        Point::Coordinate { x: x1, y: y1 }.double_point()
                    }
                } else {
                    let num = y2.sub_field(y1);
                    let dx = x2.sub_field(x1);
                    let den: C::Field = match dx.invert() {
                        Some(d) => d,
                        None => return Point::<C>::Identity, // Handle exceptional case safely
                    };
                    let lambda = num.mul_field(den);

                    let lambda_sq = lambda.mul_field(lambda);
                    let x3 = lambda_sq.sub_field(x1).sub_field(x2);
                    let dx3 = x1.sub_field(x3);
                    let y3 = lambda.mul_field(dx3).sub_field(y1);

                    Point::Coordinate { x: x3, y: y3 }
                }
            }
        }
    }
}

impl<C: CurveConfig> Point<C> {
    pub fn double_point(self) -> Self {
        match self {
            Point::Identity => Point::Identity,
            Point::Coordinate { x, y } => {
                if y.is_zero() {
                    return Point::Identity;
                }

                let x_sq = x.mul_field(x);
                let num = x_sq.double().add_field(x_sq).add_field(C::A);

                let y2: C::Field = y.double();
                let den: C::Field = match y2.invert() {
                    Some(d) => d,
                    None => return Point::<C>::Identity,
                };
                let lambda = num.mul_field(den);

                let lambda_sq = lambda.mul_field(lambda);
                let x3 = lambda_sq.sub_field(x.double());
                let dx3 = x.sub_field(x3);
                let y3 = lambda.mul_field(dx3).sub_field(y);

                Point::Coordinate { x: x3, y: y3 }
            }
        }
    }
}