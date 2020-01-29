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

impl Struct {
    pub fn decode<'a>(term: ::rustler::Term<'a>) -> Result<Self, ::rustler::Error> {
        let terms = ::rustler::types::tuple::get_tuple(term)?;
        Ok(Struct {
            field1: match ::rustler::Decoder::decode(terms[1usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field1 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field2: match ::rustler::Decoder::decode(terms[2usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field2 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field3: match ::rustler::Decoder::decode(terms[3usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field3 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field4: match ::rustler::Decoder::decode(terms[4usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field4 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field5: match ::rustler::Decoder::decode(terms[5usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field5 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field6: match ::rustler::Decoder::decode(terms[6usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field6 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field7: match ::rustler::Decoder::decode(terms[7usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field7 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field8: match ::rustler::Decoder::decode(terms[8usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field8 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field9: match ::rustler::Decoder::decode(terms[9usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field9 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field10: match ::rustler::Decoder::decode(terms[10usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field10 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field11: match ::rustler::Decoder::decode(terms[11usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field11 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field12: match ::rustler::Decoder::decode(terms[12usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field12 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field13: match ::rustler::Decoder::decode(terms[13usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field13 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field14: match ::rustler::Decoder::decode(terms[14usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field14 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field15: match ::rustler::Decoder::decode(terms[15usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field15 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field16: match ::rustler::Decoder::decode(terms[16usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field16 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field17: match ::rustler::Decoder::decode(terms[17usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field17 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field18: match ::rustler::Decoder::decode(terms[18usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field18 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field19: match ::rustler::Decoder::decode(terms[19usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field19 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field20: match ::rustler::Decoder::decode(terms[20usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field20 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field21: match ::rustler::Decoder::decode(terms[21usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field21 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field22: match ::rustler::Decoder::decode(terms[22usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field22 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field23: match ::rustler::Decoder::decode(terms[23usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field23 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field24: match ::rustler::Decoder::decode(terms[24usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field24 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field25: match ::rustler::Decoder::decode(terms[25usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field25 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field26: match ::rustler::Decoder::decode(terms[26usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field26 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field27: match ::rustler::Decoder::decode(terms[27usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field27 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field28: match ::rustler::Decoder::decode(terms[28usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field28 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field29: match ::rustler::Decoder::decode(terms[29usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field29 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field30: match ::rustler::Decoder::decode(terms[30usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field30 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field31: match ::rustler::Decoder::decode(terms[31usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field31 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field32: match ::rustler::Decoder::decode(terms[32usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field32 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field33: match ::rustler::Decoder::decode(terms[33usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field33 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field34: match ::rustler::Decoder::decode(terms[34usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field34 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field35: match ::rustler::Decoder::decode(terms[35usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field35 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field36: match ::rustler::Decoder::decode(terms[36usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field36 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field37: match ::rustler::Decoder::decode(terms[37usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field37 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field38: match ::rustler::Decoder::decode(terms[38usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field38 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field39: match ::rustler::Decoder::decode(terms[39usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field39 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field40: match ::rustler::Decoder::decode(terms[40usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field40 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field41: match ::rustler::Decoder::decode(terms[41usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field41 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field42: match ::rustler::Decoder::decode(terms[42usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field42 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field43: match ::rustler::Decoder::decode(terms[43usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field43 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field44: match ::rustler::Decoder::decode(terms[44usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field44 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field45: match ::rustler::Decoder::decode(terms[45usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field45 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field46: match ::rustler::Decoder::decode(terms[46usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field46 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field47: match ::rustler::Decoder::decode(terms[47usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field47 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field48: match ::rustler::Decoder::decode(terms[48usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field48 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field49: match ::rustler::Decoder::decode(terms[49usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field49 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field50: match ::rustler::Decoder::decode(terms[50usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field50 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field51: match ::rustler::Decoder::decode(terms[51usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field51 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field52: match ::rustler::Decoder::decode(terms[52usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field52 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field53: match ::rustler::Decoder::decode(terms[53usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field53 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field54: match ::rustler::Decoder::decode(terms[54usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field54 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field55: match ::rustler::Decoder::decode(terms[55usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field55 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field56: match ::rustler::Decoder::decode(terms[56usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field56 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field57: match ::rustler::Decoder::decode(terms[57usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field57 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field58: match ::rustler::Decoder::decode(terms[58usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field58 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field59: match ::rustler::Decoder::decode(terms[59usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field59 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field60: match ::rustler::Decoder::decode(terms[60usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field60 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field61: match ::rustler::Decoder::decode(terms[61usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field61 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field62: match ::rustler::Decoder::decode(terms[62usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field62 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field63: match ::rustler::Decoder::decode(terms[63usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field63 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
            field64: match ::rustler::Decoder::decode(terms[64usize]) {
                Err(_) => {
                    return Err(::rustler::Error::RaiseTerm(Box::new(
                        "Could not decode field field64 on Record Struct",
                    )))
                }
                Ok(value) => value,
            },
        })
    }
}
