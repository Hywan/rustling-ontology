use rustling::*;
use dimension::*;
use helpers;
use moment::{Weekday, Grain, PeriodComp};

pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("segundo (cycle)",
        b.reg(r#""segundos?""#)?,
        |_| CycleValue::new(Grain::Second)
    );
    b.rule_1("minutos (cycle)",
        b.reg(r#"minutos?"#)?,
        |_| CycleValue::new(Grain::Minute)
    );
    b.rule_1("hora (cycle)",
        b.reg(r#"horas?"#)?,
        |_| CycleValue::new(Grain::Hour)
    );
    b.rule_1("dia (cycle)",
        b.reg(r#"d(?:í|i)as?"#)?,
        |_| CycleValue::new(Grain::Day)
    );
    b.rule_1("semana (cycle)",
        b.reg(r#"semanas?"#)?,
        |_| CycleValue::new(Grain::Week)
    );
    b.rule_1("mes (cycle)",
        b.reg(r#"mes(?:es)?"#)?,
        |_| CycleValue::new(Grain::Month)
    );
    b.rule_1("trimestre (cycle)",
        b.reg(r#"trimestres?"#)?,
        |_| CycleValue::new(Grain::Quarter)
    );
    b.rule_1("año (cycle)",
        b.reg(r#"a(?:n|ñ)os?"#)?,
        |_| CycleValue::new(Grain::Year)
    );
    b.rule_2("este|en un <cycle>",
        b.reg(r#"(?:est(?:e|a|os)|en (?:el|los|la|las) ?)"#)?,
        cycle_check!(),
        |_, cycle| helpers::cycle_nth(cycle.value().grain, 0)
    );
    b.rule_3("la <cycle> pasado",
        b.reg(r#"(?:el|los|la|las) ?"#)?,
        cycle_check!(),
        b.reg(r#"pasad(?:a|o)s?|[u|ú]ltim[a|o]s?"#)?,
        |_, cycle, _| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_3("la pasado <cycle>",
        b.reg(r#"(?:el|los|la|las) ?"#)?,
        b.reg(r#"pasad(?:a|o)s?|[u|ú]ltim[a|o]s?"#)?,
        cycle_check!(),
        |_, _, cycle| helpers::cycle_nth(cycle.value().grain, -1)
    );
    b.rule_3("el <cycle> (proximo|que viene)",
        b.reg(r#"(?:el|los|la|las) ?"#)?,
        cycle_check!(),
        b.reg(r#"(?:pr(?:ó|o)xim(?:o|a)s?|que vienen?|siguientes?)"#)?,
        |_, cycle, _| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_3("el proximo <cycle>",
        b.reg(r#"(?:el|los|la|las) ?"#)?,
        b.reg(r#"pr(?:ó|o)xim(?:o|a)s?|siguientes?"#)?,
        cycle_check!(),
        |_, _, cycle| helpers::cycle_nth(cycle.value().grain, 1)
    );
    b.rule_4("el <cycle> proximo|que viene <time>",
        b.reg(r#"(?:el|los|la|las)"#)?,
        cycle_check!(),
        b.reg(r#"(?:pr(?:ó|o)xim(?:o|a)s?|que vienen?|siguientes?)"#)?,
        time_check!(),
        |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, 1, time.value())
    );
    b.rule_4("el <cycle> antes <time>",
        b.reg(r#"l[ea']? ?"#)?,
        cycle_check!(),
        b.reg(r#"antes de"#)?,
        time_check!(),
        |_, cycle, _, time| helpers::cycle_nth_after(cycle.value().grain, -1, time.value())
    );
    b.rule_3("pasados n <cycle>",
        b.reg(r#"pasad(?:a|o)s?"#)?,
        integer_check!(2, 9999),
        cycle_check!(),
        |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("n pasados <cycle>",
        integer_check!(2, 9999),
        b.reg(r#"pasad(?:a|o)s?"#)?,
        cycle_check!(),
        |integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("proximas n <cycle>",
        b.reg(r#"pr(?:ó|o)xim(?:o|a)s?"#)?,
        integer_check!(2, 9999),
        cycle_check!(),
        |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_3("n proximas <cycle>",
        integer_check!(2, 9999),
        b.reg(r#"pr(?:ó|o)xim(?:o|a)s?"#)?,
        cycle_check!(),
        |integer, _, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_3("n <cycle> (proximo|que viene)",
        integer_check!(2, 9999),
        cycle_check!(),
        b.reg(r#"(?:pr(?:ó|o)xim(?:o|a)s?|que vienen?|siguientes?)"#)?,
        |integer, cycle, _| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_2("<ordinal> quarter",
        ordinal_check!(),
        cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
        |ordinal, _| helpers::cycle_nth_after(
                Grain::Quarter, 
                ordinal.value().value - 1, 
                &helpers::cycle_nth(Grain::Year, 0)?
        )
    );
    b.rule_4("<ordinal> quarter <year>",
        ordinal_check!(),
        cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
        b.reg(r#"del? ?"#)?,
        time_check!(),
        |ordinal, _, _, time| helpers::cycle_nth_after(
                Grain::Quarter, 
                ordinal.value().value - 1, 
                time.value()
        )
    );
    Ok(())
}

pub fn rules_temperature(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("number as temp", number_check!(), |a| {
        Ok(TemperatureValue {
               value: a.value().value(),
               unit: None,
               latent: true,
           })
    });
    b.rule_2("<latent temp> temp",
             temperature_check!(),
             b.reg(r#"(?:grados?)|°"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                        value: a.value().value,
                        unit: Some("degree"),
                        latent: false,
                    })
             });
    b.rule_2("<temp> Celcius",
             temperature_check!(),
             b.reg(r#"(?:cent(?:i|í)grados?|c(?:el[cs]?(?:ius)?)?\.?)"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                        value: a.value().value,
                        unit: Some("celsius"),
                        latent: false,
                    })
             });
    b.rule_2("<temp> Fahrenheit",
             temperature_check!(),
             b.reg(r#"f(?:ah?reh?n(?:h?eit)?)?\.?"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                        value: a.value().value,
                        unit: Some("fahrenheit"),
                        latent: false,
                    })
             });
    b.rule_2("<latent temp> temp bajo cero",
             temperature_check!(),
             b.reg(r#"(?:(?:grados?)|°)?(?: bajo cero)"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                        value: -1.0 * a.value().value,
                        latent: false,
                        ..*a.value()
                    })
             });
    Ok(())
}

pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1(
            "number (0..15)",
            b.reg(r#"((?:c|z)ero|un(?:o|a)?|dos|tr(?:é|e)s|cuatro|cinco|s(?:e|é)is|siete|ocho|nueve|die(?:z|s)|once|doce|trece|catorce|quince)"#)?,
            |text_match| {
                let value = match text_match.group(1).as_ref() {
                    "cero" => 0,
                    "zero" => 0, 
                    "un" => 1, 
                    "uno" => 1,
                    "una" => 1,
                    "dos" => 2,
                    "tres" => 3,
                    "trés" => 3,
                    "cuatro" => 4,
                    "cinco" => 5,
                    "seis" => 6,
                    "séis" => 6,
                    "siete" => 7,
                    "ocho" => 8,
                    "nueve" => 9,
                    "diez" => 10,
                    "dies" => 10,
                    "once" => 11,
                    "doce" => 12,
                    "trece" => 13,
                    "catorce" => 14,
                    "quince" => 15,
                     _ => panic!("Unknow match"),
                };
                IntegerValue::new(value)
             }
        );
    b.rule_1("number (20..90)",
             b.reg(r#"(veinte|treinta|cuarenta|cincuenta|sesenta|setenta|ochenta|noventa)"#)?,
             |text_match| {
        let value = match text_match.group(1).as_ref() {
            "veinte" => 20, 
            "treinta" => 30,
            "cuarenta" => 40,
            "cincuenta" => 50,
            "sesenta" => 60,
            "setenta" => 70,
            "ochenta" => 80,
            "noventa" => 90,
            _ => panic!("Unknow match"),
        };
        IntegerValue::new(value)
    });
    b.rule_3("number (16..19)",
             integer_check!(0, 10),
             b.reg(r#"y"#)?,
             integer_check!(6, 9),
             |_, _, a| IntegerValue::new(a.value().value + 10));
    b.rule_3("number (21..29 31..39 41..49 51..59 61..69 71..79 81..89 91..99)",
             integer_check!(20, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             b.reg(r#"y"#)?,
             integer_check!(1, 9),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_1(
            "number (16..19 21..29)",
            b.reg(r#"(die(?:c|s)is(?:é|e)is|diecisiete|dieciocho|diecinueve|veintiun(?:o|a)|veintidos|veintitr(?:é|e)s|veinticuatro|veinticinco|veintis(?:é|e)is|veintisiete|veintiocho|veintinueve)"#)?,
            |text_match| {
                let value = match text_match.group(1).as_ref() {
                    "dieciseis" => 16, 
                    "diesiseis" => 16,
                    "diesiséis" => 16,
                    "dieciséis" => 16,
                    "diecisiete" => 17,
                    "dieciocho" => 18,
                    "diecinueve" => 19,
                    "veintiuno" => 21,
                    "veintiuna" => 21, 
                    "veintidos" => 22,
                    "veintitres" => 23,
                    "veintitrés" => 23,
                    "veinticuatro" => 24,
                    "veinticinco" => 25,
                    "veintiseis" => 26,
                    "veintiséis" => 26, 
                    "veintisiete" => 27,
                    "veintiocho" => 28, 
                    "veintinueve" => 29,
                    _ => panic!("Unknown match")
                };
                IntegerValue::new(value)
            });
    b.rule_1(
            "number 100..1000",
            b.reg(r#"(cien(?:to)?s?|doscientos|trescientos|cuatrocientos|quinientos|seiscientos|setecientos|ochocientos|novecientos|mil)"#)?,
            |text_match| {
                let value = match text_match.group(1).as_ref() {
                    "cien" => 100,
                    "cientos" => 100,
                    "ciento" => 100,
                    "doscientos" => 200,
                    "trescientos" => 300,
                    "cuatrocientos" => 400,
                    "quinientos" => 500, 
                    "seiscientos" => 600, 
                    "setecientos" => 700,
                    "ochocientos" => 800,
                    "novecientos" => 900,
                    "mil" => 1000,
                    _ => panic!("Unknown match")
                };
                IntegerValue::new(value)
            });
    b.rule_3("numbers 200..999",
             integer_check!(2, 9),
             integer_check!(100, 100),
             integer_check!(0, 99),
             |a, b, c| IntegerValue::new(a.value().value * b.value().value + c.value().value));
    b.rule_1("integer (numeric)",
             b.reg(r#"(\d{1,18})"#)?,
             |text_match| IntegerValue::new(text_match.group(0).parse()?));
    b.rule_1("integer with thousands separator .",
             b.reg(r#"(\d{1,3}(\.\d\d\d){1,5})"#)?,
             |text_match| {
                 let reformatted_string = text_match.group(1).replace(".", "");
                 let value: i64 = reformatted_string.parse()?;
                 IntegerValue::new(value)
             });
    b.rule_1("decimal number", b.reg(r#"(\d*,\d+)"#)?, |text_match| {
        let reformatted_string = text_match.group(1).replace(",", ".");
        let value: f32 = reformatted_string.parse()?;
        FloatValue::new(value)
    });
    b.rule_3("number dot number",
             number_check!(|number: &NumberValue| !number.prefixed()),
             b.reg(r#"punto"#)?,
             number_check!(|number: &NumberValue| !number.suffixed()),
             |a, _, b| {
                 Ok(FloatValue {
                        value: b.value().value() * 0.1 + a.value().value(),
                        ..FloatValue::default()
                    })
             });
    b.rule_1("decimal with thousands separator",
             b.reg(r#"(\d+(\.\d\d\d)+,\d+)"#)?,
             |text_match| {
                 let reformatted_string = text_match.group(1).replace(".", "").replace(",", ".");
                 let value: f32 = reformatted_string.parse()?;
                 FloatValue::new(value)
             });
    b.rule_2("numbers prefix with -, negative or minus",
             b.reg(r#"-|menos"#)?,
             number_check!(|number: &NumberValue| !number.prefixed()),
             |_, a| -> RuleResult<NumberValue> {
        Ok(match a.value().clone() { // checked
               NumberValue::Integer(integer) => {
                   IntegerValue {
                           value: integer.value * -1,
                           prefixed: true,
                           ..integer
                       }
                       .into()
               }
               NumberValue::Float(float) => {
                   FloatValue {
                           value: float.value * -1.0,
                           prefixed: true,
                           ..float
                       }
                       .into()
               }
           })
    });
    b.rule_2("numbers suffixes (K, M, G)",
             number_check!(|number: &NumberValue| !number.suffixed()),
             b.reg_neg_lh(r#"([kmg])"#, r#"^[\W\$€]"#)?,
             |a, text_match| -> RuleResult<NumberValue> {
        let multiplier = match text_match.group(0).as_ref() {
            "k" => 1000,
            "m" => 1000000,
            "g" => 1000000000,
            _ => panic!("Unknown match"),
        };
        Ok(match a.value().clone() { // checked
               NumberValue::Integer(integer) => {
                   IntegerValue {
                           value: integer.value * multiplier,
                           suffixed: true,
                           ..integer
                       }
                       .into()
               }
               NumberValue::Float(float) => {
            let product = float.value * (multiplier as f32);
            if product.floor() == product {
                IntegerValue {
                        value: product as i64,
                        suffixed: true,
                        ..IntegerValue::default()
                    }
                    .into()
            } else {
                FloatValue {
                        value: product,
                        suffixed: true,
                        ..float
                    }
                    .into()
            }
        }
           })
    });
    b.rule_1(
            "ordinals (primero..10)",
            b.reg(r#"(primer|tercer(os?|as?)?|(primer|segund|cuart|quint|sext|s[eé]ptim|octav|noven|d[eé]cim)(os?|as?))"#)?,
            |text_match| {
                let value = match text_match.group(1).as_ref() {
                    "primer" => 1,
                    "primero" => 1,
                    "segundo" => 2,
                    "tercero" => 3,
                    "tercer" => 3,
                    "cuarto" => 4,
                    "quinto" => 5,
                    "sexto" => 6,
                    "séptimo" => 7,
                    "septimo" => 7,
                    "octavo" => 8,
                    "noveno" => 9,
                    "décimo" => 10,
                    "decimo" => 10,
                    "primeros" => 1, 
                    "segundos" => 2, 
                    "terceros" => 3, 
                    "cuartos" => 4, 
                    "quintos" => 5, 
                    "sextos" => 6,
                    "séptimos" => 7, 
                    "septimos" => 7, 
                    "octavos" => 8, 
                    "novenos" => 9, 
                    "décimos" => 10, 
                    "decimos" => 10,
                    "primera" => 1, 
                    "segunda" => 2, 
                    "tercera" => 3, 
                    "cuarta" => 4, 
                    "quinta" => 5, 
                    "sexta" => 6,
                    "séptima" => 7, 
                    "septima" => 7, 
                    "octava" => 8, 
                    "novena" => 9, 
                    "décima" => 10,
                    "decima" => 10,
                    "primeras" => 1, 
                    "segundas" => 2, 
                    "terceras" => 3, 
                    "cuartas" => 4, 
                    "quintas" => 5, 
                    "sextas" => 6,
                    "séptimas" => 7, 
                    "septimas" => 7, 
                    "octavas" => 8, 
                    "novenas" => 9, 
                    "décimas" => 10, 
                    "decimas" => 10,
                    _ => panic!("Unknown match")
                };
                Ok(OrdinalValue { value: value})
            });
    Ok(())
}

pub fn rules_finance(_b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    Ok(())
}

pub fn rules_time(_b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    Ok(())
}
