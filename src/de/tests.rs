use super::from_str;

#[test]
fn one_element() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct Document {
        value: String,
    }
    
    let expected = Document { value: "plain text".to_string() };
    
    let input = r"<document><value>plain text</value></document>";
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn nested_elements() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct Document {
        inner: InnerElement,
    }
    
    #[derive(Debug, PartialEq, Deserialize)]
    struct InnerElement {
        value: String,
    }
    
    let expected = Document { inner: InnerElement { value: "plain text".to_string() } };
    
    let input = r"
        <document>
            <inner>
                <value>plain text</value>
            </inner>
        </document>";
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn multiple_elements() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct Document {
        first: String,
        second: String,
    }
    
    let expected = Document {
        first: "plain text".to_string(),
        second: "more text".to_string(),
    };
    
    let input = r"
        <document>
            <first>plain text</first>
            <second>more text</second>
        </document>";
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
#[ignore]
fn sequence() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct Document {
        #[serde(rename = "item")]
        items: Vec<String>,
    }
    
    let expected = Document {
        items: vec!["first".to_string(), "second".to_string(), "third".to_string()],
    };
    
    let input = r"
        <document>
            <items>
                <item>first</item>
                <item>second</item>
                <item>third</item>
            </items>
        </document>";
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn unit_variant() {
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    enum ABC {
        A, B, C
    }
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Document {
        content: ABC,
    }
    
    let expected = Document {
        content: ABC::A,
    };
    
    let input = r"
        <document>
            <content>a</content>
        </document>";
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn struct_variant() {
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    enum Suit {
        CLUBS, DIAMONDS, HEARTS, SPADES,
    }
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    enum Rank {
        ACE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, TEN, JACK, KNIGHT, QUEEN, KING
    }
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    enum Card {
        Trump { number: u8 }, Fool, Suited { suit: Suit, rank: Rank },
    }
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Document {
        content: Card,
    }
    
    let expected = Document {
        content: Card::Trump { number: 21 },
    };
    
    let input = r"
        <document>
            <content><trump><number>21</number></trump></content>
        </document>";
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn newtype_variant() {
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    enum Value {
        I(i64),
        F(f64),
        S(String),
    }
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Document {
        content: Value,
    }
    
    let expected = Document {
        content: Value::I(42),
    };
    
    let input = r#"
        <document>
            <content>
                <i>42</i>
            </content>
        </document>
    "#;
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}

#[test]
fn tuple_variant() {
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    enum Value {
        I(i64),
        F(f64),
        S(String),
        Kv(String, String),
    }
    
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Document {
        content: Value,
    }
    
    let expected = Document {
        content: Value::Kv("abc".to_string(), "123".to_string()),
    };
    
    let input = r#"
        <document>
            <content>
                <kv>abc 123</kv>
            </content>
        </document>
    "#;
    
    let actual: Document = from_str(input).unwrap();
    
    assert_eq!(expected, actual);
}