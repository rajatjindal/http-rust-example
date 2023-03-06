#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use anyhow::Result;
use spin_analytics::http_component_with_analytics;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
#[allow(clippy :: all)]
mod spin_http {
    pub type HttpStatus = u16;
    pub type Body = Vec<u8>;
    pub type Headers = Vec<(String, String)>;
    pub type Params = Vec<(String, String)>;
    pub type Uri = String;
    #[repr(u8)]
    pub enum Method { Get, Post, Put, Delete, Patch, Head, Options, }
    #[automatically_derived]
    impl ::core::clone::Clone for Method {
        #[inline]
        fn clone(&self) -> Method { *self }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Method { }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Method { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Method {
        #[inline]
        fn eq(&self, other: &Method) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Method { }
    #[automatically_derived]
    impl ::core::cmp::Eq for Method {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl core::fmt::Debug for Method {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Method::Get => { f.debug_tuple("Method::Get").finish() }
                Method::Post => { f.debug_tuple("Method::Post").finish() }
                Method::Put => { f.debug_tuple("Method::Put").finish() }
                Method::Delete => { f.debug_tuple("Method::Delete").finish() }
                Method::Patch => { f.debug_tuple("Method::Patch").finish() }
                Method::Head => { f.debug_tuple("Method::Head").finish() }
                Method::Options => {
                    f.debug_tuple("Method::Options").finish()
                }
            }
        }
    }
    pub struct Request {
        pub method: Method,
        pub uri: Uri,
        pub headers: Headers,
        pub params: Params,
        pub body: Option<Body>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Request {
        #[inline]
        fn clone(&self) -> Request {
            Request {
                method: ::core::clone::Clone::clone(&self.method),
                uri: ::core::clone::Clone::clone(&self.uri),
                headers: ::core::clone::Clone::clone(&self.headers),
                params: ::core::clone::Clone::clone(&self.params),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    impl core::fmt::Debug for Request {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Request").field("method",
                                    &self.method).field("uri",
                                &self.uri).field("headers",
                            &self.headers).field("params",
                        &self.params).field("body", &self.body).finish()
        }
    }
    pub struct Response {
        pub status: HttpStatus,
        pub headers: Option<Headers>,
        pub body: Option<Body>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Response {
        #[inline]
        fn clone(&self) -> Response {
            Response {
                status: ::core::clone::Clone::clone(&self.status),
                headers: ::core::clone::Clone::clone(&self.headers),
                body: ::core::clone::Clone::clone(&self.body),
            }
        }
    }
    impl core::fmt::Debug for Response {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Response").field("status",
                            &self.status).field("headers",
                        &self.headers).field("body", &self.body).finish()
        }
    }
    #[repr(u8)]
    pub enum HttpError {
        Success,
        DestinationNotAllowed,
        InvalidUrl,
        RequestError,
        RuntimeError,
        TooManyRequests,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for HttpError {
        #[inline]
        fn clone(&self) -> HttpError { *self }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for HttpError { }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for HttpError { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for HttpError {
        #[inline]
        fn eq(&self, other: &HttpError) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for HttpError { }
    #[automatically_derived]
    impl ::core::cmp::Eq for HttpError {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl core::fmt::Debug for HttpError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                HttpError::Success => {
                    f.debug_tuple("HttpError::Success").finish()
                }
                HttpError::DestinationNotAllowed => {
                    f.debug_tuple("HttpError::DestinationNotAllowed").finish()
                }
                HttpError::InvalidUrl => {
                    f.debug_tuple("HttpError::InvalidUrl").finish()
                }
                HttpError::RequestError => {
                    f.debug_tuple("HttpError::RequestError").finish()
                }
                HttpError::RuntimeError => {
                    f.debug_tuple("HttpError::RuntimeError").finish()
                }
                HttpError::TooManyRequests => {
                    f.debug_tuple("HttpError::TooManyRequests").finish()
                }
            }
        }
    }
    #[export_name = "handle-http-request"]
    unsafe extern "C" fn __wit_bindgen_spin_http_handle_http_request(arg0:
            i32, arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: i32,
        arg6: i32, arg7: i32, arg8: i32, arg9: i32) -> i32 {
        let len0 = arg2 as usize;
        let base3 = arg3;
        let len3 = arg4;
        let mut result3 = Vec::with_capacity(len3 as usize);
        for i in 0..len3 {
            let base = base3 + i * 16;
            result3.push({
                    let len1 = *((base + 4) as *const i32) as usize;
                    let len2 = *((base + 12) as *const i32) as usize;
                    (String::from_utf8(Vec::from_raw_parts(*((base + 0) as
                                                    *const i32) as *mut _, len1, len1)).unwrap(),
                        String::from_utf8(Vec::from_raw_parts(*((base + 8) as
                                                    *const i32) as *mut _, len2, len2)).unwrap())
                });
        }
        if len3 != 0 {
                std::alloc::dealloc(base3 as *mut _,
                    std::alloc::Layout::from_size_align_unchecked((len3 as
                                    usize) * 16, 4));
            }
        let base6 = arg5;
        let len6 = arg6;
        let mut result6 = Vec::with_capacity(len6 as usize);
        for i in 0..len6 {
            let base = base6 + i * 16;
            result6.push({
                    let len4 = *((base + 4) as *const i32) as usize;
                    let len5 = *((base + 12) as *const i32) as usize;
                    (String::from_utf8(Vec::from_raw_parts(*((base + 0) as
                                                    *const i32) as *mut _, len4, len4)).unwrap(),
                        String::from_utf8(Vec::from_raw_parts(*((base + 8) as
                                                    *const i32) as *mut _, len5, len5)).unwrap())
                });
        }
        if len6 != 0 {
                std::alloc::dealloc(base6 as *mut _,
                    std::alloc::Layout::from_size_align_unchecked((len6 as
                                    usize) * 16, 4));
            }
        let result =
            <super::SpinHttp as
                    SpinHttp>::handle_http_request(Request {
                    method: match arg0 {
                        0 => Method::Get,
                        1 => Method::Post,
                        2 => Method::Put,
                        3 => Method::Delete,
                        4 => Method::Patch,
                        5 => Method::Head,
                        6 => Method::Options,
                        _ =>
                            ::core::panicking::panic_fmt(format_args!("invalid enum discriminant")),
                    },
                    uri: String::from_utf8(Vec::from_raw_parts(arg1 as *mut _,
                                len0, len0)).unwrap(),
                    headers: result3,
                    params: result6,
                    body: match arg7 {
                        0 => None,
                        1 =>
                            Some({
                                    let len7 = arg9 as usize;
                                    Vec::from_raw_parts(arg8 as *mut _, len7, len7)
                                }),
                        _ =>
                            ::core::panicking::panic_fmt(format_args!("invalid enum discriminant")),
                    },
                });
        let ptr8 = __SPIN_HTTP_RET_AREA.0.as_mut_ptr() as i32;
        let Response { status: status9, headers: headers9, body: body9 } =
            result;
        *((ptr8 + 0) as *mut u16) =
            (wit_bindgen_rust::rt::as_i32(status9)) as u16;
        match headers9 {
            Some(e) => {
                *((ptr8 + 4) as *mut u8) = (1i32) as u8;
                let vec13 = e;
                let len13 = vec13.len() as i32;
                let layout13 =
                    core::alloc::Layout::from_size_align_unchecked(vec13.len() *
                            16, 4);
                let result13 =
                    if layout13.size() != 0 {
                            let ptr = std::alloc::alloc(layout13);
                            if ptr.is_null() {
                                    std::alloc::handle_alloc_error(layout13);
                                }
                            ptr
                        } else { std::ptr::null_mut() };
                for (i, e) in vec13.into_iter().enumerate() {
                    let base = result13 as i32 + (i as i32) * 16;
                    {
                        let (t10_0, t10_1) = e;
                        let vec11 = (t10_0.into_bytes()).into_boxed_slice();
                        let ptr11 = vec11.as_ptr() as i32;
                        let len11 = vec11.len() as i32;
                        core::mem::forget(vec11);
                        *((base + 4) as *mut i32) = len11;
                        *((base + 0) as *mut i32) = ptr11;
                        let vec12 = (t10_1.into_bytes()).into_boxed_slice();
                        let ptr12 = vec12.as_ptr() as i32;
                        let len12 = vec12.len() as i32;
                        core::mem::forget(vec12);
                        *((base + 12) as *mut i32) = len12;
                        *((base + 8) as *mut i32) = ptr12;
                    }
                }
                *((ptr8 + 12) as *mut i32) = len13;
                *((ptr8 + 8) as *mut i32) = result13 as i32;
            }
            None => {
                let e = ();
                { *((ptr8 + 4) as *mut u8) = (0i32) as u8; let () = e; }
            }
        };
        match body9 {
            Some(e) => {
                *((ptr8 + 16) as *mut u8) = (1i32) as u8;
                let vec14 = (e).into_boxed_slice();
                let ptr14 = vec14.as_ptr() as i32;
                let len14 = vec14.len() as i32;
                core::mem::forget(vec14);
                *((ptr8 + 24) as *mut i32) = len14;
                *((ptr8 + 20) as *mut i32) = ptr14;
            }
            None => {
                let e = ();
                { *((ptr8 + 16) as *mut u8) = (0i32) as u8; let () = e; }
            }
        };
        ptr8
    }
    #[repr(align(4))]
    struct __SpinHttpRetArea([u8; 28]);
    static mut __SPIN_HTTP_RET_AREA: __SpinHttpRetArea =
        __SpinHttpRetArea([0; 28]);
    pub trait SpinHttp {
        fn handle_http_request(req: Request)
        -> Response;
    }
}
struct SpinHttp;
impl spin_http::SpinHttp for SpinHttp {
    fn handle_http_request(req: spin_http::Request) -> spin_http::Response {
        fn handle_http_request_mine(req: spin_http::Request)
            -> Result<spin_http::Response> {
            use spin_analytics::recorder::enable_http_analytics;
            use anyhow::anyhow;
            let xy =
                req.try_into().expect("cannot convert from Spin HTTP request");
            let recorder = enable_http_analytics(&xy);
            fn handle_http_analytics(_: Request) -> Result<Response> {
                { ::std::io::_print(format_args!("hello there\n")); };
                Ok(Response::builder().status(200).header("foo",
                                "bar").body(Some("Hello, Fermyon".into()))?)
            }
            #[doc = " A simple Spin HTTP component."]
            fn handle_http_rust_example(_: Request) -> Result<Response> {
                Ok(http::Response::builder().status(200).header("foo",
                                "bar").body(Some("Hello, Fermyon".into()))?)
            }
            let result = handle_http_rust_example(xy);
            let a =
                match result {
                    Ok(resp) => {
                        {
                            ::std::io::_print(format_args!("from inside resp aa\n"));
                        };
                        resp.try_into().expect("cannot convert to Spin HTTP response")
                    }
                    Err(e) => {
                        { ::std::io::_print(format_args!("from inside error\n")); };
                        let body = e.to_string();
                        {
                            ::std::io::_eprint(format_args!("Handler returned an error: {0}\n",
                                    body));
                        };
                        spin_http::Response {
                            status: 500,
                            headers: None,
                            body: Some(body.as_bytes().to_vec()),
                        }
                    }
                };
            Ok(a)
        }
        match handle_http_request_mine(req.try_into().expect("cannot convert from Spin HTTP request"))
            {
            Ok(resp) =>
                resp.try_into().expect("cannot convert to Spin HTTP response"),
            Err(e) => {
                let body = e.to_string();
                {
                    ::std::io::_eprint(format_args!("Handler returned an error: {0}\n",
                            body));
                };
                spin_http::Response {
                    status: 500,
                    headers: None,
                    body: Some(body.as_bytes().to_vec()),
                }
            }
        }
    }
}
impl TryFrom<spin_http::Request> for http::Request<Option<bytes::Bytes>> {
    type Error = anyhow::Error;
    fn try_from(spin_req: spin_http::Request) -> Result<Self, Self::Error> {
        let mut http_req =
            http::Request::builder().method(spin_req.method).uri(&spin_req.uri);
        append_request_headers(&mut http_req, &spin_req)?;
        let body =
            match spin_req.body {
                Some(b) => b.to_vec(),
                None => Vec::new(),
            };
        let body = Some(bytes::Bytes::from(body));
        Ok(http_req.body(body)?)
    }
}
impl From<spin_http::Method> for http::Method {
    fn from(spin_method: spin_http::Method) -> Self {
        match spin_method {
            spin_http::Method::Get => http::Method::GET,
            spin_http::Method::Post => http::Method::POST,
            spin_http::Method::Put => http::Method::PUT,
            spin_http::Method::Delete => http::Method::DELETE,
            spin_http::Method::Patch => http::Method::PATCH,
            spin_http::Method::Head => http::Method::HEAD,
            spin_http::Method::Options => http::Method::OPTIONS,
        }
    }
}
fn append_request_headers(http_req: &mut http::request::Builder,
    spin_req: &spin_http::Request) -> anyhow::Result<()> {
    let headers = http_req.headers_mut().unwrap();
    for (k, v) in &spin_req.headers {
        headers.insert(<http::header::HeaderName as
                        std::str::FromStr>::from_str(k)?,
            http::header::HeaderValue::from_str(v)?);
    }
    Ok(())
}
impl TryFrom<spin_http::Response> for http::Response<Option<bytes::Bytes>> {
    type Error = anyhow::Error;
    fn try_from(spin_res: spin_http::Response) -> Result<Self, Self::Error> {
        let mut http_res = http::Response::builder().status(spin_res.status);
        append_response_headers(&mut http_res, spin_res.clone())?;
        let body =
            match spin_res.body {
                Some(b) => b.to_vec(),
                None => Vec::new(),
            };
        let body = Some(bytes::Bytes::from(body));
        Ok(http_res.body(body)?)
    }
}
fn append_response_headers(http_res: &mut http::response::Builder,
    spin_res: spin_http::Response) -> anyhow::Result<()> {
    let headers = http_res.headers_mut().unwrap();
    for (k, v) in spin_res.headers.unwrap() {
        headers.insert(<http::header::HeaderName as
                        std::str::FromStr>::from_str(&k)?,
            http::header::HeaderValue::from_str(&v)?);
    }
    Ok(())
}
impl TryFrom<http::Response<Option<bytes::Bytes>>> for spin_http::Response {
    type Error = anyhow::Error;
    fn try_from(http_res: http::Response<Option<bytes::Bytes>>)
        -> Result<Self, Self::Error> {
        let status = http_res.status().as_u16();
        let headers = Some(outbound_headers(http_res.headers())?);
        let body = http_res.body().as_ref().map(|b| b.to_vec());
        Ok(spin_http::Response { status, headers, body })
    }
}
fn outbound_headers(hm: &http::HeaderMap)
    -> anyhow::Result<Vec<(String, String)>> {
    let mut res = Vec::new();
    for (k, v) in hm {
        res.push((k.as_str().to_string(),
                std::str::from_utf8(v.as_bytes())?.to_string()));
    }
    Ok(res)
}
