pub mod settings {
    use std::env::home_dir;
    use serde_derive::{Deserialize, Serialize};
    use serde_json::{Value, Map};

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub(crate) struct Settings {
        pub home_dir: String
    }

    impl Default for Settings {
        fn default() -> Self {
            Self(
                home_dir
            )
        }
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
        assert_eq!(home_dir, "/Users/samspencer/");
    }
}