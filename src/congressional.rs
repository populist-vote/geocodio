use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateLegislativeDistricts {
    house: Vec<StateLegislativeDistrict>,
    senate: Vec<StateLegislativeDistrict>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateLegislativeDistrict {
    name: String,
    district_number: String,
    is_upcoming_state_legislative_district: bool,
    proportion: i16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CongressionalDistrict {
    pub name: String,
    pub district_number: i16,
    pub congress_number: String,
    pub congress_years: String,
    pub proportion: i16,
    pub current_legislators: Option<Vec<Legislator>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Legislator {
    #[serde(rename = "type")]
    pub type_field: String,
    pub bio: Bio,
    pub contact: Contact,
    pub social: CongressionalSocial,
    pub references: References,
    pub source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bio {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<String>,
    pub gender: Option<String>,
    pub party: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    pub url: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub contact_form: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CongressionalSocial {
    pub rss_url: Option<String>,
    pub twitter: Option<String>,
    pub facebook: Option<String>,
    pub youtube: Option<String>,
    pub youtube_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct References {
    pub bioguide_id: Option<String>,
    pub thomas_id: Option<String>,
    pub govtrack_id: Option<String>,
    pub opensecrets_id: Option<String>,
    pub votesmart_id: Option<String>,
    pub lis_id: Option<String>,
    pub cspan_id: Option<String>,
    pub icpsr_id: Option<String>,
    pub wikipedia_id: Option<String>,
    pub washington_post_id: Option<String>,
}
