#[cfg(test)]
mod norm {
    use crate::base_structs::vector::Vector;

    #[test]
    fn test_norm() {
        let mut u = Vector::from([0., 0., 0.]);
        assert_eq!(u.norm_1(), 0.0);
        assert_eq!(u.norm(), 0.0);
        assert_eq!(u.norm_inf(), 0.0);
        let mut u = Vector::from([1., 2., 3.]);
        assert_eq!(u.norm_1(), 6.0);
        assert_eq!(u.norm(), f32::sqrt(14.0));
        assert_eq!(u.norm_inf(), 3.0);
        let mut u = Vector::from([-1., -2.]);
        assert_eq!(u.norm_1(), 3.0);
        assert_eq!(u.norm(), f32::sqrt(5.0));
        assert_eq!(u.norm_inf(), 2.0);
    }
}