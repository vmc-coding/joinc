use crate::error::Result;
use crate::Error;
use serde::Deserialize;

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    quick_xml::de::from_str(s).map_err(|err| Error::UnexpectedXml(err.to_string()))
}

// ----- Tests -----

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_repr::Deserialize_repr;

    #[test]
    fn deserializes_structs() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct InnerDTO {
            b: i32,
        }

        #[derive(Deserialize, Debug, PartialEq)]
        struct OuterDTO {
            a: u32,
            inner: InnerDTO,
        }

        let expected = OuterDTO {
            a: 47,
            inner: InnerDTO { b: -11 },
        };

        let xml = "<outer><a>47</a><inner><b>-11</b></inner></outer>";
        let deserialized: OuterDTO = super::from_str(xml).unwrap();

        assert_eq!(deserialized, expected);
    }

    #[test]
    fn deserializes_enums() {
        #[derive(Deserialize_repr, Debug, PartialEq)]
        #[repr(u8)]
        enum Color {
            Red = 5,
            Blue = 7,
        }

        #[derive(Deserialize, Debug, PartialEq)]
        struct Dto {
            a_color: Color,
            another_color: Color,
        }

        let expected = Dto {
            a_color: Color::Red,
            another_color: Color::Blue,
        };

        let xml = "<dto><a_color>5</a_color><another_color>7</another_color></dto>";
        let deserialized: Dto = super::from_str(xml).unwrap();

        assert_eq!(deserialized, expected);
    }

    #[test]
    fn deserializes_sequences() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Dto {
            number: Vec<i32>,
        }

        let expected = Dto {
            number: vec![2, 3, 5],
        };

        let xml = "<dto><number>2</number><number>3</number><number>5</number></dto>";
        let deserialized: Dto = super::from_str(xml).unwrap();

        assert_eq!(deserialized, expected);
    }

    #[test]
    fn deserializes_sequences2() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Msg {
            id: u32,
            body: String,
        }

        #[derive(Deserialize, Debug, PartialEq)]
        struct Msgs {
            msg: Vec<Msg>,
        }

        let expected = Msgs {
            msg: vec![
                Msg {
                    id: 2,
                    body: "foo".to_string(),
                },
                Msg {
                    id: 5,
                    body: "bar".to_string(),
                },
            ],
        };

        let xml = "<msgs><msg><id>2</id><body>foo</body></msg><msg><id>5</id><body>bar</body></msg></msgs>";
        let deserialized: Msgs = super::from_str(xml).unwrap();

        assert_eq!(deserialized, expected);
    }

    #[test]
    fn deserializes_sequences3() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct GuiUrl {
            name: String
        }

        #[derive(Deserialize, Debug, PartialEq)]
        struct GuiUrlsDto {
            gui_url: Vec<GuiUrl>
        }

        #[derive(Deserialize, Debug, PartialEq)]
        #[serde(from = "GuiUrlsDto")]
        struct GuiUrls(pub Vec<GuiUrl>);

        impl From<GuiUrlsDto> for GuiUrls {
            fn from(dto: GuiUrlsDto) -> Self {
                GuiUrls ( dto.gui_url )
            }
        }

        #[derive(Deserialize, Debug, PartialEq)]
        struct Project {
            url: String,
            gui_urls: GuiUrls,
        }

        #[derive(Deserialize, Debug, PartialEq)]
        struct Projects {
            project: Vec<Project>
        }

        let expected = Projects {
            project: vec![
                Project {
                    url: "p1".to_string(),
                    gui_urls: GuiUrls (vec![
                        GuiUrl { name : "u1.1".to_string() },
                        GuiUrl { name : "u1.2".to_string() },
                    ])
                },
                Project {
                    url: "p2".to_string(),
                    gui_urls: GuiUrls (vec![
                        GuiUrl { name : "u2.1".to_string() },
                        GuiUrl { name : "u2.3".to_string() },
                    ])
                },
            ],
        };

        let xml = "\
            <projects>
                <project>
                    <url>p1</url>
                    <gui_urls>
                        <gui_url><name>u1.1</name></gui_url>
                        <gui_url><name>u1.2</name></gui_url>
                    </gui_urls>
                </project>
                <project>
                    <url>p2</url>
                    <gui_urls>
                        <gui_url><name>u2.1</name></gui_url>
                        <ifteam>
                            <gui_url><name>u2.2</name></gui_url>
                        </ifteam>
                        <gui_url><name>u2.3</name></gui_url>
                    </gui_urls>
                </project>
            </projects>";
        let deserialized: Projects = super::from_str(xml).unwrap();

        assert_eq!(deserialized, expected);
    }
}
