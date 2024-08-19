use thiserror::Error;
use url::Url;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PageUrlKind {
    Artist,
    Album,
    Track,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PageUrl {
    pub kind: PageUrlKind,
    pub url: Url,
}

#[derive(Error, Debug)]
pub enum UrlParseError {
    #[error("invalid domain")]
    InvalidDomain,
    #[error("invalid path")]
    InvalidPath,
    #[error(transparent)]
    Url(#[from] url::ParseError),
}

pub fn url_from_artist(artist: &str) -> Result<Url, url::ParseError> {
    Url::parse(format!("https://{artist}.bandcamp.com/music").as_str())
}

pub fn parse_url(url: &Url) -> Result<PageUrl, UrlParseError> {
    let (artist, domain) = url
        .domain()
        .ok_or(UrlParseError::InvalidDomain)?
        .split_once('.')
        .ok_or(UrlParseError::InvalidDomain)?;

    if domain != "bandcamp.com" {
        Err(UrlParseError::InvalidDomain)?;
    };

    let mut path_segments =
        url.path_segments().ok_or(UrlParseError::InvalidPath)?;
    match path_segments.next().ok_or(UrlParseError::InvalidPath)? {
        "" | "music" => {
            let url = Url::parse(
                format!("https://{artist}.bandcamp.com/music").as_str(),
            )?;
            Ok(PageUrl { kind: PageUrlKind::Artist, url })
        }
        "album" => {
            let url = path_segments
                .next()
                .ok_or(UrlParseError::InvalidPath)
                .map(|album| {
                format!("https://{artist}.bandcamp.com/album/{album}")
            })?;

            let url = Url::parse(&url)?;
            Ok(PageUrl { kind: PageUrlKind::Album, url })
        }
        "track" => {
            let url = path_segments
                .next()
                .ok_or(UrlParseError::InvalidPath)
                .map(|track| {
                format!("https://{artist}.bandcamp.com/track/{track}")
            })?;

            let url = Url::parse(&url)?;
            Ok(PageUrl { kind: PageUrlKind::Track, url })
        }
        _ => Err(UrlParseError::InvalidPath),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_from_artist() {
        assert_eq!(
            url_from_artist("foo").unwrap().as_str(),
            "https://foo.bandcamp.com/music"
        );
    }

    #[test]
    fn test_parse_url_artist() {
        let url = Url::parse("https://artist.bandcamp.com/").unwrap();
        let page = parse_url(&url).unwrap();
        assert_eq!(page.kind, PageUrlKind::Artist);
        assert_eq!(page.url.as_str(), "https://artist.bandcamp.com/music");
    }

    #[test]
    fn test_parse_url_artist_music() {
        let url = Url::parse("https://artist.bandcamp.com/music").unwrap();
        let page = parse_url(&url).unwrap();
        assert_eq!(page.kind, PageUrlKind::Artist);
        assert_eq!(page.url.as_str(), "https://artist.bandcamp.com/music");
    }

    #[test]
    fn test_parse_url_album() {
        let url = Url::parse("https://artist.bandcamp.com/album/foo").unwrap();
        let page = parse_url(&url).unwrap();
        assert_eq!(page.kind, PageUrlKind::Album);
        assert_eq!(page.url.as_str(), "https://artist.bandcamp.com/album/foo");
    }

    #[test]
    fn test_parse_url_track() {
        let url = Url::parse("https://artist.bandcamp.com/track/foo").unwrap();
        let page = parse_url(&url).unwrap();
        assert_eq!(page.kind, PageUrlKind::Track);
        assert_eq!(page.url.as_str(), "https://artist.bandcamp.com/track/foo");
    }

    #[test]
    fn test_parse_url_missing_artist() {
        let url = Url::parse("https://bandcamp.com/").unwrap();
        assert!(parse_url(&url).is_err());
    }

    #[test]
    fn test_parse_url_invalid_domain() {
        let url = Url::parse("https://artist.notbandcamp.com/music").unwrap();
        assert!(parse_url(&url).is_err());
    }

    #[test]
    fn test_parse_url_invalid_path() {
        let url = Url::parse("https://artist.bandcamp.com/invalid").unwrap();
        assert!(parse_url(&url).is_err());
    }
}
