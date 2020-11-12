
use std::str::FromStr;

use num_bigint::{BigInt};


pub fn multiply(xstr: &str, ystr: &str) -> BigInt {
    let x_len = xstr.len() as u32;
    let y_len = ystr.len() as u32;

    let x = BigInt::from_str(xstr).unwrap();
    let y = BigInt::from_str(ystr).unwrap();


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


    let astr = a.to_string();
    let bstr = b.to_string();
    let cstr = c.to_string();
    let dstr = d.to_string();


    let ac = multiply(&astr, &cstr);
    let bd = multiply(&bstr, &dstr);
    let ad = multiply(&astr, &dstr);
    let bc = multiply(&bstr, &cstr);

    return 10_u128.pow((half_x_len + half_y_len) as u32) * ac + 10_u128.pow((half_x_len) as u32) * ad + 10_u128.pow((half_y_len) as u32) * bc + bd;
}

#[cfg(test)]
mod tests {
    use crate::{multiply};

    use num_bigint::BigInt;
    use std::str::FromStr;


    #[test]
    fn multiply_works() {
        let a = BigInt::from_str("1234567891011333456756333333333333335555555").unwrap();
        let b = BigInt::from_str("12345678910113333453456").unwrap();
        assert_eq!(BigInt::from(a.clone() * b.clone()).to_string(), multiply(&a.to_string(), &b.to_string()).to_string());
    }


}
