```
use karatsuba_rs::multiply;
use num_bigint::{BigInt};

let a = BigInt::from_str("1234567891011333456756333333333333335555555").unwrap();
let b = BigInt::from_str("12345678910113333453456").unwrap();
assert_eq!(BigInt::from(a.clone() * b.clone()).to_string(), multiply(a, b).to_string());
```