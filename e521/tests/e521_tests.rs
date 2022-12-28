#[cfg(test)]
mod tests {
    
    use e521::curve::e521::e521::{get_e521_gen_point, get_e521_id_point, sec_mul};
    use num_bigint::{BigInt};
    
    #[test]
    pub fn test_zero_times_g() {
        let point  = get_e521_gen_point(false);
        let s = BigInt::from(0);
        let result = sec_mul(&s, &point);
        let id_point = get_e521_id_point();
        assert_eq!(result.x, id_point.x);
        assert_eq!(result.y, id_point.y);
    }

    #[test]
    pub fn test_g_times_one() {
        let point  = get_e521_gen_point(false);
        let s = BigInt::from(1);
        let result = sec_mul(&s, &point);
        assert_eq!(result.x, point.x);
        assert_eq!(result.y, point.y);
    }
    

}