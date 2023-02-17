/*!

normalize-url-rs is a port of Node.js [normalize-url](https://github.com/sindresorhus/normalize-url) package
for the [Rust](http://rust-lang.org/) programming language.

# Sample usage
```
use normalize_url_rs::{normalize_url, OptionsBuilder};

let options = OptionsBuilder::default().build().unwrap();
let result = normalize_url("https://www.rust-lang.org/", options);

assert_eq!(result.unwrap(), "https://rust-lang.org")
```

# Known differences vs original Node.js library

- Custom protocols are not supported
- Data URLs are not supported
*/

use derive_builder::Builder;
use fancy_regex::Regex;
use lazy_static::lazy_static;
use std::iter::Peekable;
use thiserror::Error;
use url::Url;
use urlencoding::decode;

struct SkipLastIterator<I: Iterator>(Peekable<I>);
impl<I: Iterator> Iterator for SkipLastIterator<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.0.next();
        match self.0.peek() {
            Some(_) => Some(item.unwrap()),
            None => None,
        }
    }
}
trait SkipLast: Iterator + Sized {
    fn skip_last(self) -> SkipLastIterator<Self> {
        SkipLastIterator(self.peekable())
    }
}
impl<I: Iterator> SkipLast for I {}

#[derive(Debug, Clone)]
/// Controls whether query parameters will be removed.
pub enum RemoveQueryParametersOptions {
    /// No query parameters will be removed.
    None,
    /// All query parameters will be removed.
    All,
    /// Only query parameters matching provided regular expressions will be removed.
    List(Vec<Regex>),
}

#[derive(Debug, Clone)]
/// Controls whether directory index will be removed.
pub enum RemoveDirectoryIndexOptions {
    /// No directory indices will be removed.
    None,
    /// Default regex `^index\.[a-z]+$` wil be used.
    Default,
    /// Only directory indices matching provided regular expressions will be removed.
    List(Vec<Regex>),
}

#[derive(Builder, Debug)]
#[builder(setter(into))]
/// Normalization options.
pub struct Options {
    #[builder(default = "\"http\".to_string()")]
    /// Default protocol.
    ///
    /// Default value: `http`.
    pub default_protocol: String,
    /// Prepend `defaultProtocol` to the URL if it's protocol-relative.
    ///
    /// Default value: `true`.
    #[builder(default = "true")]
    pub normalize_protocol: bool,
    /// Normalize HTTPS to HTTP.
    ///
    /// Default value: `false`.
    #[builder(default = "false")]
    pub force_http: bool,
    /// Normalize HTTP to HTTPS.
    ///
    /// This option cannot be used with the `force_http` option at the same time.
    ///
    /// Default value: `false`.
    #[builder(default = "false")]
    pub force_https: bool,
    /// Strip the authentication part of the URL.
    ///
    /// Default value: `true`.
    #[builder(default = "true")]
    pub strip_authentication: bool,
    /// Strip the hash part of the URL.
    ///
    /// Default value: `false`.
    #[builder(default = "false")]
    pub strip_hash: bool,
    /// Remove the protocol from the URL: `http://sindresorhus.com` → `sindresorhus.com`.
    ///
    /// It will only remove `https://` and `http://` protocols.
    ///
    /// Default value: `false`.
    #[builder(default = "false")]
    pub strip_protocol: bool,
    /// Strip the text fragment part of the URL.
    ///
    /// **Note**: The text fragment will always be removed if the `strip_hash` option is set to true, as the hash contains the text fragment.
    ///
    /// Default value: `true`.
    #[builder(default = "true")]
    pub strip_text_fragment: bool,
    /// Remove www. from the URL.
    ///
    /// Default value: `true`.
    #[builder(default = "true")]
    pub strip_www: bool,
    /// Remove query parameters that matches any of the provided strings or regexes.
    ///
    /// Default value: `^utm_\w+`.
    #[builder(
        default = "RemoveQueryParametersOptions::List(vec![Regex::new(r\"^utm_\\w+\").unwrap()])"
    )]
    pub remove_query_parameters: RemoveQueryParametersOptions,
    /// Keeps only query parameters that matches any of the provided strings or regexes.
    ///
    /// **Note**: It overrides the `remove_query_parameters` option.
    ///
    /// Default value: `None`.
    #[builder(default = "None")]
    pub keep_query_parameters: Option<Vec<Regex>>,
    /// Remove trailing slash.
    ///
    /// **Note**: Trailing slash is always removed if the URL doesn't have a pathname unless the `remove_single_slash` option is set to false.
    ///
    /// Default value: `true`.
    #[builder(default = "true")]
    pub remove_trailing_slash: bool,
    /// Remove a sole `/` pathname in the output. This option is independent of `remove_trailing_slash`.
    ///
    /// Default value: `true`.
    #[builder(default = "true")]
    pub remove_single_slash: bool,
    /// Removes the default directory index file from path that matches any of the provided strings or regexes. When `true`, the regex `^index\.[a-z]+$` is used.
    ///
    /// Default value: `None`.
    #[builder(default = "RemoveDirectoryIndexOptions::None")]
    pub remove_directory_index: RemoveDirectoryIndexOptions,
    /// Removes an explicit port number from the URL.
    ///
    /// Port 443 is always removed from HTTPS URLs and 80 is always removed from HTTP URLs regardless of this option.
    ///
    /// Default value: `false`.
    #[builder(default = "false")]
    pub remove_explicit_port: bool,
    /// Sorts the query parameters alphabetically by key.
    ///
    /// Default value: `true`.
    #[builder(default = "true")]
    pub sort_query_parameters: bool,
}

#[derive(Error, Debug)]
/// Errors that can occur during normalization.
pub enum NormalizeUrlError {
    #[error("The `forceHttp` and `forceHttps` options cannot be used together")]
    ForceHttpAndHttpAreExclusive,
    #[error("Unexpected error returned by `Url` library")]
    URLError,
    #[error("Unexpected error")]
    UnexpectedError(#[from] anyhow::Error),
}

pub fn normalize_url(url: &str, options: Options) -> Result<String, NormalizeUrlError> {
    if options.force_http && options.force_https {
        return Err(NormalizeUrlError::ForceHttpAndHttpAreExclusive);
    }

    let mut url_string = url.trim().to_owned();

    // Data URL
    //if (/^data:/i.test(urlString)) {
    //	return normalizeDataURL(urlString, options);
    //}
    //
    //if (hasCustomProtocol(urlString)) {
    //	return urlString;
    //}
    //

    let has_relative_protocol = url_string.starts_with("//");
    let is_relative_url = !has_relative_protocol && {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\.*\/").unwrap();
        }
        RE.is_match(&url_string)
            .map_err(Into::into)
            .map_err(NormalizeUrlError::UnexpectedError)?
    };

    // Prepend protocol
    if !is_relative_url {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?!(?:\w+:)?\/\/)|^\/\/").unwrap();
        }
        url_string = RE
            .replace(&url_string, format!("{}://", options.default_protocol))
            .to_string();
    }

    let mut url_obj = Url::parse(&url_string)
        .map_err(Into::into)
        .map_err(NormalizeUrlError::UnexpectedError)?;

    if options.force_http && url_obj.scheme() == "https" {
        url_obj
            .set_scheme("http")
            .map_err(|()| NormalizeUrlError::URLError)?;
    }

    if options.force_https && url_obj.scheme() == "http" {
        url_obj
            .set_scheme("https")
            .map_err(|()| NormalizeUrlError::URLError)?;
    }

    // Remove auth
    if options.strip_authentication {
        url_obj
            .set_username("")
            .map_err(|()| NormalizeUrlError::URLError)?;
        url_obj
            .set_password(None)
            .map_err(|()| NormalizeUrlError::URLError)?;
    }

    // Remove hash
    if options.strip_hash {
        url_obj.set_fragment(None);
    } else if options.strip_text_fragment && url_obj.fragment().is_some() {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#?:~:text.*?$").unwrap();
        }
        let new_fragment = RE.replace(url_obj.fragment().unwrap(), "").to_string();
        url_obj.set_fragment(match new_fragment.is_empty() {
            true => None,
            false => Some(&new_fragment),
        });
    }

    // Remove duplicate slashes if not preceded by a protocol
    if url_obj.path().len() > 0 {
        // Split the string by occurrences of this protocol regex, and perform
        // duplicate-slash replacement on the strings between those occurrences
        // (if any).
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\b[a-z][a-z\d+\-.]{1,50}:\/\/").unwrap();
            static ref RE2: Regex = Regex::new(r"\/{2,}").unwrap();
        }

        let mut last_index = 0;
        let mut result = "".to_string();
        for re_match in RE.captures_iter(url_obj.path()) {
            let re_match = re_match
                .map_err(Into::into)
                .map_err(NormalizeUrlError::UnexpectedError)?;

            let protocol = re_match.get(0).unwrap();
            let protocol_at_index = protocol.start();
            let intermediate = &url_obj.path()[last_index..protocol_at_index];

            result += &RE2.replace_all(intermediate, "/");
            result += protocol.as_str();
            last_index = protocol_at_index + protocol.as_str().len();
        }

        let remnant = &url_obj.path()[last_index..];
        result += &RE2.replace_all(remnant, "/");

        url_obj.set_path(&result);
    }

    // Decode URI octets
    if !url_obj.path().is_empty() {
        let decoded_path =
            decode(url_obj.path()).unwrap_or(std::borrow::Cow::Borrowed(url_obj.path()));
        url_obj.set_path(&decoded_path.to_string());
    }

    // Remove directory index
    let remove_directory_index_regexs = match &options.remove_directory_index {
        RemoveDirectoryIndexOptions::None => vec![],
        RemoveDirectoryIndexOptions::Default => vec![Regex::new(r"^index\.[a-z]+$").unwrap()],
        RemoveDirectoryIndexOptions::List(regexs) => regexs.to_vec(),
    };

    if remove_directory_index_regexs.len() > 0 && url_obj.path_segments().is_some() {
        let mut matched = false;
        let path_segments = url_obj
            .path_segments()
            .unwrap()
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();

        if let Some(last_path) = path_segments.last() {
            for regex in &remove_directory_index_regexs {
                if regex
                    .is_match(last_path)
                    .map_err(Into::into)
                    .map_err(NormalizeUrlError::UnexpectedError)?
                {
                    matched = true;
                    break;
                }
            }
        }

        let it = match matched {
            true => path_segments.iter().skip_last().collect::<Vec<_>>(),
            false => path_segments.iter().collect(),
        };

        url_obj
            .path_segments_mut()
            .map_err(Into::into)
            .map_err(|()| NormalizeUrlError::URLError)?
            .clear()
            .extend(&it);

        if matched {
            url_obj.set_path(&format!("{}/", url_obj.path()));
        }
    }

    if url_obj.host_str().is_some() {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\.$").unwrap();
        }
        // Remove trailing dot
        url_obj
            .set_host(Some(
                &RE.replace(url_obj.host_str().unwrap(), "").to_string(),
            ))
            .map_err(Into::into)
            .map_err(NormalizeUrlError::UnexpectedError)?;

        // Remove `www.`
        if options.strip_www {
            lazy_static! {
                static ref RE: Regex =
                    Regex::new(r"^www\.(?!www\.)[a-z\-\d]{1,63}\.[a-z.\-\d]{2,63}$").unwrap();
                static ref RE2: Regex = Regex::new(r"^www\.").unwrap();
            }
            // Each label should be max 63 at length (min: 1).
            // Source: https://en.wikipedia.org/wiki/Hostname#Restrictions_on_valid_host_names
            // Each TLD should be up to 63 characters long (min: 2).
            // It is technically possible to have a single character TLD, but none currently exist.
            let host_str = url_obj.host_str().unwrap().to_string();
            if RE
                .is_match(&host_str)
                .map_err(Into::into)
                .map_err(NormalizeUrlError::UnexpectedError)?
            {
                url_obj
                    .set_host(Some(&RE2.replace(&host_str, "")))
                    .map_err(Into::into)
                    .map_err(NormalizeUrlError::UnexpectedError)?;
            }
        }
    }

    // Remove query unwanted parameters
    if let RemoveQueryParametersOptions::List(ref regexs) = options.remove_query_parameters {
        let url_copy = url_obj.clone();
        let mut query_pairs = url_obj.query_pairs_mut();
        query_pairs.clear();

        for (key, value) in url_copy.query_pairs() {
            let mut matched = false;
            for regex in regexs {
                if regex
                    .is_match(&key)
                    .map_err(Into::into)
                    .map_err(NormalizeUrlError::UnexpectedError)?
                {
                    matched = true;
                    break;
                }
            }

            if !matched {
                query_pairs.append_pair(&key, &value);
            }
        }

        query_pairs.finish();
    }

    if options.keep_query_parameters.is_none() {
        if let RemoveQueryParametersOptions::All = &options.remove_query_parameters {
            url_obj.set_query(None);
        }
    }

    // Keep wanted query parameters
    if options.keep_query_parameters.is_some() {
        let url_copy = url_obj.clone();
        let mut query_pairs = url_obj.query_pairs_mut();
        query_pairs.clear();
        for (key, value) in url_copy.query_pairs() {
            for regex in options.keep_query_parameters.as_ref().unwrap() {
                if regex
                    .is_match(&key)
                    .map_err(Into::into)
                    .map_err(NormalizeUrlError::UnexpectedError)?
                {
                    query_pairs.append_pair(&key, &value);
                    break;
                }
            }
        }
        query_pairs.finish();
    }

    if let Some(query_str) = url_obj.query() {
        if query_str.is_empty() {
            url_obj.set_query(None);
        }
    }

    // Sort query parameters
    if options.sort_query_parameters && url_obj.query_pairs().count() > 0 {
        {
            let url_copy = url_obj.clone();
            let mut query_pairs = url_obj.query_pairs_mut();
            query_pairs.clear();
            let mut pairs = url_copy.query_pairs().collect::<Vec<_>>();
            pairs.sort_by(|a, b| a.0.cmp(&b.0));
            query_pairs.extend_pairs(pairs).finish();
        }

        if let Some(query) = url_obj.query() {
            let decoded_query = decode(query).unwrap_or(std::borrow::Cow::Borrowed(query));
            url_obj.set_query(Some(&decoded_query.to_string()));
        }
    }

    if options.remove_trailing_slash {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\/$").unwrap();
        }
        url_obj.set_path(&RE.replace(url_obj.path(), "").to_string());
    }

    // Remove an explicit port number, excluding a default port number, if applicable
    if options.remove_explicit_port && url_obj.port().is_some() {
        url_obj
            .set_port(None)
            .map_err(|()| NormalizeUrlError::URLError)?;
    }

    let old_url_string = url_string;

    url_string = url_obj.to_string();

    let is_option_empty = |x: Option<&str>| -> bool {
        match x {
            Some("") => true,
            None => true,
            _ => false,
        }
    };

    if !options.remove_single_slash
        && url_obj.path() == "/"
        && !old_url_string.ends_with('/')
        && is_option_empty(url_obj.fragment())
    {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\/$").unwrap();
        }
        url_string = RE.replace(&url_string, "").to_string();
    }

    // Remove ending `/` unless removeSingleSlash is false
    if (options.remove_trailing_slash || url_obj.path() == "/")
        && is_option_empty(url_obj.fragment())
        && options.remove_single_slash
    {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\/$").unwrap();
        }
        url_string = RE.replace(&url_string, "").to_string();
    }

    // Restore relative protocol, if applicable
    if has_relative_protocol && !options.normalize_protocol {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^http:\/\/").unwrap();
        }
        url_string = RE.replace(&url_string, "//").to_string();
    }

    // Remove http/https
    if options.strip_protocol {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?:https?:)?\/\/").unwrap();
        }
        url_string = RE.replace(&url_string, "").to_string();
    }

    Ok(url_string)
}
