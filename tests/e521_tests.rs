#[cfg(test)]
mod tests {
    
    use e521::curve::e521::e521::{get_e521_gen_point, get_e521_id_point, sec_mul, negate_point, add_points, e521_equals, get_e521_point};
    use num_bigint::{BigInt};
    
    #[test]
    // 0 * G = neutral point
    fn test_zero_times_g() {
        let point  = get_e521_gen_point(false);
        let s = BigInt::from(0);
        let result = sec_mul(s, point);
        let id_point = get_e521_id_point();
        assert!(e521_equals(&id_point, &result), "points are not equal, check addition function")
        
    }

    // G * 1 = G
    #[test]
    fn test_g_times_one() {
        
        let point  = get_e521_gen_point(false);
        let s = BigInt::from(1);
        let g = get_e521_gen_point(false);
        let result = sec_mul(s, point);
        
        assert!(e521_equals(&g, &result), "points are not equal, check mul and add functions")
    }

    // G + (-G) = 1
    #[test]
    fn test_g_plus_neg_g() {

        let g = get_e521_gen_point(false);
        let neg_g = negate_point(&g);

        let id = get_e521_id_point();
        let sum = add_points(&g, &neg_g);
        assert!(e521_equals(&sum, &id), "points are not equal, check mul and add functions")
        
    }

    #[test]
    // 2 * G = G + G
    fn test_two_times_g() {

        let g = get_e521_gen_point(false);
        let s = BigInt::from(2);

        let product = sec_mul(s, g);

        let g = get_e521_gen_point(false);
        let sum = add_points(&g, &g);

        assert!(e521_equals(&sum, &product), "points are not equal, check mul and add functions")
    }

    #[test]
    // 4 * G = 2 * (2 * G)
    fn test_four_g() {

        let four_g = sec_mul(BigInt::from(4), get_e521_gen_point(false));
        let two_times_two_g = sec_mul(BigInt::from(2), sec_mul(BigInt::from(2), get_e521_gen_point(false)));
        
        assert!(e521_equals(&four_g, &two_times_two_g))
    }

    #[test]
    //4 * G != neutral point
    fn test_four_g_not_id() {



    }


}