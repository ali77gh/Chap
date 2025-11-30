use std::str::FromStr;

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT},
};

use crate::{
    builtin_function::utils::{param_to_datatype, returns},
    common::{
        data_type::DataType,
        errors::{ChapError, Result},
        executable::ExecutableLine,
    },
    runtime::Runtime,
};

pub fn http_get(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let p1 = param_to_datatype(runtime, executable.params.first(), executable.line_number)?;
    let p2 = if let Some(_) = executable.params.get(1) {
        Some(param_to_datatype(
            runtime,
            executable.params.get(1),
            executable.line_number,
        )?)
    } else {
        None
    };

    match p1 {
        DataType::String(url) => {
            let mut headers = HeaderMap::new();

            headers.insert(USER_AGENT, HeaderValue::from_static("Chap"));

            if let Some(p2) = p2 {
                if let DataType::Map(p2) = p2 {
                    for (key, value) in p2 {
                        let header = HeaderName::from_str(key).map_err(|_| {
                            ChapError::runtime_with_msg(
                                executable.line_number,
                                format!("invalid header name: {}", key),
                            )
                        })?;

                        let value = if let DataType::String(value) = value {
                            value
                        } else {
                            return Err(ChapError::runtime_with_msg(
                                executable.line_number,
                                format!("header value must me a string, got {}", value.type_name()),
                            ));
                        };

                        let value = HeaderValue::from_str(value).map_err(|_| {
                            ChapError::runtime_with_msg(
                                executable.line_number,
                                format!("failed to initialize TLS backend"),
                            )
                        })?;

                        headers.insert(header, value);
                    }
                } else {
                    return Err(ChapError::runtime_with_msg(
                        executable.line_number,
                        format!(
                            "second parameter for headers must be a map, got {}",
                            p2.type_name()
                        ),
                    ));
                }
            }

            let client = Client::builder()
                .default_headers(headers)
                .build()
                .map_err(|_| {
                    ChapError::runtime_with_msg(
                        executable.line_number,
                        format!("failed to initialize TLS backend"),
                    )
                })?;

            let resp = client.get(url).send().map_err(|e| {
                ChapError::runtime_with_msg(
                    executable.line_number,
                    format!("failed to send GET HTTP request: {}", e),
                )
            })?;

            let body = resp.text().map_err(|e| {
                ChapError::runtime_with_msg(
                    executable.line_number,
                    format!("failed to read response body: {}", e),
                )
            })?;

            returns(runtime, executable, DataType::String(body))
        }
        _ => {
            return Err(ChapError::runtime_with_msg(
                executable.line_number,
                format!("first parameter must be a string, got {}", p1.type_name()),
            ));
        }
    }
}
