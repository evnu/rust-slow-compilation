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
    pub fn decode<'a>(term: Term<'a>) -> Result<Self, &'static str> {
        let terms = tuple::get_tuple(term).unwrap();
        Ok(Struct {
            field1: Decoder::decode(terms[1usize])
                .map_err(|_| "Could not decode field field1 on Record Struct")?,
            field2: Decoder::decode(terms[2usize])
                .map_err(|_| "Could not decode field field2 on Record Struct")?,
            field3: Decoder::decode(terms[3usize])
                .map_err(|_| "Could not decode field field3 on Record Struct")?,
            field4: Decoder::decode(terms[4usize])
                .map_err(|_| "Could not decode field field4 on Record Struct")?,
            field5: Decoder::decode(terms[5usize])
                .map_err(|_| "Could not decode field field5 on Record Struct")?,
            field6: Decoder::decode(terms[6usize])
                .map_err(|_| "Could not decode field field6 on Record Struct")?,
            field7: Decoder::decode(terms[7usize])
                .map_err(|_| "Could not decode field field7 on Record Struct")?,
            field8: Decoder::decode(terms[8usize])
                .map_err(|_| "Could not decode field field8 on Record Struct")?,
            field9: Decoder::decode(terms[9usize])
                .map_err(|_| "Could not decode field field9 on Record Struct")?,
            field10: Decoder::decode(terms[10usize])
                .map_err(|_| "Could not decode field field10 on Record Struct")?,
            field11: Decoder::decode(terms[11usize])
                .map_err(|_| "Could not decode field field11 on Record Struct")?,
            field12: Decoder::decode(terms[12usize])
                .map_err(|_| "Could not decode field field12 on Record Struct")?,
            field13: Decoder::decode(terms[13usize])
                .map_err(|_| "Could not decode field field13 on Record Struct")?,
            field14: Decoder::decode(terms[14usize])
                .map_err(|_| "Could not decode field field14 on Record Struct")?,
            field15: Decoder::decode(terms[15usize])
                .map_err(|_| "Could not decode field field15 on Record Struct")?,
            field16: Decoder::decode(terms[16usize])
                .map_err(|_| "Could not decode field field16 on Record Struct")?,
            field17: Decoder::decode(terms[17usize])
                .map_err(|_| "Could not decode field field17 on Record Struct")?,
            field18: Decoder::decode(terms[18usize])
                .map_err(|_| "Could not decode field field18 on Record Struct")?,
            field19: Decoder::decode(terms[19usize])
                .map_err(|_| "Could not decode field field19 on Record Struct")?,
            field20: Decoder::decode(terms[20usize])
                .map_err(|_| "Could not decode field field20 on Record Struct")?,
            field21: Decoder::decode(terms[21usize])
                .map_err(|_| "Could not decode field field21 on Record Struct")?,
            field22: Decoder::decode(terms[22usize])
                .map_err(|_| "Could not decode field field22 on Record Struct")?,
            field23: Decoder::decode(terms[23usize])
                .map_err(|_| "Could not decode field field23 on Record Struct")?,
            field24: Decoder::decode(terms[24usize])
                .map_err(|_| "Could not decode field field24 on Record Struct")?,
            field25: Decoder::decode(terms[25usize])
                .map_err(|_| "Could not decode field field25 on Record Struct")?,
            field26: Decoder::decode(terms[26usize])
                .map_err(|_| "Could not decode field field26 on Record Struct")?,
            field27: Decoder::decode(terms[27usize])
                .map_err(|_| "Could not decode field field27 on Record Struct")?,
            field28: Decoder::decode(terms[28usize])
                .map_err(|_| "Could not decode field field28 on Record Struct")?,
            field29: Decoder::decode(terms[29usize])
                .map_err(|_| "Could not decode field field29 on Record Struct")?,
            field30: Decoder::decode(terms[30usize])
                .map_err(|_| "Could not decode field field30 on Record Struct")?,
            field31: Decoder::decode(terms[31usize])
                .map_err(|_| "Could not decode field field31 on Record Struct")?,
            field32: Decoder::decode(terms[32usize])
                .map_err(|_| "Could not decode field field32 on Record Struct")?,
            field33: Decoder::decode(terms[33usize])
                .map_err(|_| "Could not decode field field33 on Record Struct")?,
            field34: Decoder::decode(terms[34usize])
                .map_err(|_| "Could not decode field field34 on Record Struct")?,
            field35: Decoder::decode(terms[35usize])
                .map_err(|_| "Could not decode field field35 on Record Struct")?,
            field36: Decoder::decode(terms[36usize])
                .map_err(|_| "Could not decode field field36 on Record Struct")?,
            field37: Decoder::decode(terms[37usize])
                .map_err(|_| "Could not decode field field37 on Record Struct")?,
            field38: Decoder::decode(terms[38usize])
                .map_err(|_| "Could not decode field field38 on Record Struct")?,
            field39: Decoder::decode(terms[39usize])
                .map_err(|_| "Could not decode field field39 on Record Struct")?,
            field40: Decoder::decode(terms[40usize])
                .map_err(|_| "Could not decode field field40 on Record Struct")?,
            field41: Decoder::decode(terms[41usize])
                .map_err(|_| "Could not decode field field41 on Record Struct")?,
            field42: Decoder::decode(terms[42usize])
                .map_err(|_| "Could not decode field field42 on Record Struct")?,
            field43: Decoder::decode(terms[43usize])
                .map_err(|_| "Could not decode field field43 on Record Struct")?,
            field44: Decoder::decode(terms[44usize])
                .map_err(|_| "Could not decode field field44 on Record Struct")?,
            field45: Decoder::decode(terms[45usize])
                .map_err(|_| "Could not decode field field45 on Record Struct")?,
            field46: Decoder::decode(terms[46usize])
                .map_err(|_| "Could not decode field field46 on Record Struct")?,
            field47: Decoder::decode(terms[47usize])
                .map_err(|_| "Could not decode field field47 on Record Struct")?,
            field48: Decoder::decode(terms[48usize])
                .map_err(|_| "Could not decode field field48 on Record Struct")?,
            field49: Decoder::decode(terms[49usize])
                .map_err(|_| "Could not decode field field49 on Record Struct")?,
            field50: Decoder::decode(terms[50usize])
                .map_err(|_| "Could not decode field field50 on Record Struct")?,
            field51: Decoder::decode(terms[51usize])
                .map_err(|_| "Could not decode field field51 on Record Struct")?,
            field52: Decoder::decode(terms[52usize])
                .map_err(|_| "Could not decode field field52 on Record Struct")?,
            field53: Decoder::decode(terms[53usize])
                .map_err(|_| "Could not decode field field53 on Record Struct")?,
            field54: Decoder::decode(terms[54usize])
                .map_err(|_| "Could not decode field field54 on Record Struct")?,
            field55: Decoder::decode(terms[55usize])
                .map_err(|_| "Could not decode field field55 on Record Struct")?,
            field56: Decoder::decode(terms[56usize])
                .map_err(|_| "Could not decode field field56 on Record Struct")?,
            field57: Decoder::decode(terms[57usize])
                .map_err(|_| "Could not decode field field57 on Record Struct")?,
            field58: Decoder::decode(terms[58usize])
                .map_err(|_| "Could not decode field field58 on Record Struct")?,
            field59: Decoder::decode(terms[59usize])
                .map_err(|_| "Could not decode field field59 on Record Struct")?,
            field60: Decoder::decode(terms[60usize])
                .map_err(|_| "Could not decode field field60 on Record Struct")?,
            field61: Decoder::decode(terms[61usize])
                .map_err(|_| "Could not decode field field61 on Record Struct")?,
            field62: Decoder::decode(terms[62usize])
                .map_err(|_| "Could not decode field field62 on Record Struct")?,
            field63: Decoder::decode(terms[63usize])
                .map_err(|_| "Could not decode field field63 on Record Struct")?,
            field64: Decoder::decode(terms[64usize])
                .map_err(|_| "Could not decode field field64 on Record Struct")?,
        })
    }
}
