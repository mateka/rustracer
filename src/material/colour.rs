use auto_ops::*;
use image::Rgb;

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Colour {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Colour {
    pub fn clamped(&self) -> Self {
        let components = [self.red, self.green, self.blue];
        let norm = components
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        self / norm.max(1.0)
    }
}

impl From<Rgb<u8>> for Colour {
    fn from(cl: Rgb<u8>) -> Self {
        Self {
            red: cl.0[0] as f32 / 255.0,
            green: cl.0[1] as f32 / 255.0,
            blue: cl.0[2] as f32 / 255.0,
        }
    }
}

impl Into<Rgb<u8>> for Colour {
    fn into(self) -> Rgb<u8> {
        let self_byted = self.clamped() * 255.0;
        Rgb([
            self_byted.red as u8,
            self_byted.green as u8,
            self_byted.blue as u8,
        ])
    }
}

impl_op_ex!(+|a: &Colour, b: &Colour| -> Colour {
    Colour {
        red: a.red + b.red,
        green: a.green + b.green,
        blue: a.blue + b.blue,
    }
});

impl_op_ex!(+=|a: &mut Colour, b: &Colour| {
    a.red += b.red;
    a.green += b.green;
    a.blue += b.blue;
});

impl_op_ex!(-|a: &Colour, b: &Colour| -> Colour {
    Colour {
        red: a.red - b.red,
        green: a.green - b.green,
        blue: a.blue - b.blue,
    }
});

impl_op_ex!(-=|a: &mut Colour, b: &Colour| {
    a.red -= b.red;
    a.green -= b.green;
    a.blue -= b.blue;
});

impl_op_ex!(*|a: &Colour, b: &Colour| -> Colour {
    Colour {
        red: a.red * b.red,
        green: a.green * b.green,
        blue: a.blue * b.blue,
    }
});

impl_op_ex!(*=|a: &mut Colour, b: &Colour| {
    a.red *= b.red;
    a.green *= b.green;
    a.blue *= b.blue;
});

impl_op_ex!(/|a: &Colour, b: &Colour| -> Colour {
    Colour {
        red: a.red / b.red,
        green: a.green / b.green,
        blue: a.blue / b.blue,
    }
});

impl_op_ex!(/=|a: &mut Colour, b: &Colour| {
    a.red /= b.red;
    a.green /= b.green;
    a.blue /= b.blue;
});

impl_op_ex_commutative!(*|c: &Colour, s: &f32| -> Colour {
    Colour {
        red: s * c.red,
        green: s * c.green,
        blue: s * c.blue,
    }
});

impl_op_ex!(*=|c: &mut Colour, s: &f32| {
    c.red *= s;
    c.green *= s;
    c.blue *= s;
});

impl_op_ex!(/|c: &Colour, s: &f32| -> Colour {
    Colour {
        red: c.red / s,
        green: c.green / s,
        blue: c.blue / s,
    }
});

impl_op_ex!(/=|c: &mut Colour, s: &f32| {
    c.red /= s;
    c.green /= s;
    c.blue /= s;
});

#[allow(clippy::op_ref)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamped_colour_has_components_between_0_and_1() {
        #[rustfmt::skip]
        let c = Colour {red: 0.5, green: 1.0, blue: 2.0,};
        #[rustfmt::skip]
        assert_eq!(Colour{red: 0.25, green: 0.5, blue: 1.0}, c.clamped());
    }

    #[test]
    fn clamped_does_not_change_components_between_0_and_1() {
        #[rustfmt::skip]
        let c = Colour {red: 0.5, green: 1.0, blue: 0.75,};
        #[rustfmt::skip]
        assert_eq!(Colour{red: 0.5, green: 1.0, blue: 0.75}, c.clamped());
    }

    #[test]
    fn colours_can_be_multiplied_by_scalar() {
        #[rustfmt::skip]
        let c = Colour {red: 1.0, green: 2.0, blue: 3.0,};
        #[rustfmt::skip]
        assert_eq!(Colour{red: 2.0, green: 4.0, blue: 6.0}, &c * &2.0);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 2.0, green: 4.0, blue: 6.0}, c * &2.0);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 2.0, green: 4.0, blue: 6.0}, &c * 2.0);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 2.0, green: 4.0, blue: 6.0}, c * 2.0);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 0.5, green: 1.0, blue: 1.5}, &0.5 * &c);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 0.5, green: 1.0, blue: 1.5}, 0.5 * &c);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 0.5, green: 1.0, blue: 1.5}, &0.5 * c);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 0.5, green: 1.0, blue: 1.5}, 0.5 * c);
    }

    #[test]
    fn colours_can_be_multiplied_inplace_by_scalar() {
        #[rustfmt::skip]
        let mut c = Colour {red: 1.0, green: 2.0, blue: 3.0,};
        c *= 2.0;
        #[rustfmt::skip]
        assert_eq!(Colour{red: 2.0, green: 4.0, blue: 6.0}, c);
    }

    #[test]
    fn colours_can_be_divided_by_scalar() {
        #[rustfmt::skip]
        let c = Colour {red: 4.0, green: 8.0, blue: 12.0,};
        #[rustfmt::skip]
        assert_eq!(Colour{red: 2.0, green: 4.0, blue: 6.0}, &c / &2.0);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 2.0, green: 4.0, blue: 6.0}, c / &2.0);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 2.0, green: 4.0, blue: 6.0}, &c / 2.0);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 2.0, green: 4.0, blue: 6.0}, c / 2.0);
    }

    #[test]
    fn colours_can_be_divided_inplace_by_scalar() {
        #[rustfmt::skip]
        let mut c = Colour {red: 4.0, green: 8.0, blue: 12.0,};
        c /= 2.0;
        #[rustfmt::skip]
        assert_eq!(Colour{red: 2.0, green: 4.0, blue: 6.0}, c);
    }

    #[test]
    fn colours_can_be_added() {
        #[rustfmt::skip]
        let a = Colour {red: 1.0, green: 2.0, blue: 3.0,};
        #[rustfmt::skip]
        let b = Colour {red: 2.0, green: 3.0, blue: 5.0,};
        #[rustfmt::skip]
        assert_eq!(Colour{red: 3.0, green: 5.0, blue: 8.0}, &a + &b);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 3.0, green: 5.0, blue: 8.0}, a + &b);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 3.0, green: 5.0, blue: 8.0}, &a + b);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 3.0, green: 5.0, blue: 8.0}, a + b);
    }

    #[test]
    fn colours_can_be_added_inplace() {
        #[rustfmt::skip]
        let mut a = Colour {red: 1.0, green: 2.0, blue: 3.0,};
        #[rustfmt::skip]
        let b = Colour {red: 2.0, green: 3.0, blue: 5.0,};
        a += b;
        #[rustfmt::skip]
        assert_eq!(Colour{red: 3.0, green: 5.0, blue: 8.0}, a);
    }

    #[test]
    fn colours_can_be_substracted() {
        #[rustfmt::skip]
        let a = Colour {red: 1.0, green: 2.0, blue: 3.0,};
        #[rustfmt::skip]
        let b = Colour {red: 2.0, green: 3.0, blue: 5.0,};
        #[rustfmt::skip]
        assert_eq!(Colour{red: -1.0, green: -1.0, blue: -2.0}, &a - &b);
        #[rustfmt::skip]
        assert_eq!(Colour{red: -1.0, green: -1.0, blue: -2.0}, a - &b);
        #[rustfmt::skip]
        assert_eq!(Colour{red: -1.0, green: -1.0, blue: -2.0}, &a - b);
        #[rustfmt::skip]
        assert_eq!(Colour{red: -1.0, green: -1.0, blue: -2.0}, a - b);
    }

    #[test]
    fn colours_can_be_substracted_inplace() {
        #[rustfmt::skip]
        let mut a = Colour {red: 1.0, green: 2.0, blue: 3.0,};
        #[rustfmt::skip]
        let b = Colour {red: 2.0, green: 3.0, blue: 5.0,};
        a -= b;
        #[rustfmt::skip]
        assert_eq!(Colour{red: -1.0, green: -1.0, blue: -2.0}, a);
    }

    #[test]
    fn colours_can_be_multiplied() {
        #[rustfmt::skip]
        let a = Colour {red: 1.0, green: 2.0, blue: 3.0,};
        #[rustfmt::skip]
        let b = Colour {red: 2.0, green: 3.0, blue: 5.0,};
        #[rustfmt::skip]
        assert_eq!(Colour{red: 2.0, green: 6.0, blue: 15.0}, &a * &b);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 2.0, green: 6.0, blue: 15.0}, a * &b);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 2.0, green: 6.0, blue: 15.0}, &a * b);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 2.0, green: 6.0, blue: 15.0}, a * b);
    }

    #[test]
    fn colours_can_be_multiplied_inplace() {
        #[rustfmt::skip]
        let mut a = Colour {red: 1.0, green: 2.0, blue: 3.0,};
        #[rustfmt::skip]
        let b = Colour {red: 2.0, green: 3.0, blue: 5.0,};
        a *= b;
        #[rustfmt::skip]
        assert_eq!(Colour{red: 2.0, green: 6.0, blue: 15.0}, a);
    }

    #[test]
    fn colours_can_be_divided() {
        #[rustfmt::skip]
        let a = Colour {red: 1.0, green: 2.0, blue: 3.0,};
        #[rustfmt::skip]
        let b = Colour {red: 2.0, green: 3.0, blue: 5.0,};
        #[rustfmt::skip]
        assert_eq!(Colour{red: 0.5, green: 2.0 / 3.0, blue: 3.0 / 5.0}, &a / &b);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 0.5, green: 2.0 / 3.0, blue: 3.0 / 5.0}, a / &b);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 0.5, green: 2.0 / 3.0, blue: 3.0 / 5.0}, &a / b);
        #[rustfmt::skip]
        assert_eq!(Colour{red: 0.5, green: 2.0 / 3.0, blue: 3.0 / 5.0}, a / b);
    }

    #[test]
    fn colours_can_be_divided_inplace() {
        #[rustfmt::skip]
        let mut a = Colour {red: 1.0, green: 2.0, blue: 3.0,};
        #[rustfmt::skip]
        let b = Colour {red: 2.0, green: 3.0, blue: 5.0,};
        a /= b;
        #[rustfmt::skip]
        assert_eq!(Colour{red: 0.5, green: 2.0 / 3.0, blue: 3.0 / 5.0}, a);
    }

    #[test]
    fn converting_from_image_rgb() {
        #[rustfmt::skip]
        assert_eq!(
            Colour::from(Rgb([255u8, 0u8, 0u8])), Colour {red: 1.0, green: 0.0, blue: 0.0}
        );
        #[rustfmt::skip]
        assert_eq!(
            Colour::from(Rgb([0u8, 255u8, 0u8])), Colour {red: 0.0, green: 1.0, blue: 0.0}
        );
        #[rustfmt::skip]
        assert_eq!(
            Colour::from(Rgb([0u8, 0u8, 255u8])), Colour {red: 0.0, green: 0.0, blue: 1.0}
        );
    }

    #[test]
    fn converting_to_image_rgb() {
        #[rustfmt::skip]
        assert_eq!(
            Rgb([255u8, 0u8, 0u8]), Colour {red: 1.0, green: 0.0, blue: 0.0}.into()
        );
        #[rustfmt::skip]
        assert_eq!(
            Rgb([0u8, 255u8, 0u8]), Colour {red: 0.0, green: 2.0, blue: 0.0}.into()
        );
        #[rustfmt::skip]
        assert_eq!(
            Rgb([0u8, 0u8, 255u8]), Colour {red: 0.0, green: 0.0, blue: 5.0}.into()
        );
        #[rustfmt::skip]
        assert_eq!(
            Rgb([0u8, 63u8, 127u8]), Colour {red: 0.0, green: 0.25, blue: 0.5}.into()
        );
    }
}
