use handlebars::{Handlebars, ScopedJson, RenderError};
use chrono::NaiveDate;
use chrono::Datelike;

pub struct ParseDate;

impl handlebars::HelperDef for ParseDate {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        helper: &handlebars::Helper<'reg, 'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc handlebars::Context,
        _: &mut handlebars::RenderContext<'reg, 'rc>
        ) -> Result<Option<ScopedJson<'reg, 'rc>>, RenderError>
    {
        let param = helper.param(0)
            .ok_or_else(|| RenderError::new("no parameter given!"))
            .and_then(|v| v.value().as_str().ok_or_else(|| RenderError::new("parameter is not a string!")))?;
        let parsed = param.parse::<NaiveDate>().map_err(|e| RenderError::from_error("date parse error", e))?;
        let result = json!({"year": parsed.year(), "month": parsed.month(), "day": parsed.day(), "month_name": month_name(parsed.month())});
        Ok(Some(ScopedJson::Derived(result)))
    }
}

fn month_name(y: u32) -> &'static str {
    match y {
        1 => "January", 2 => "February", 3 => "March",
        4 => "April", 5 => "May", 6 => "June",
        7 => "July", 8 => "August", 9 => "September",
        10 => "October", 11 => "November", 12 => "December",
        _ => panic!("Invalid month number: {}", y)
    }
}

use std::convert::TryInto;
handlebars_helper!(take: |n: u64, arr: array| if n as usize > arr.len() { arr } else { arr.split_at(n.try_into().unwrap()).0 });
