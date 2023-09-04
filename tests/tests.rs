#[cfg(test)]
mod tests {
    use fancy_regex::Regex;
    use normalize_url_rs::*;
    use rstest::rstest;

    #[rstest]
    #[case("sindresorhus.com", "http://sindresorhus.com")]
    #[case("sindresorhus.com ", "http://sindresorhus.com")]
    #[case("sindresorhus.com.", "http://sindresorhus.com")]
    #[case("SindreSorhus.com", "http://sindresorhus.com")]
    #[case("HTTP://sindresorhus.com", "http://sindresorhus.com")]
    #[case("//sindresorhus.com", "http://sindresorhus.com")]
    #[case("http://sindresorhus.com", "http://sindresorhus.com")]
    #[case("http://sindresorhus.com:80", "http://sindresorhus.com")]
    #[case("https://sindresorhus.com:443", "https://sindresorhus.com")]
    #[case("http://www.sindresorhus.com", "http://sindresorhus.com")]
    #[case("www.com", "http://www.com")]
    #[case("http://www.www.sindresorhus.com", "http://www.www.sindresorhus.com")]
    #[case("www.sindresorhus.com", "http://sindresorhus.com")]
    #[case("http://sindresorhus.com/foo/", "http://sindresorhus.com/foo")]
    #[case(
        "sindresorhus.com/?foo=bar baz",
        "http://sindresorhus.com/?foo=bar+baz"
    )]
    #[case("https://foo.com/https://bar.com", "https://foo.com/https://bar.com")]
    #[case(
        "https://foo.com/https://bar.com/foo//bar",
        "https://foo.com/https://bar.com/foo/bar"
    )]
    #[case("https://foo.com/http://bar.com", "https://foo.com/http://bar.com")]
    #[case(
        "https://foo.com/http://bar.com/foo//bar",
        "https://foo.com/http://bar.com/foo/bar"
    )]
    #[case("http://sindresorhus.com/%7Efoo/", "http://sindresorhus.com/~foo")]
    #[case(
        "https://foo.com/%FAIL%/07/94/ca/55.jpg",
        "https://foo.com/%FAIL%/07/94/ca/55.jpg"
    )]
    #[case("http://sindresorhus.com/?", "http://sindresorhus.com")]
    #[case("êxample.com", "http://xn--xample-hva.com")]
    #[case(
        "http://sindresorhus.com/?b=bar&a=foo",
        "http://sindresorhus.com/?a=foo&b=bar"
    )]
    #[case(
        r#"http://sindresorhus.com/?foo=bar*|<>:""#,
        "http://sindresorhus.com/?foo=bar*|%3C%3E:%22"
    )]
    #[case("http://sindresorhus.com:5000", "http://sindresorhus.com:5000")]
    #[case("http://sindresorhus.com/foo#bar", "http://sindresorhus.com/foo#bar")]
    #[case(
        "http://sindresorhus.com/foo/bar/../baz",
        "http://sindresorhus.com/foo/baz"
    )]
    #[case(
        "http://sindresorhus.com/foo/bar/./baz",
        "http://sindresorhus.com/foo/bar/baz"
    )]
    #[case("https://i.vimeocdn.com/filter/overlay?src0=https://i.vimeocdn.com/video/598160082_1280x720.jpg&src1=https://f.vimeocdn.com/images_v6/share/play_icon_overlay.png", "https://i.vimeocdn.com/filter/overlay?src0=https://i.vimeocdn.com/video/598160082_1280x720.jpg&src1=https://f.vimeocdn.com/images_v6/share/play_icon_overlay.png")]
    fn main_tests(#[case] input: String, #[case] expected: String) {
        let result = normalize_url(&input, &OptionsBuilder::default().build().unwrap())
            .expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("sindresorhus.com", "http", "http://sindresorhus.com")]
    #[case("sindresorhus.com", "https", "https://sindresorhus.com")]
    fn default_protocol_tests(
        #[case] input: String,
        #[case] protocol: String,
        #[case] expected: String,
    ) {
        let options = OptionsBuilder::default()
            .default_protocol(protocol)
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(
        "http://user:password@www.sindresorhus.com",
        true,
        "http://sindresorhus.com"
    )]
    #[case(
        "https://user:password@www.sindresorhus.com",
        true,
        "https://sindresorhus.com"
    )]
    #[case(
        "https://user:password@www.sindresorhus.com/@user",
        true,
        "https://sindresorhus.com/@user"
    )]
    #[case(
        "http://user:password@www.êxample.com",
        true,
        "http://xn--xample-hva.com"
    )]
    #[case(
        "http://user:password@www.sindresorhus.com",
        false,
        "http://user:password@sindresorhus.com"
    )]
    #[case(
        "https://user:password@www.sindresorhus.com",
        false,
        "https://user:password@sindresorhus.com"
    )]
    #[case(
        "https://user:password@www.sindresorhus.com/@user",
        false,
        "https://user:password@sindresorhus.com/@user"
    )]
    #[case(
        "http://user:password@www.êxample.com",
        false,
        "http://user:password@xn--xample-hva.com"
    )]
    fn strip_auth_tests(#[case] input: String, #[case] strip_auth: bool, #[case] expected: String) {
        let options = OptionsBuilder::default()
            .strip_authentication(strip_auth)
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("http://www.sindresorhus.com", "sindresorhus.com")]
    #[case("http://sindresorhus.com", "sindresorhus.com")]
    #[case("https://www.sindresorhus.com", "sindresorhus.com")]
    #[case("//www.sindresorhus.com", "sindresorhus.com")]
    fn strip_protocols_tests(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default()
            .strip_protocol(true)
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("http://sindresorhus.com", false, true, "http://sindresorhus.com")]
    #[case(
        "http://sindresorhus.com/about#",
        false,
        true,
        "http://sindresorhus.com/about"
    )]
    #[case(
        "http://sindresorhus.com/about#:~:text=hello",
        false,
        true,
        "http://sindresorhus.com/about"
    )]
    #[case(
        "http://sindresorhus.com/about#main",
        false,
        true,
        "http://sindresorhus.com/about#main"
    )]
    #[case(
        "http://sindresorhus.com/about#main:~:text=hello",
        false,
        true,
        "http://sindresorhus.com/about#main"
    )]
    #[case(
        "http://sindresorhus.com/about#main:~:text=hello%20world",
        false,
        true,
        "http://sindresorhus.com/about#main"
    )]
    #[case("http://sindresorhus.com", false, false, "http://sindresorhus.com")]
    #[case(
        "http://sindresorhus.com/about#:~:text=hello",
        false,
        false,
        "http://sindresorhus.com/about#:~:text=hello"
    )]
    #[case(
        "http://sindresorhus.com/about#main",
        false,
        false,
        "http://sindresorhus.com/about#main"
    )]
    #[case(
        "http://sindresorhus.com/about#main:~:text=hello",
        false,
        false,
        "http://sindresorhus.com/about#main:~:text=hello"
    )]
    #[case(
        "http://sindresorhus.com/about#main:~:text=hello%20world",
        false,
        false,
        "http://sindresorhus.com/about#main:~:text=hello%20world"
    )]
    #[case("http://sindresorhus.com", true, false, "http://sindresorhus.com")]
    #[case(
        "http://sindresorhus.com/about#:~:text=hello",
        true,
        false,
        "http://sindresorhus.com/about"
    )]
    #[case(
        "http://sindresorhus.com/about#main",
        true,
        false,
        "http://sindresorhus.com/about"
    )]
    #[case(
        "http://sindresorhus.com/about#main:~:text=hello",
        true,
        false,
        "http://sindresorhus.com/about"
    )]
    #[case(
        "http://sindresorhus.com/about#main:~:text=hello%20world",
        true,
        false,
        "http://sindresorhus.com/about"
    )]
    fn strip_text_fragment_tests(
        #[case] input: String,
        #[case] strip_hash: bool,
        #[case] strip_text_fragment: bool,
        #[case] expected: String,
    ) {
        let options = OptionsBuilder::default()
            .strip_hash(strip_hash)
            .strip_text_fragment(strip_text_fragment)
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("http://www.sindresorhus.com", false, "http://www.sindresorhus.com")]
    #[case("www.sindresorhus.com", false, "http://www.sindresorhus.com")]
    #[case("http://www.êxample.com", false, "http://www.xn--xample-hva.com")]
    #[case("http://www.vue.amsterdam", true, "http://vue.amsterdam")]
    #[case(
        "http://www.sorhus.xx--bck1b9a5dre4c",
        true,
        "http://sorhus.xx--bck1b9a5dre4c"
    )]
    fn strip_www_tests(#[case] input: String, #[case] strip: bool, #[case] expected: String) {
        let options = OptionsBuilder::default().strip_www(strip).build().unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(
        "www.sindresorhus.com?foo=bar&utm_medium=test",
        "http://sindresorhus.com/?foo=bar"
    )]
    fn remove_query_parameters_tests_1(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default().build().unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("http://www.sindresorhus.com", "http://www.sindresorhus.com")]
    #[case("www.sindresorhus.com?foo=bar", "http://www.sindresorhus.com/?foo=bar")]
    #[case(
        "www.sindresorhus.com?foo=bar&utm_medium=test&ref=test_ref",
        "http://www.sindresorhus.com/?foo=bar"
    )]
    fn remove_query_parameters_tests_2(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default()
            .strip_www(false)
            .remove_query_parameters(RemoveQueryParametersOptions::List(vec![
                Regex::new(r"^utm_\w+").unwrap(),
                Regex::new("ref").unwrap(),
            ]))
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("http://www.sindresorhus.com", "http://www.sindresorhus.com")]
    #[case("www.sindresorhus.com?foo=bar", "http://www.sindresorhus.com")]
    #[case(
        "www.sindresorhus.com?foo=bar&utm_medium=test&ref=test_ref",
        "http://www.sindresorhus.com"
    )]
    fn remove_query_parameters_tests_3(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default()
            .strip_www(false)
            .remove_query_parameters(RemoveQueryParametersOptions::All)
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("http://www.sindresorhus.com", "http://www.sindresorhus.com")]
    #[case("www.sindresorhus.com?foo=bar", "http://www.sindresorhus.com/?foo=bar")]
    #[case(
        "www.sindresorhus.com?foo=bar&utm_medium=test&ref=test_ref",
        "http://www.sindresorhus.com/?foo=bar&ref=test_ref&utm_medium=test"
    )]
    fn remove_query_parameters_tests_4(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default()
            .strip_www(false)
            .remove_query_parameters(RemoveQueryParametersOptions::None)
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("https://sindresorhus.com", "https://sindresorhus.com")]
    #[case("www.sindresorhus.com?foo=bar", "http://www.sindresorhus.com")]
    #[case(
        "www.sindresorhus.com?foo=bar&utm_medium=test&ref=test_ref",
        "http://www.sindresorhus.com/?ref=test_ref&utm_medium=test"
    )]
    fn keep_query_parameters_tests(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default()
            .strip_www(false)
            .remove_query_parameters(RemoveQueryParametersOptions::None)
            .keep_query_parameters(vec![
                Regex::new(r"^utm_\w+").unwrap(),
                Regex::new("ref").unwrap(),
            ])
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("https://sindresorhus.com", "https://sindresorhus.com")]
    fn force_http_tests_1(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default().build().unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("http://sindresorhus.com", "http://sindresorhus.com")]
    #[case("https://www.sindresorhus.com", "http://sindresorhus.com")]
    #[case("//sindresorhus.com", "http://sindresorhus.com")]
    fn force_http_tests_2(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default().force_http(true).build().unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("https://sindresorhus.com", "https://sindresorhus.com")]
    fn force_https_tests_1(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default().build().unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("http://sindresorhus.com", "https://sindresorhus.com")]
    #[case("https://www.sindresorhus.com", "https://sindresorhus.com")]
    #[case("//sindresorhus.com", "https://sindresorhus.com")]
    fn force_https_tests_2(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default().force_https(true).build().unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("http://sindresorhus.com", "http://sindresorhus.com")]
    #[case("http://sindresorhus.com/", "http://sindresorhus.com")]
    #[case("http://sindresorhus.com/redirect", "http://sindresorhus.com/redirect")]
    #[case(
        "http://sindresorhus.com/redirect/",
        "http://sindresorhus.com/redirect"
    )]
    #[case("http://sindresorhus.com/#/", "http://sindresorhus.com/#/")]
    #[case(
        "http://sindresorhus.com/?unicorns=true",
        "http://sindresorhus.com/?unicorns=true"
    )]
    fn remove_trailing_slash_tests_1(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default().build().unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("http://sindresorhus.com", "http://sindresorhus.com")]
    #[case("http://sindresorhus.com/", "http://sindresorhus.com")]
    #[case(
        "http://sindresorhus.com/redirect/",
        "http://sindresorhus.com/redirect/"
    )]
    #[case("http://sindresorhus.com/#/", "http://sindresorhus.com/#/")]
    #[case(
        "http://sindresorhus.com/?unicorns=true",
        "http://sindresorhus.com/?unicorns=true"
    )]
    fn remove_trailing_slash_tests_2(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default()
            .remove_trailing_slash(false)
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("http://sindresorhus.com:123", "http://sindresorhus.com")]
    #[case("https://sindresorhus.com:123", "https://sindresorhus.com")]
    #[case("http://sindresorhus.com:443", "http://sindresorhus.com")]
    #[case("https://sindresorhus.com:80", "https://sindresorhus.com")]
    fn remove_explicit_port(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default()
            .remove_explicit_port(true)
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("https://sindresorhus.com", "https://sindresorhus.com")]
    #[case("https://sindresorhus.com/", "https://sindresorhus.com/")]
    #[case(
        "https://sindresorhus.com/redirect",
        "https://sindresorhus.com/redirect"
    )]
    #[case(
        "https://sindresorhus.com/redirect/",
        "https://sindresorhus.com/redirect"
    )]
    #[case("https://sindresorhus.com/#/", "https://sindresorhus.com/#/")]
    #[case(
        "https://sindresorhus.com/?unicorns=true",
        "https://sindresorhus.com/?unicorns=true"
    )]
    fn remove_single_slash_tests(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default()
            .remove_single_slash(false)
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("https://sindresorhus.com", "https://sindresorhus.com")]
    #[case("https://sindresorhus.com/", "https://sindresorhus.com/")]
    #[case(
        "https://sindresorhus.com/redirect",
        "https://sindresorhus.com/redirect"
    )]
    #[case(
        "https://sindresorhus.com/redirect/",
        "https://sindresorhus.com/redirect/"
    )]
    #[case("https://sindresorhus.com/#/", "https://sindresorhus.com/#/")]
    #[case(
        "https://sindresorhus.com/?unicorns=true",
        "https://sindresorhus.com/?unicorns=true"
    )]
    fn remove_single_slash_with_remove_trailing_slash_tests(
        #[case] input: String,
        #[case] expected: String,
    ) {
        let options = OptionsBuilder::default()
            .remove_single_slash(false)
            .remove_trailing_slash(false)
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(
        "http://sindresorhus.com/index.html",
        "http://sindresorhus.com/index.html"
    )]
    #[case(
        "http://sindresorhus.com/path/index.html",
        "http://sindresorhus.com/path/index.html"
    )]
    fn remove_directory_index_tests_1(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default().build().unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("http://sindresorhus.com/index.html", "http://sindresorhus.com")]
    #[case(
        "http://sindresorhus.com/index.htm",
        "http://sindresorhus.com/index.htm"
    )]
    #[case("http://sindresorhus.com/index.php", "http://sindresorhus.com")]
    #[case(
        "http://sindresorhus.com/path/index.html",
        "http://sindresorhus.com/path"
    )]
    #[case(
        "http://sindresorhus.com/path/index.htm",
        "http://sindresorhus.com/path/index.htm"
    )]
    #[case(
        "http://sindresorhus.com/path/index.php",
        "http://sindresorhus.com/path"
    )]
    #[case(
        "http://sindresorhus.com/foo/bar/index.html",
        "http://sindresorhus.com/foo/bar"
    )]
    fn remove_directory_index_tests_2(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default()
            .remove_directory_index(RemoveDirectoryIndexOptions::List(vec![
                Regex::new(r"index\.html").unwrap(),
                Regex::new(r"index\.php").unwrap(),
            ]))
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("http://sindresorhus.com/index.html", "http://sindresorhus.com")]
    #[case(
        "http://sindresorhus.com/index/index.html",
        "http://sindresorhus.com/index"
    )]
    #[case("http://sindresorhus.com/remove.html", "http://sindresorhus.com")]
    #[case(
        "http://sindresorhus.com/default.htm",
        "http://sindresorhus.com/default.htm"
    )]
    #[case("http://sindresorhus.com/index.php", "http://sindresorhus.com")]
    fn remove_directory_index_tests_3(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default()
            .remove_directory_index(RemoveDirectoryIndexOptions::List(vec![
                Regex::new(r"^index\.[a-z]+$").unwrap(),
                Regex::new(r"remove\.html").unwrap(),
            ]))
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("http://sindresorhus.com/index.html", "http://sindresorhus.com")]
    #[case("http://sindresorhus.com/index.htm", "http://sindresorhus.com")]
    #[case("http://sindresorhus.com/index.php", "http://sindresorhus.com")]
    fn remove_directory_index_tests_4(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default()
            .remove_directory_index(RemoveDirectoryIndexOptions::Default)
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("http://sindresorhus.com/path/", "http://sindresorhus.com/path")]
    #[case(
        "http://sindresorhus.com/path/index.html",
        "http://sindresorhus.com/path"
    )]
    #[case("http://sindresorhus.com/#/path/", "http://sindresorhus.com/#/path/")]
    #[case(
        "http://sindresorhus.com/foo/#/bar/",
        "http://sindresorhus.com/foo#/bar/"
    )]
    fn remove_trailing_slash_and_directory_index_tests_1(
        #[case] input: String,
        #[case] expected: String,
    ) {
        let options = OptionsBuilder::default()
            .remove_directory_index(RemoveDirectoryIndexOptions::Default)
            .remove_trailing_slash(true)
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("http://sindresorhus.com/path/", "http://sindresorhus.com/path/")]
    #[case(
        "http://sindresorhus.com/path/index.html",
        "http://sindresorhus.com/path/"
    )]
    #[case("http://sindresorhus.com/#/path/", "http://sindresorhus.com/#/path/")]
    fn remove_trailing_slash_and_directory_index_tests_2(
        #[case] input: String,
        #[case] expected: String,
    ) {
        let options = OptionsBuilder::default()
            .remove_directory_index(RemoveDirectoryIndexOptions::Default)
            .remove_trailing_slash(false)
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(
        "http://sindresorhus.com/?a=Z&b=Y&c=X&d=W",
        "http://sindresorhus.com/?a=Z&b=Y&c=X&d=W"
    )]
    #[case(
        "http://sindresorhus.com/?b=Y&c=X&a=Z&d=W",
        "http://sindresorhus.com/?a=Z&b=Y&c=X&d=W"
    )]
    #[case(
        "http://sindresorhus.com/?a=Z&d=W&b=Y&c=X",
        "http://sindresorhus.com/?a=Z&b=Y&c=X&d=W"
    )]
    #[case("http://sindresorhus.com/", "http://sindresorhus.com")]
    fn sort_query_parameters_tests_1(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default()
            .sort_query_parameters(true)
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(
        "http://sindresorhus.com/?a=Z&b=Y&c=X&d=W",
        "http://sindresorhus.com/?a=Z&b=Y&c=X&d=W"
    )]
    #[case(
        "http://sindresorhus.com/?b=Y&c=X&a=Z&d=W",
        "http://sindresorhus.com/?b=Y&c=X&a=Z&d=W"
    )]
    #[case(
        "http://sindresorhus.com/?a=Z&d=W&b=Y&c=X",
        "http://sindresorhus.com/?a=Z&d=W&b=Y&c=X"
    )]
    #[case("http://sindresorhus.com/", "http://sindresorhus.com")]
    fn sort_query_parameters_tests_2(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default()
            .sort_query_parameters(false)
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("http://")]
    #[case("/")]
    #[case("/relative/path/")]
    fn invalid_url_tests(#[case] input: String) {
        let options = OptionsBuilder::default().build().unwrap();
        let result = normalize_url(&input, &options).expect_err("Normalization succeeded");
        assert!(matches!(result, NormalizeUrlError::UnexpectedError(_)));
    }

    #[rstest]
    #[case(
        "http://sindresorhus.com////foo/bar",
        "http://sindresorhus.com/foo/bar"
    )]
    #[case(
        "http://sindresorhus.com////foo////bar",
        "http://sindresorhus.com/foo/bar"
    )]
    #[case(
        "http://sindresorhus.com:5000///foo",
        "http://sindresorhus.com:5000/foo"
    )]
    #[case("http://sindresorhus.com///foo", "http://sindresorhus.com/foo")]
    #[case(
        "http://sindresorhus.com:5000//foo",
        "http://sindresorhus.com:5000/foo"
    )]
    #[case("http://sindresorhus.com//foo", "http://sindresorhus.com/foo")]
    #[case(
        "http://sindresorhus.com/s3://sindresorhus.com",
        "http://sindresorhus.com/s3://sindresorhus.com"
    )]
    #[case(
        "http://sindresorhus.com/s3://sindresorhus.com//foo",
        "http://sindresorhus.com/s3://sindresorhus.com/foo"
    )]
    #[case(
        "http://sindresorhus.com//foo/s3://sindresorhus.com",
        "http://sindresorhus.com/foo/s3://sindresorhus.com"
    )]
    #[case(
        "http://sindresorhus.com/git://sindresorhus.com",
        "http://sindresorhus.com/git://sindresorhus.com"
    )]
    #[case(
        "http://sindresorhus.com/git://sindresorhus.com//foo",
        "http://sindresorhus.com/git://sindresorhus.com/foo"
    )]
    #[case(
        "http://sindresorhus.com//foo/git://sindresorhus.com//foo",
        "http://sindresorhus.com/foo/git://sindresorhus.com/foo"
    )]
    #[case(
        "http://sindresorhus.com/a://sindresorhus.com//foo",
        "http://sindresorhus.com/a:/sindresorhus.com/foo"
    )]
    #[case("http://sindresorhus.com/alongprotocolwithin50charlimitxxxxxxxxxxxxxxxxxxxx://sindresorhus.com//foo", "http://sindresorhus.com/alongprotocolwithin50charlimitxxxxxxxxxxxxxxxxxxxx://sindresorhus.com/foo")]
    #[case("http://sindresorhus.com/alongprotocolexceeds50charlimitxxxxxxxxxxxxxxxxxxxxx://sindresorhus.com//foo", "http://sindresorhus.com/alongprotocolexceeds50charlimitxxxxxxxxxxxxxxxxxxxxx:/sindresorhus.com/foo")]
    #[case(
        "http://sindresorhus.com/a2-.+://sindresorhus.com",
        "http://sindresorhus.com/a2-.+://sindresorhus.com"
    )]
    #[case(
        "http://sindresorhus.com/a2-.+_://sindresorhus.com",
        "http://sindresorhus.com/a2-.+_:/sindresorhus.com"
    )]
    #[case(
        "http://sindresorhus.com/2abc://sindresorhus.com",
        "http://sindresorhus.com/2abc:/sindresorhus.com"
    )]
    fn remove_duplicate_slashes_tests(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default().build().unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("https://ebаy.com", "https://xn--eby-7cd.com")]
    fn prevents_homograph_attacks(#[case] input: String, #[case] expected: String) {
        let options = OptionsBuilder::default()
            .sort_query_parameters(true)
            .build()
            .unwrap();
        let result = normalize_url(&input, &options).expect("Normalization failed");
        assert_eq!(result, expected);
    }

    #[test]
    fn returns_error_if_force_http_and_force_https_are_both_set() {
        let result = normalize_url(
            "",
            &OptionsBuilder::default()
                .force_http(true)
                .force_https(true)
                .build()
                .unwrap(),
        );
        assert_eq!(result.is_err(), true);
        assert!(matches!(
            result,
            Err(NormalizeUrlError::ForceHttpAndHttpAreExclusive)
        ));
    }
}
