use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Components {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    query: String,
    components: Components,
    formatted: String,
    location: Coordinates,
    accuracy: f64,
    accuracy_type: String,
    source: String,
    fields: Fields,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fields {
    timezone: Timezone,
    zip4: Zip4,
    congressional_district: CongressionalDistrict,
    congressional_districts: Vec<CongressionalDistrict>,
    state_legislative_districts: StateLegislativeDistricts,
    school_districts: Vec<SchoolDistrict>,
    // census: CensusResults, // TO DO
    // acs: CencusAcs, // TO DO
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    #[serde(rename = "lat")]
    latitude: f64,
    #[serde(rename = "lng")]
    longitude: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateLegislativeDistricts {
    house: StateLegislativeDistrict,
    senate: StateLegislativeDistrict,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateLegislativeDistrict {
    name: String,
    district_number: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchoolDistricts {
    unified: SchoolDistrict,
    elementary: SchoolDistrict,
    secondary: SchoolDistrict,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchoolDistrict {
    name: String,
    lea_code: String,
    grade_low: String,
    grade_high: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timezone {
    name: String,
    abbreviation: String,
    utc_offset: i16,
    observes_dst: bool,
    source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Zip4 {
    record_type: RecordType,
    carrier_route: CarrierRoute,
    building_or_firm_name: String,
    plus4: Vec<String>,
    zip9: Vec<String>,
    government_building: String,
    facility_code: FacilityCode,
    city_delivery: bool,
    valid_delivery_area: bool,
    exact_match: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordType {
    code: String,
    description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CarrierRoute {
    id: String,
    description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FacilityCode {
    code: String,
    description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CongressionalDistrict {
    pub name: String,
    pub district_number: i16,
    pub congress_number: String,
    pub congress_years: String,
    pub proportion: i16,
    pub current_legislators: Vec<Legislator>,
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
    pub first_name: String,
    pub last_name: String,
    pub birthday: String,
    pub gender: String,
    pub party: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    pub url: String,
    pub address: String,
    pub phone: String,
    pub contact_form: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CongressionalSocial {
    pub rss_url: String,
    pub twitter: String,
    pub facebook: String,
    pub youtube: String,
    pub youtube_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct References {
    pub bioguide_id: String,
    pub thomas_id: String,
    pub govtrack_id: String,
    pub opensecrets_id: String,
    pub votesmart_id: String,
    pub lis_id: String,
    pub cspan_id: String,
    pub icpsr_id: String,
    pub wikipedia_id: String,
    pub washington_post_id: String,
}
