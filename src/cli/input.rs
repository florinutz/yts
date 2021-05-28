use strum::{EnumString, EnumVariantNames, Display, EnumIter};

#[derive(Display, Debug, PartialEq, EnumString, EnumVariantNames, EnumIter)]
pub enum Qualities {
    #[strum(serialize = "720p")]
    Q720p,
    #[strum(serialize = "1080p")]
    Q1080p,
    #[strum(serialize = "2160p")]
    Q2160p,
    #[strum(serialize = "3D")]
    Q3D,
}

pub fn validate_natural_one_plus(msg: String) -> impl Fn(String) -> Result<(), String> {
    move |val| match val.trim().parse::<u16>() {
        Ok(x) if x > 0 => Ok(()),
        _ => Err(msg.clone()),
    }
}

pub fn validate_min_rating(msg: String) -> impl Fn(String) -> Result<(), String> {
    move |val| match val.trim().parse::<u8>() {
        Ok(x) if (1..=9).contains(&x) => Ok(()),
        _ => Err(msg.clone()),
    }
}
