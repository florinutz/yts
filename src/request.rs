use crate::parse::api::ListResponse;
use cached_path::{Cache, Options as CacheOptions};
use reqwest::blocking::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue};
use std::error::Error;
use std::string::ToString;
use std::time::Duration;
use strum::{Display, EnumIter, EnumString, EnumVariantNames, IntoStaticStr};
use url::Url;

#[derive(Debug, Default)]
pub struct ListRequest<'a> {
    limit: Option<u8>,
    page: Option<u32>,
    rating: Option<u8>,
    quality: Option<Quality>,
    minimum_rating: Option<u8>,
    query_term: Option<String>,
    genre: Option<&'a str>,
    sort_by: Option<Sort>,
    order_by: Option<Order>,
    wirth_rt_ratings: Option<bool>,
    mirror: Option<&'a str>,
}

#[derive(Debug, PartialEq, Display, EnumString, EnumVariantNames, EnumIter, IntoStaticStr)]
pub enum Quality {
    #[strum(serialize = "720p")]
    Quality720p,
    #[strum(serialize = "1080p")]
    Quality1080p,
    #[strum(serialize = "2160p")]
    Quality2160p,
    #[strum(serialize = "3D")]
    Quality3D,
}

#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Display,
    EnumString,
    EnumVariantNames,
    EnumIter,
    IntoStaticStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum Sort {
    Title,
    Year,
    Rating,
    Peers,
    Seeds,
    DownloadCount,
    LikeCount,
    DateAdded,
}

#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Display,
    EnumString,
    EnumVariantNames,
    EnumIter,
    IntoStaticStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum Order {
    Asc,
    Desc,
}

impl<'a> ListRequest<'a> {
    pub fn new() -> ListRequest<'a> {
        ListRequest::default()
    }

    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.limit = Some(limit);
        self
    }
    pub fn page(&mut self, page: u32) -> &mut Self {
        self.page = Some(page);
        self
    }
    pub fn rating(&mut self, rating: u8) -> &mut Self {
        self.rating = Some(rating);
        self
    }
    pub fn quality(&mut self, quality: Quality) -> &mut Self {
        self.quality = Some(quality);
        self
    }
    pub fn query_term(&mut self, query_term: String) -> &mut Self {
        self.query_term = Some(query_term);
        self
    }
    pub fn genre(&mut self, genre: &'a str) -> &mut Self {
        self.genre = Some(genre);
        self
    }
    pub fn sort_by(&mut self, sort_by: Sort) -> &mut Self {
        self.sort_by = Some(sort_by);
        self
    }
    pub fn order_by(&mut self, order_by: Order) -> &mut Self {
        self.order_by = Some(order_by);
        self
    }
    pub fn wirth_rt_ratings(&mut self, wirth_rt_ratings: bool) -> &mut Self {
        self.wirth_rt_ratings = wirth_rt_ratings.then_some(true);
        self
    }
    /// This will change the domain name or fail
    pub fn mirror(&mut self, mirror: &'a str) -> Result<&mut Self, Box<dyn Error>> {
        let mut u = Url::parse("https://smth.com")?;
        u.set_host(Some(&mirror))?;
        self.mirror = Some(&mirror);
        Ok(self)
    }

    pub fn url(&self) -> Url {
        let mut url = Url::parse("https://yts.mx/api/v2/list_movies.json").unwrap();

        if let Some(limit) = self.limit {
            url.query_pairs_mut()
                .append_pair("limit", &limit.to_string());
        }
        if let Some(val) = self.page {
            url.query_pairs_mut().append_pair("page", &val.to_string());
        }
        if let Some(val) = &self.quality {
            url.query_pairs_mut().append_pair("quality", val.into());
        }
        if let Some(val) = self.minimum_rating {
            url.query_pairs_mut()
                .append_pair("minimum_rating", &val.to_string());
        }
        if let Some(val) = &self.query_term {
            url.query_pairs_mut().append_pair("query_term", val);
        }
        if let Some(val) = self.genre {
            url.query_pairs_mut().append_pair("genre", val);
        }
        if let Some(val) = &self.sort_by {
            url.query_pairs_mut().append_pair("sort_by", val.into());
        }
        if let Some(val) = &self.order_by {
            url.query_pairs_mut().append_pair("order_by", val.into());
        }
        if self.wirth_rt_ratings.is_some() {
            url.query_pairs_mut().append_key_only("with_rt_ratings");
        }
        if let Some(val) = self.mirror {
            // val is validated in the mirror setter so it's safe at this point
            url.set_host(Some(val)).unwrap();
        }

        url
    }

    pub fn execute(&self) -> Result<ListResponse, Box<dyn Error>> {
        get_list(&self.url())
    }
}

impl<'a> From<ListRequest<'a>> for Url {
    fn from(r: ListRequest<'a>) -> Self {
        r.url()
    }
}

/// The list getter wraps a cache layer around the actual api call.
/// The cache is stored in the os's temp folder.
/// Whenever the content has modified it will be fetched again.
fn get_list(url: &Url) -> Result<ListResponse, Box<dyn std::error::Error>> {
    let mut header_map = HeaderMap::new();
    header_map.insert("user-agent", HeaderValue::from_static("florinutz/yts"));

    let client_builder = ClientBuilder::new()
        .timeout(Duration::from_secs(5))
        .default_headers(header_map);

    let cache = Cache::builder()
        .progress_bar(None)
        .dir(std::env::temp_dir().join("yts/"))
        .client_builder(client_builder)
        .build()?;

    let path = cache.cached_path_with_options(
        url.to_string().as_str(),
        &CacheOptions::default().subdir("list"),
    )?;

    let json = std::fs::read_to_string(path)?;
    let res: ListResponse = serde_json::from_str(json.as_str())?;

    Ok(res)
}
