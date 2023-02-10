
pub enum  Method {
    Get,
    Post,
    Put,
    Update,
    Delete
}

pub struct GetResult {
    pub args: Option<Args>,
    pub data: Option<String>,
    pub files: Option<Args>,
    pub form: Option<Args>,
    // pub headers: Option<Headers>,
    pub json: Option<serde_json::Value>,
    pub origin: Option<String>,
    pub url: Option<String>
}

pub struct Args {

}

// pub struct Headers {
//     #[serde(rename = "Accept")]
//     pub accept: Option<String>,
//     #[serde(rename = "Accept-Encoding")]
//     pub accept_encoding: Option<String>,
//     #[serde(rename = "Accept-Language")]
//     pub accept_language: Option<String>,
//     #[serde(rename = "Dnt")]
//     pub dnt: Option<String>,
//     #[serde(rename = "Host")]
//     pub host: Option<String>,
//     #[serde(rename = "Origin")]
//     pub origin: Option<String>,
//     #[serde(rename = "Referer")]
//     pub referer: Option<String>,
//
//     #[serde(rename = "Sec-Ch-Ua")]
//     pub sec_ch_ua: Option<String>,
//     #[serde(rename = "Sec-Ch-Ua-Mobile")]
//     pub sec_ch_ua_mobile: Option<String>,
//     #[serde(rename = "Sec-Ch-Ua-Platform")]
//     pub sec_ch_ua_platform: Option<String>,
//
//     #[serde(rename = "Sec-Fetch-Dest")]
//     pub sec_fetch_dest: Option<String>,
//     #[serde(rename = "Sec-Fetch-Mode")]
//     pub sec_fetch_mode: Option<String>,
//     #[serde(rename = "Sec-Fetch-Site")]
//     pub sec_fetch_site: Option<String>,
//
//     #[serde(rename = "User-Agent")]
//     pub user_agent: Option<String>,
//     #[serde(rename = "X-Amzn-Trace-Id")]
//     pub x_amzn_trace_id: Option<String>
// }