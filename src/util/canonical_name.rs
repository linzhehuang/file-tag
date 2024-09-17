use chrono::Local;
use regex::Regex;

#[derive(Clone)]
pub struct CanonicalName {
    original: String,
    base_name: String,
    version: u8,
    timestamp: String,
    tag: Option<String>,
    extension: String,
    canonical: bool,
}

impl ToString for CanonicalName {
    fn to_string(&self) -> String {
        let suffix = match &self.tag {
            Some(t) => format!(".{}", t),
            None => String::new(),
        };

        format!(
            "{}.v{}.{}{}.{}",
            self.base_name, self.version, self.timestamp, suffix, self.extension
        )
    }
}

impl CanonicalName {
    pub fn new(name: String) -> Self {
        let original = name.clone();
        let (name, extension) = Self::split(name);

        let parts: Vec<&str> = name.split(".").into_iter().collect();
        if parts.len() < 3 {
            return Self::default_with_name(original, name, extension);
        }

        let mut tag = None;
        let timestamp;
        let version;
        let base_name;

        let mut idx = parts.len() - 1;
        let mut part = parts[idx];

        let timestamp_regex = Regex::new(r"^\d{4}\d{2}\d{2}$").unwrap();
        if !timestamp_regex.is_match(part) {
            tag.replace(String::from(part));

            idx = idx - 1;
            part = parts[idx];
            if !timestamp_regex.is_match(part) {
                return Self::default_with_name(original, name, extension);
            }
        }
        timestamp = String::from(part);

        let version_regex = Regex::new(r"^v\d{1,2}$").unwrap();
        idx = idx - 1;
        let part = parts[idx];
        if !version_regex.is_match(part) {
            return Self::default_with_name(original, name, extension);
        }
        version = part.replace("v", "").parse::<u8>().unwrap();

        idx = idx - 1;
        base_name = parts[0..idx + 1].join(".");

        Self {
            original,
            base_name,
            version,
            timestamp,
            tag,
            extension,
            canonical: true,
        }
    }

    pub fn is_canonical(&self) -> bool {
        self.canonical
    }

    pub fn get_original(&self) -> String {
        self.original.clone()
    }

    pub fn to_upgrade_string(&self) -> String {
        let mut upgrade_self = self.to_owned().clone();
        upgrade_self.version = self.version + 1;
        upgrade_self.timestamp = Self::timestamp();
        upgrade_self.to_string()
    }

    fn split(name: String) -> (String, String) {
        let parts: Vec<&str> = name.split(".").into_iter().collect();
        (
            String::from(parts[0..parts.len() - 1].join(".")),
            String::from(parts[parts.len() - 1]),
        )
    }

    fn default_with_name(original: String, name: String, extension: String) -> Self {
        Self {
            original,
            base_name: String::from(name),
            version: 1,
            timestamp: Self::timestamp(),
            tag: None,
            extension: String::from(extension),
            canonical: false,
        }
    }

    fn timestamp() -> String {
        Local::now().format("%Y%m%d").to_string()
    }
}

#[test]
pub fn test_canonical_name() {
    let cname = CanonicalName::new(String::from("foo.txt"));
    assert_eq!(cname.is_canonical(), false);
    assert_eq!(cname.base_name, "foo");

    let cname = CanonicalName::new(String::from("foo.v1.20240916.txt"));
    assert_eq!(cname.is_canonical(), true);
    assert_eq!(cname.base_name, "foo");
    assert_eq!(cname.version, 1);
    assert_eq!(cname.timestamp, "20240916");
    assert_eq!(cname.extension, "txt");

    let cname = CanonicalName::new(String::from("foo.v1.20240916.tag.txt"));
    assert_eq!(cname.is_canonical(), true);
    assert_eq!(cname.base_name, "foo");
    assert_eq!(cname.version, 1);
    assert_eq!(cname.timestamp, "20240916");
    assert_eq!(cname.tag.unwrap(), "tag");
    assert_eq!(cname.extension, "txt");

    let cname = CanonicalName::new(String::from("foo.bar.v1.20240916.tag.txt"));
    assert_eq!(cname.is_canonical(), true);
    assert_eq!(cname.base_name, "foo.bar");
    assert_eq!(cname.version, 1);
    assert_eq!(cname.timestamp, "20240916");
    assert_eq!(cname.tag.unwrap(), "tag");
    assert_eq!(cname.extension, "txt");
}
