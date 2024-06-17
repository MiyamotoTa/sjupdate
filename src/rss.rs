use anyhow::Result;
use bytes::Bytes;
use rss::Channel;

#[derive(Debug)]
pub struct FeedItem {
    pub title: String,
    pub link: String,
}

pub fn get_feed_items(url: &str) -> Result<Vec<FeedItem>> {
    let content = get_content(url)?;
    let channel = Channel::read_from(&content[..])?;

    let feed_items: Vec<FeedItem> = channel
        .items()
        .iter()
        .map(|item| FeedItem {
            title: item.title().unwrap().to_string(),
            link: item.link().unwrap().to_string(),
        })
        .collect();

    Ok(feed_items)
}

#[tokio::main]
async fn get_content(url: &str) -> Result<Bytes> {
    let content = reqwest::get(url).await?.bytes().await?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use mockito::Server;

    use super::get_feed_items;

    #[test]
    fn test_get_feed_items() {
        let mut server = Server::new();
        let path = "/tags.rss";
        let xml_body = r#"
<?xml version="1.0" encoding="UTF-8"?><rss version="2.0" xmlns:content="http://purl.org/rss/1.0/modules/content/">
  <channel>
    <title>Example title</title>
    <link>https://example.com/sheetjs/sheetjs/tags</link>
    <description>📗 SheetJS Community Edition -- Spreadsheet Data Toolkit</description>
    <pubDate>Sat, 08 Jun 2024 11:44:36 +0000</pubDate>
    <item>
      <title>v0.20.2</title>
      <link>https://example.com/sheetjs/sheetjs/releases/tag/v0.20.2</link>
      <description></description>
      <content:encoded><![CDATA[<p dir="auto">see <a href="https://regexide.com" rel="nofollow">https://regexide.com</a> for more details</p>
]]></content:encoded>
      <author>sheetjs</author>
      <guid>3684: https://example.com/sheetjs/sheetjs/releases/tag/v0.20.2</guid>
      <pubDate>Fri, 05 Apr 2024 01:30:28 +0000</pubDate>
    </item>
    <item>
      <title>v0.20.1</title>
      <link>https://example.com/sheetjs/sheetjs/releases/tag/v0.20.1</link>
      <description></description>
      <author>sheetjs</author>
      <guid>2902: https://example.com/sheetjs/sheetjs/releases/tag/v0.20.1</guid>
      <pubDate>Tue, 05 Dec 2023 08:19:42 +0000</pubDate>
    </item>
    <item>
      <title>v0.7.6-a</title>
      <link>https://example.com/sheetjs/sheetjs/releases/tag/v0.7.6-a</link>
      <description></description>
      <author>sheetjs</author>
      <guid>83: https://example.com/sheetjs/sheetjs/releases/tag/v0.7.6-a</guid>
      <pubDate>Thu, 05 Jun 2014 07:25:49 +0000</pubDate>
    </item>
    <item>
      <title>v0.7.6</title>
      <link>https://example.com/sheetjs/sheetjs/releases/tag/v0.7.6</link>
      <description></description>
      <author>sheetjs</author>
      <guid>82: https://example.com/sheetjs/sheetjs/releases/tag/v0.7.6</guid>
      <pubDate>Thu, 05 Jun 2014 07:25:49 +0000</pubDate>
    </item>
  </channel>
</rss>
        "#;

        let mock = server
            .mock("GET", path)
            .with_status(200)
            .with_header("content-type", "application/xml")
            .with_body(xml_body)
            .create();

        let url = server.url() + path;
        let items = get_feed_items(&url).unwrap();

        assert_eq!(items.len(), 4);
        assert_eq!(items[0].title, "v0.20.2");
        assert_eq!(
            items[0].link,
            "https://example.com/sheetjs/sheetjs/releases/tag/v0.20.2"
        );
        assert_eq!(items[1].title, "v0.20.1");
        assert_eq!(
            items[1].link,
            "https://example.com/sheetjs/sheetjs/releases/tag/v0.20.1"
        );
        assert_eq!(items[2].title, "v0.7.6-a");
        assert_eq!(
            items[2].link,
            "https://example.com/sheetjs/sheetjs/releases/tag/v0.7.6-a"
        );
        assert_eq!(items[3].title, "v0.7.6");
        assert_eq!(
            items[3].link,
            "https://example.com/sheetjs/sheetjs/releases/tag/v0.7.6"
        );

        mock.assert();
    }
}
