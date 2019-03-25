use std::io::Read;

use serde::de::IntoDeserializer;

use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

use super::Deserializer;
use super::plain::PlainStringDeserializer;
use super::super::error::{self, Error, Result};

pub struct MapAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    attributes: std::vec::IntoIter<OwnedAttribute>,
    value: Option<String>,
    end_tag: Option<String>,
}

impl<'a, R: 'a + Read> MapAccess<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>, attributes: Vec<OwnedAttribute>) -> Self {
        MapAccess {
            de,
            attributes: attributes.into_iter(),
            value: None,
            end_tag: None
        }
    }
}

impl<'a, 'de, R: 'a + Read> serde::de::MapAccess<'de> for MapAccess<'a, R> {
    type Error = Error;

    fn next_key_seed<K: serde::de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        match self.attributes.next() {
            Some(OwnedAttribute { name, value }) => {
                debug!("attribute {}='{}'", name, value);
                self.value = Some(value);
                let attribute_name = format!("@{}", name.local_name);
                seed.deserialize(attribute_name.into_deserializer()).map(Some)
            },
            None => match self.de.peek()? {
                XmlEvent::EndElement { .. } | XmlEvent::EndDocument => {
                    debug!("end of map");
                    Ok(None)
                },
                XmlEvent::Characters { .. } => {
                    let body = self.de.characters()?;
                    debug!("body '{}'", body);
                    self.value = Some(body);
                    seed.deserialize(".".into_deserializer()).map(Some)
                },
                XmlEvent::StartElement { .. } => {
                    let (tag_name, attributes) = self.de.start_tag()?;
                    self.de.tag_name = Some(tag_name.clone());
                    self.de.attributes = Some(attributes);
                    debug!("subtag {}", tag_name);
                    self.end_tag = Some(tag_name.clone());
                    seed.deserialize(tag_name.into_deserializer()).map(Some)
                },
                _ => Err(error::with_message(format!("expected map key, found {:?}", self.de.next()?))),
            },
        }
    }

    fn next_value_seed<V: serde::de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value> {
        match self.value.take() {
            Some(v) => seed.deserialize(PlainStringDeserializer(v)),
            None => {
                let v = seed.deserialize(&mut *self.de)?;
                let end_tag = self.end_tag.take().unwrap();
                debug!("end of subtag {}", end_tag);
                match self.de.peek()?.clone() {
                    XmlEvent::EndElement { ref name } if name.local_name == end_tag => {
                        self.de.end_tag(&end_tag)?;
                    },
                    _ => (),
                }
                Ok(v)
            }
        }
    }
}
