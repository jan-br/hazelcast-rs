use std::str::FromStr;
use num_bigint::BigInt;
use num_traits::identities::Zero;
use num_traits::Signed;

pub struct BigDecimal {
  pub unscaled_value: BigInt,
  pub scale: i32,
}

impl BigDecimal {
  pub const MAX_SAFE_INTEGER: i64 = 9007199254740991;
  pub const MAX_EXPONENT_DIGITS: i64 = Self::MAX_SAFE_INTEGER;

  pub fn new(unscaled_value: BigInt, scale: i32) -> Self {
    Self {
      unscaled_value,
      scale,
    }
  }

  pub fn parse_exp(exp_string: String, mut offset: usize, mut len: i64) -> i32 {
    let mut exp = 0;
    offset += 1;
    let mut c = exp_string.chars().nth(offset).unwrap();
    len -= 1;
    let is_negative = c == '-';
    if is_negative || c == '+' {
      offset += 1;
      c = exp_string.chars().nth(offset).unwrap();
      len -= 1;
    }
    if len <= 0 {
      panic!("Empty exponent");
    }
    while len > BigDecimal::MAX_EXPONENT_DIGITS && c == '0' {
      offset += 1;
      c = exp_string.chars().nth(offset).unwrap();
      len -= 1;
    }

    if len > BigDecimal::MAX_EXPONENT_DIGITS {
      panic!("Exponent too large");
    }
    while len > 0 {
      let mut v = 0;
      if c >= '0' && c <= '9' {
        v = c as i32;
      } else {
        panic!("Invalid digit");
      }
      exp = exp * 10 + v;
      if len == 1 {
        break;
      }
      offset += 1;
      c = exp_string.chars().nth(offset).unwrap();
      len -= 1;
    }

    if is_negative {
      exp = -exp;
    }
    exp
  }

  pub fn from_string(value: String) -> BigDecimal {
    let mut offset = 0;
    let mut len = value.len() as i64;
    let mut precision = 0;
    let mut scale = 0;
    let mut unscaled_value = BigInt::zero();

    let mut isneg = false;
    if value.chars().nth(offset).unwrap() == '-' {
      isneg = true;
      offset += 1;
      len -= 1;
    } else if value.chars().nth(offset).unwrap() == '+' {
      offset += 1;
      len -= 1;
    }
    let mut dot = false;
    let mut exp = 0;
    let mut c = ' ';
    let mut idx = 0;
    let mut coeff = vec![];
    while len > 0 {
      c = value.chars().nth(offset).unwrap();
      if c >= '0' && c <= '9' {
        if c == '0' {
          if precision == 0 {
            coeff[idx] = c;
            precision = 1;
          } else if idx != 0 {
            idx += 1;
            coeff[idx] = c;
            precision += 1;
          }
        } else {
          if precision != 1 || idx != 0 {
            precision += 1;
          }
          idx += 1;
          coeff[idx] = c;
        }
        if dot {
          scale += 1;
        }
        offset += 1;
        len -= 1;
        continue;
      }
      if c == '.' {
        if dot {
          panic!("Multiple dots");
        }
        dot = true;
        offset += 1;
        len -= 1;
        continue;
      }

      if (c != 'e') && (c != 'E') {
        panic!("Invalid exponent character");
      }
      exp = BigDecimal::parse_exp(value, offset, len);

      //todo: Add exponent overflow check

      break;
    }
    if precision == 0 {
      panic!("No digits");
    }
    if exp != 0 {
      scale = BigDecimal::adjust_scale(scale, exp);
    }
    let string_value: String = coeff.into_iter().collect();
    if isneg {
      unscaled_value = BigInt::from_str(format!("-{}", string_value).as_str()).unwrap();
    } else {
      unscaled_value = BigInt::from_str(string_value.as_str()).unwrap();
    }
    BigDecimal::new(unscaled_value, scale)
  }

  pub fn adjust_scale(scale: i32, exp: i32) -> i32 {
    let mut adjustedScale = scale - exp;
    //todo: Add scale overflow check
    adjustedScale
  }

  pub fn big_int_abs(val: BigInt) -> BigInt {
    val.abs()
  }


  pub fn signum(&self) -> i32 {
    if self.unscaled_value > BigInt::zero() {
      1
    } else if self.unscaled_value < BigInt::zero() {
      -1
    } else {
      0
    }
  }

  pub fn to_string(&self) -> String {
    if self.scale == 0 {
      return self.unscaled_value.to_string();
    }
    if self.scale < 0 {
      if self.signum() == 0 {
        return "0".to_string();
      }
      let trailing_zeros = -self.scale;
      let mut str = "".to_string();
      str += self.unscaled_value.to_string().as_str();
      for _ in 0..trailing_zeros {
        str += "0";
      }
      return str;
    }
    let digits_string = BigDecimal::big_int_abs(self.unscaled_value.clone()).to_string();
    BigDecimal::get_value_string(self.signum(), digits_string, self.scale)
  }

  pub fn get_value_string(signum: i32, digits_string: String, scale: i32) -> String {
    let mut buf = "".to_string();
    let insertion_point = digits_string.len() as i32 - scale;
    if insertion_point == 0 {
      return format!("{}{}", if signum < 0 {
        "-0.".to_string()
      } else {
        "0.".to_string()
      }, digits_string).to_string();
    } else if insertion_point > 0 {
      buf = format!("{}.{}", digits_string[0..insertion_point as usize].to_string(), &digits_string[insertion_point as usize..]).to_string();
      if signum < 0 {
        buf = format!("-{}", buf).to_string();
      }
    } else {
      if signum < 0 {
        buf = "-0.".to_string();
      } else {
        buf = "0.".to_string();
      }
      for i in 0..-insertion_point {
        buf += "0";
      }
      buf += digits_string.as_str();
    }
    buf
  }
}