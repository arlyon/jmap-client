use std::time::Duration;

use reqwest::{
    header::{self},
    Response,
};

use crate::{
    blob,
    core::{
        request::Request,
        session::{Session, URLPart},
    },
    event_source, Error,
};

const DEFAULT_TIMEOUT_MS: u64 = 10 * 1000;
static USER_AGENT: &str = concat!("stalwart-jmap/", env!("CARGO_PKG_VERSION"));

pub struct Client {
    session: Session,
    session_url: String,
    upload_url: Vec<URLPart<blob::URLParameter>>,
    download_url: Vec<URLPart<blob::URLParameter>>,
    event_source_url: Vec<URLPart<event_source::URLParameter>>,
    timeout: u64,
    headers: header::HeaderMap,
    default_account_id: String,
}

impl Client {
    pub async fn connect(url: &str) -> crate::Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static(USER_AGENT),
        );
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_static("Basic test"),
        );

        let session: Session = serde_json::from_slice(
            &Client::handle_error(
                reqwest::Client::builder()
                    .timeout(Duration::from_millis(DEFAULT_TIMEOUT_MS))
                    .default_headers(headers.clone())
                    .build()?
                    .get(url)
                    .send()
                    .await?,
            )
            .await?
            .bytes()
            .await?,
        )?;

        let default_account_id = session
            .primary_accounts()
            .next()
            .map(|a| a.0.to_string())
            .unwrap_or_default();

        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        Ok(Client {
            download_url: URLPart::parse(session.download_url())?,
            upload_url: URLPart::parse(session.upload_url())?,
            event_source_url: URLPart::parse(session.event_source_url())?,
            session,
            session_url: url.to_string(),
            timeout: DEFAULT_TIMEOUT_MS,
            headers,
            default_account_id,
        })
    }

    pub fn set_timeout(&mut self, timeout: u64) {
        self.timeout = timeout;
    }

    pub fn timeout(&self) -> u64 {
        self.timeout
    }

    pub fn session(&self) -> &Session {
        &self.session
    }

    pub fn headers(&self) -> &header::HeaderMap {
        &self.headers
    }

    pub async fn update_session(&mut self, new_state: &str) -> crate::Result<()> {
        if new_state != self.session.state() {
            let session: Session = serde_json::from_slice(
                &Client::handle_error(
                    reqwest::Client::builder()
                        .timeout(Duration::from_millis(DEFAULT_TIMEOUT_MS))
                        .default_headers(self.headers.clone())
                        .build()?
                        .get(&self.session_url)
                        .send()
                        .await?,
                )
                .await?
                .bytes()
                .await?,
            )?;
            self.download_url = URLPart::parse(session.download_url())?;
            self.upload_url = URLPart::parse(session.upload_url())?;
            self.event_source_url = URLPart::parse(session.event_source_url())?;
            self.session = session;
        }
        Ok(())
    }

    pub fn set_default_account_id(&mut self, defaul_account_id: impl Into<String>) {
        self.default_account_id = defaul_account_id.into();
    }

    pub fn default_account_id(&self) -> &str {
        &self.default_account_id
    }

    pub fn request(&mut self) -> Request<'_> {
        Request::new(self)
    }

    pub fn download_url(&self) -> &[URLPart<blob::URLParameter>] {
        &self.download_url
    }

    pub fn upload_url(&self) -> &[URLPart<blob::URLParameter>] {
        &self.upload_url
    }

    pub fn event_source_url(&self) -> &[URLPart<event_source::URLParameter>] {
        &self.event_source_url
    }

    pub async fn handle_error(response: Response) -> crate::Result<Response> {
        if response.status().is_success() {
            Ok(response)
        } else if let Some(b"application/problem+json") = response
            .headers()
            .get(header::CONTENT_TYPE)
            .map(|h| h.as_bytes())
        {
            Err(Error::Problem(serde_json::from_slice(
                &response.bytes().await?,
            )?))
        } else {
            Err(Error::ServerError(format!("{}", response.status())))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::response::Response;

    #[test]
    fn test_serialize() {
        let r: Response = serde_json::from_slice(
            br#"{"sessionState": "123", "methodResponses": [[ "Email/query", {
                "accountId": "A1",
                "queryState": "abcdefg",
                "canCalculateChanges": true,
                "position": 0,
                "total": 101,
                "ids": [ "msg1023", "msg223", "msg110", "msg93", "msg91",
                    "msg38", "msg36", "msg33", "msg11", "msg1" ]
            }, "t0" ],
            [ "Email/get", {
                "accountId": "A1",
                "state": "123456",
                "list": [{
                    "id": "msg1023",
                    "threadId": "trd194"
                }, {
                    "id": "msg223",
                    "threadId": "trd114"
                }
                ],
                "notFound": []
            }, "t1" ],
            [ "Thread/get", {
                "accountId": "A1",
                "state": "123456",
                "list": [{
                    "id": "trd194",
                    "emailIds": [ "msg1020", "msg1021", "msg1023" ]
                }, {
                    "id": "trd114",
                    "emailIds": [ "msg201", "msg223" ]
                }
                ],
                "notFound": []
            }, "t2" ]]}"#,
        )
        .unwrap();

        println!("{:?}", r);

        /*let mut client = Client::connect("coco");
        let mut request = client.request();

        let set = request.set_email();
        set.create().from(["pepe"]).subject("coco");
        set.update("id").keyword("keyword", true);
        set.destroy(["1", "2"]);

        let ref_ = request.result_reference("/pepe/1");

        let get = request.get_email();
        get.ids_ref(ref_);

        println!("{}", serde_json::to_string_pretty(&request).unwrap());*/

        /*let mut client = Client::connect("coco");

        client.request().email_set().create(
            "coco",
            Email::new()
                .from(["Pepe"])
                .subject("Hello world!")
                .sent_at(342374),
        );*/

        /*let query: QueryRequest<EmailFilter, EmailComparator, email::QueryArguments> =
            QueryRequest::new("coco".to_string())
                .filter(Filter::or([
                    Filter::and([
                        EmailFilter::in_mailbox("peperino"),
                        EmailFilter::in_mailbox_other_than(["coco", "miel"]),
                        EmailFilter::from("comoro"),
                    ]),
                    Filter::not([EmailFilter::after(428374234)]),
                ]))
                .sort([
                    EmailComparator::has_keyword("cocomiel"),
                    EmailComparator::size(),
                ]);

        println!("{}", serde_json::to_string_pretty(&query).unwrap());*/
    }
}