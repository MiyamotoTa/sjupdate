use semver::Version;

use crate::rss::FeedItem;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Release {
    pub version: Version,
    pub link: String,
}

pub fn convert_to_release(feed_items: Vec<FeedItem>) -> Vec<Release> {
    feed_items
        .iter()
        .map(|item| Release {
            version: lenient_semver::parse(&item.title).unwrap(),
            link: item.link.clone(),
        })
        .collect()
}

pub fn find_latest_release(releases: Vec<Release>) -> Option<Release> {
    releases
        .into_iter()
        .max_by(|a, b| a.version.cmp(&b.version))
}

#[cfg(test)]
mod tests {
    use semver::{BuildMetadata, Prerelease, Version};

    use super::{convert_to_release, find_latest_release, FeedItem, Release};

    #[test]
    fn test_convert_to_release() {
        let feed_items = vec![
            FeedItem {
                title: "0.0.0".to_string(),
                link: "https://example.com/0.0.0".to_string(),
            },
            FeedItem {
                title: "1.0.0".to_string(),
                link: "https://example.com/1.0.0".to_string(),
            },
            FeedItem {
                title: "v1.1.0".to_string(),
                link: "https://example.com/v1.1.0".to_string(),
            },
            FeedItem {
                title: "1.1.0-a".to_string(),
                link: "https://example.com/1.1.0-a".to_string(),
            },
        ];

        let releases = convert_to_release(feed_items);

        assert_eq!(releases.len(), 4);
        assert_eq!(releases[0].version, Version::new(0, 0, 0));
        assert_eq!(releases[0].link, "https://example.com/0.0.0");
        assert_eq!(releases[1].version, Version::new(1, 0, 0));
        assert_eq!(releases[1].link, "https://example.com/1.0.0");
        assert_eq!(releases[2].version, Version::new(1, 1, 0));
        assert_eq!(releases[2].link, "https://example.com/v1.1.0");
        assert_eq!(
            releases[3].version,
            Version {
                major: 1,
                minor: 1,
                patch: 0,
                pre: Prerelease::new("a").unwrap(),
                build: BuildMetadata::EMPTY,
            }
        );
        assert_eq!(releases[3].link, "https://example.com/1.1.0-a");
    }

    #[test]
    fn test_find_latest_release() {
        let release_0_0_0 = Release {
            version: Version::new(0, 0, 0),
            link: "https://example.com/0.0.0".to_string(),
        };
        let release_1_0_0 = Release {
            version: Version::new(1, 0, 0),
            link: "https://example.com/1.0.0".to_string(),
        };
        let release_1_1_0 = Release {
            version: Version::new(1, 1, 0),
            link: "https://example.com/v1.1.0".to_string(),
        };
        let release_1_1_0_a = Release {
            version: Version {
                major: 1,
                minor: 1,
                patch: 0,
                pre: Prerelease::new("a").unwrap(),
                build: BuildMetadata::EMPTY,
            },
            link: "https://example.com/1.1.0-a".to_string(),
        };
        let release_1_1_0_b = Release {
            version: Version {
                major: 1,
                minor: 1,
                patch: 0,
                pre: Prerelease::new("b").unwrap(),
                build: BuildMetadata::EMPTY,
            },
            link: "https://example.com/1.1.0-b".to_string(),
        };

        let releases = vec![
            release_0_0_0,
            release_1_0_0,
            release_1_1_0.clone(),
            release_1_1_0_a,
            release_1_1_0_b,
        ];

        // The pre-release version is considered less than the normal version
        assert_eq!(find_latest_release(releases), Some(release_1_1_0));
    }

    #[test]
    fn test_find_latest_release_empty() {
        let releases = vec![];
        assert_eq!(find_latest_release(releases), None);
    }
}
