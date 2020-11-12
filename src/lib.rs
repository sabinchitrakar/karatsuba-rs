use std::str::FromStr;
use num_bigint::{BigInt};


pub fn multiply(x: BigInt, y: BigInt) -> BigInt {
    let x_len = x.to_string().len() as u32;
    let y_len = y.to_string().len() as u32;


    if x_len == 1 && y_len == 1 {
        return x * y;
    }
    let half_x_len = x_len / 2;
    let half_y_len = y_len / 2;
    let x_multiplier = 10_u128.pow(half_x_len as u32);
    let y_multiplier = 10_u128.pow(half_y_len as u32);


    let b = x.clone() % (x_multiplier);
    let d = y.clone() % (y_multiplier);
    let a = x.clone() / (x_multiplier);
    let c = y.clone() / (y_multiplier);


    let ac = multiply(a.clone(), c.clone());
    let bd = multiply(b.clone(), d.clone());
    let ad = multiply(a.clone(), d.clone());
    let bc = multiply(b.clone(), c.clone());

    return 10_u128.pow((half_x_len + half_y_len) as u32) * ac + 10_u128.pow((half_x_len) as u32) * ad + 10_u128.pow((half_y_len) as u32) * bc + bd;
}

pub fn multiply_string(x_str:&str, y_str:&str) -> BigInt {
    let x=BigInt::from_str(x_str).unwrap();
    let y=BigInt::from_str(y_str).unwrap();
    return multiply(x,y);

}

#[cfg(test)]
mod tests {
    use crate::{multiply, multiply_string};

    use num_bigint::BigInt;
    use std::str::FromStr;


    #[test]
    fn multiply_works() {
        let a = BigInt::from_str("1234567891011333456756333333333333335555555").unwrap();
        let b = BigInt::from_str("12345678910113333453456").unwrap();
        assert_eq!(BigInt::from(a.clone() * b.clone()).to_string(), multiply(a, b).to_string());
    }
    #[test]
    fn multiply_string_works() {
        let a_str = "1234567891011333456756333333333333335555555";
        let b_str = "12345678910113333453456";
        let a=BigInt::from_str(a_str).unwrap();
        let b=BigInt::from_str(b_str).unwrap();
        assert_eq!(BigInt::from(a.clone() * b.clone()).to_string(), multiply_string(&a_str, &b_str).to_string());
    }
}
