extern crate web_view;
extern crate rust_embed;

use web_view::*;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/app/build/"]
struct Asset;

pub fn new_webview() {
    const TITLE: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let window_title = format!("{} ({})", TITLE, VERSION);

    let css_content = Asset::get("app.css").unwrap();
    let js_content = Asset::get("app.js").unwrap();

    let html = format!(
        r#"
		<!doctype html>
		<html>
			<head>
				{styles}
			</head>
			<body>
				<!--[if lt IE 9]>
				<div class="ie-upgrade-container">
					<p class="ie-upgrade-message">Please, upgrade Internet Explorer to continue using this software.</p>
					<a class="ie-upgrade-link" target="_blank" href="https://www.microsoft.com/en-us/download/internet-explorer.aspx">Upgrade</a>
				</div>
				<![endif]-->
				<!--[if gte IE 9 | !IE ]> <!-->
				<div id="app"></div>
				{scripts}
				<![endif]-->
			</body>
		</html>
		"#,
        styles = inline_style(std::str::from_utf8(css_content.as_ref()).unwrap()),
        scripts = inline_script(std::str::from_utf8(js_content.as_ref()).unwrap())
    );

    #[cfg(debug_assertions)]
    let debug = true;
    #[cfg(not(debug_assertions))]
    let debug = false;

    let webview = web_view::builder()
        .title(window_title.as_ref())
        .content(Content::Html(html))
        .size(1600, 900)
        .resizable(true)
        .debug(debug)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .build()
        .unwrap();

    let res = webview.run().unwrap();

    println!("final state: {:?}", res);
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}