//! 爬虫模块

use hyper::Client;
use downloader::{Request, Response, Method, RequestContent};
use url::Url;
//爬虫特质
pub trait Spider: Send + Sync{
    //传输数据
    type ItemType;
    /// 返回爬虫工具名称.
    fn name(&self) -> &str;
    /// 返回允许的域名.
    fn allowed_domains(&self) -> &[String];
    ///返回一组url，供爬虫工具使用
    fn start_urls(&self) -> &[String];
    /// 返回一组供请求的对象. By default
    /// It will call start_urls() to get start urls and issue http get requests
    /// to those urls.
    fn start_requests(&self) -> Vec<Request>{
        let start_urls = self.start_urls();
        let mut requests: Vec<Request> = vec![];
        for start_url in start_urls {
            match Url::parse(&start_url){
                Ok(url) => {
                    requests.push(Request{
                        content: RequestContent{
                            url: url,
                            method: Method::Get,
                            body: None,
                        },
                        client: Client::new(),
                    });
                },
                Err(e) => {
                    self.log(&format!("{}", e));
                }
            }
        }
        requests
    }
    /// 日志函数.默认输出到标准stdout.
    fn log(&self, _str: &str){
        println!("{}", _str);
    }
    /// 解析 `Response` 并得到 `Request`集合 和 item 集合.
    fn parse(&self, response: Response) -> (Vec<Request>, Vec<ItemType>);
}
