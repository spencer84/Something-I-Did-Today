pub mod settings {
    use std::env::home_dir;
    use serde_derive::{Deserialize, Serialize};
    use serde_json::{Value, Map};

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub(crate) struct Settings {
        #[serde(default = "default_home_dir")]
        pub home_dir: String
    }

    fn default_home_dir() -> String {
        home_dir().unwrap().to_str().unwrap().to_string()
    }

    pub fn read_settings() -> Settings {
        let settings_string = include_str!("../resources/settings.json");
        let settings: Settings = serde_json::from_str(settings_string).unwrap();
        settings
    }

    #[test]
    fn test_read_settings() {
        let settings = read_settings();
        let home_dir = settings.home_dir;
        println!("{:?}", home_dir);
        assert_eq!(home_dir, "/Users/samspencer");
    }

    #[test]
    fn test_default_home_dir() {
        let home_dir = default_home_dir();
        println!("{:?}", home_dir);
        assert_eq!(home_dir, "/Users/samspencer");

    }
}