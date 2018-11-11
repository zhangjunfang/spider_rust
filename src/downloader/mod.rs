//! 下载 模块

use hyper::Client;
use hyper::client::RequestBuilder;
use hyper::header::Headers;
use hyper::status::StatusCode;
use hyper::client::response::Response as HpResp;
use hyper::error::Error as HpErr;
use url::Url;
use std::io::{Read, Error as ReadErr};
use std::io::ErrorKind;

/// Download error occured when issueing a `Request`.
pub enum DownloadError{
    /// The status code is not Ok.
    BadStatus(HpResp),
    /// Special read error timeout.
    TimedOut(HpResp),
    /// Other read error except for timeout.
    ReadError(HpResp, ReadErr),
    /// Errors that can occur in parsing HTTP streams.
    BadRequest(HpErr),
}

/// Http methods
#[derive(Clone)]
pub enum Method {
    /// Http get
    Get,
    /// Http post
    Post,
}

/// A simple ok response including url, headers and body.
pub struct Response {
    /// The target url from the original `Request`
    pub url: Url,
    /// The reponse header
    pub headers: Headers,
    /// The response body. Can be either text or binary.
    pub body: Vec<u8>,//TODO: use a better linear container.
}

/// The content of a `Request` including url, headers and body.
#[derive(Clone)]
pub struct RequestContent{
    /// The target url
    pub url: Url,
    /// The method used to issue the `Request`
    pub method: Method,
    /// The body, text only for now
    pub body: Option<String>,
}

/// A simple request
pub struct Request
{
    /// The request content
    pub content: RequestContent,
    /// The hyper client used for issuing the request
    pub client: Client,
}

impl Request {
    /// Issuse the `Request` to the server
    pub fn download(self)-> Result<Response, DownloadError> {
        let url = self.content.url.clone();
        let mut client: RequestBuilder;
        match self.content.method {
            Method::Get => {
                client = self.client.get(url);
            },
            Method::Post => {
                client = self.client.post(url);
            },
        }
        if let Some(ref body) = self.content.body{
            client = client.body(body);
        }
        let response = client.send();
        match response{
            Ok(mut response) => {
                if let StatusCode::Ok = response.status{
                    let mut buffer = vec![];
                    match response.read_to_end(&mut buffer){
                        Ok(_) => {
                            Ok(Response{
                                url: response.url.clone(),
                                headers: response.headers.clone(),
                                body: buffer,
                            })
                        }
                        Err(e) => {
                            match e.kind(){
                                ErrorKind::TimedOut => {
                                    Err(DownloadError::TimedOut(response))
                                }
                                _ => {
                                    Err(DownloadError::ReadError(response, e))
                                }
                            }
                        }
                    }
                }
                else{
                    Err(DownloadError::BadStatus(response))
                }
            },
            Err(e) => Err(DownloadError::BadRequest(e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_download() {
        let url = Url::parse("http://www.baidu.com").unwrap();
        let client = Client::new();
        let request:Request = Request{
            url: url,
            method: Method::Get,
            body: None,
            client: client,
        };
        request.download().unwrap();
    }
}
