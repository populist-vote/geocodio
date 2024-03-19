use crate::AddressInput;

impl AddressInput {
    pub fn fmt_string(&self) -> String {
        let binding = "".to_string();
        let line_1 = match &self.line_1 {
            Some(val) => val,
            None => &binding,
        };
        let city = match &self.city {
            Some(val) => val,
            None => &binding,
        };
        let state = match &self.state {
            Some(val) => val,
            None => &binding,
        };
        let country = match &self.country {
            Some(val) => val,
            None => &binding,
        };
        let postal_code = match &self.postal_code {
            Some(val) => val,
            None => &binding,
        };
        format!(
            "street={}&city={}&state={}&country={}&postal_code={}",
            line_1, city, state, country, postal_code
        )
    }
    
    pub fn to_string(&self) -> String {
        let binding = "".to_string();
        let line_1 = match &self.line_1 {
            Some(val) => val,
            None => &binding,
        };
        let city = match &self.city {
            Some(val) => val,
            None => &binding,
        };
        let state = match &self.state {
            Some(val) => val,
            None => &binding,
        };
        let country = match &self.country {
            Some(val) => val,
            None => &binding,
        };
        let postal_code = match &self.postal_code {
            Some(val) => val,
            None => &binding,
        };
        format!(
            "{} {}, {}, {}, {}",
            line_1, city, state, country, postal_code
        )
    }
}
