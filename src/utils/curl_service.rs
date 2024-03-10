use std::io::Read;

use curl::easy::Easy;

use crate::http_method::HTTPMethod;

pub fn curl_call(url: &str, data: &mut Vec<u8>, headers: curl::easy::List, post_data: &str, method: HTTPMethod) {
    match method {
        HTTPMethod::GET => curl_get_call(url, data, headers),
        HTTPMethod::POST => curl_post_call(url, data, headers, post_data)
    }
}

pub fn curl_get_call(url: &str, data: &mut Vec<u8>, headers: curl::easy::List) {
        let mut easy = Easy::new();
        easy.url(url).unwrap();
        easy.http_headers(headers).unwrap();
        let mut transfer = easy.transfer();

        transfer.write_function(|d| {
            data.extend_from_slice(d);
            Ok(d.len())
        }).unwrap();

        transfer.perform().unwrap();
}

pub fn curl_post_call(url: &str, data: &mut Vec<u8>, headers: curl::easy::List, post_data: &str) {
        let mut post_data_as_bytes = post_data.as_bytes();
        let mut easy = Easy::new();
        easy.post(true).unwrap();
        easy.url(url).unwrap();
        easy.http_headers(headers).unwrap();
        easy.post_field_size(post_data_as_bytes.len() as u64).unwrap();
        let mut transfer = easy.transfer();

        transfer.read_function(|buf| {
            Ok(post_data_as_bytes.read(buf).unwrap_or(0))
        }).unwrap();


        transfer.write_function(|d| {
            data.extend_from_slice(d);
            Ok(d.len())
        }).unwrap();

        transfer.perform().unwrap_or(());
}
