#[allow(unused)]
pub struct Struct {
    field1: i64,
    field2: i64,
    field3: i64,
    field4: i64,
    field5: i64,
    field6: i64,
    field7: i64,
    field8: i64,
    field9: i64,
    field10: i64,
    field11: i64,
    field12: i64,
    field13: i64,
    field14: i64,
    field15: i64,
    field16: i64,
    field17: i64,
    field18: i64,
    field19: i64,
    field20: i64,
    field21: i64,
    field22: i64,
    field23: i64,
    field24: i64,
    field25: i64,
    field26: i64,
    field27: i64,
    field28: i64,
    field29: i64,
    field30: i64,
    field31: i64,
    field32: i64,
    field33: i64,
    field34: i64,
    field35: i64,
    field36: i64,
    field37: i64,
    field38: i64,
    field39: i64,
    field40: i64,
    field41: i64,
    field42: i64,
    field43: i64,
    field44: i64,
    field45: i64,
    field46: i64,
    field47: i64,
    field48: i64,
    field49: i64,
    field50: i64,
    field51: i64,
    field52: i64,
    field53: i64,
    field54: i64,
    field55: i64,
    field56: i64,
    field57: i64,
    field58: i64,
    field59: i64,
    field60: i64,
    field61: i64,
    field62: i64,
    field63: i64,
    field64: i64,
}

use rustler::types::tuple;
use rustler::Decoder;
use rustler::Term;

impl Struct {
    pub fn decode<'a>(term: Term<'a>) -> Result<Self, ()> {
        let terms = tuple::get_tuple(term).unwrap();
        Ok(Struct {
            field1: match Decoder::decode(terms[1usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field2: match Decoder::decode(terms[2usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field3: match Decoder::decode(terms[3usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field4: match Decoder::decode(terms[4usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field5: match Decoder::decode(terms[5usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field6: match Decoder::decode(terms[6usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field7: match Decoder::decode(terms[7usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field8: match Decoder::decode(terms[8usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field9: match Decoder::decode(terms[9usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field10: match Decoder::decode(terms[10usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field11: match Decoder::decode(terms[11usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field12: match Decoder::decode(terms[12usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field13: match Decoder::decode(terms[13usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field14: match Decoder::decode(terms[14usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field15: match Decoder::decode(terms[15usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field16: match Decoder::decode(terms[16usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field17: match Decoder::decode(terms[17usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field18: match Decoder::decode(terms[18usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field19: match Decoder::decode(terms[19usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field20: match Decoder::decode(terms[20usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field21: match Decoder::decode(terms[21usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field22: match Decoder::decode(terms[22usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field23: match Decoder::decode(terms[23usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field24: match Decoder::decode(terms[24usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field25: match Decoder::decode(terms[25usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field26: match Decoder::decode(terms[26usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field27: match Decoder::decode(terms[27usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field28: match Decoder::decode(terms[28usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field29: match Decoder::decode(terms[29usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field30: match Decoder::decode(terms[30usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field31: match Decoder::decode(terms[31usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field32: match Decoder::decode(terms[32usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field33: match Decoder::decode(terms[33usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field34: match Decoder::decode(terms[34usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field35: match Decoder::decode(terms[35usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field36: match Decoder::decode(terms[36usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field37: match Decoder::decode(terms[37usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field38: match Decoder::decode(terms[38usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field39: match Decoder::decode(terms[39usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field40: match Decoder::decode(terms[40usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field41: match Decoder::decode(terms[41usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field42: match Decoder::decode(terms[42usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field43: match Decoder::decode(terms[43usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field44: match Decoder::decode(terms[44usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field45: match Decoder::decode(terms[45usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field46: match Decoder::decode(terms[46usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field47: match Decoder::decode(terms[47usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field48: match Decoder::decode(terms[48usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field49: match Decoder::decode(terms[49usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field50: match Decoder::decode(terms[50usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field51: match Decoder::decode(terms[51usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field52: match Decoder::decode(terms[52usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field53: match Decoder::decode(terms[53usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field54: match Decoder::decode(terms[54usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field55: match Decoder::decode(terms[55usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field56: match Decoder::decode(terms[56usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field57: match Decoder::decode(terms[57usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field58: match Decoder::decode(terms[58usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field59: match Decoder::decode(terms[59usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field60: match Decoder::decode(terms[60usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field61: match Decoder::decode(terms[61usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field62: match Decoder::decode(terms[62usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field63: match Decoder::decode(terms[63usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
            field64: match Decoder::decode(terms[64usize]) {
                Err(_) => return Err(()),
                Ok(value) => value,
            },
        })
    }
}
