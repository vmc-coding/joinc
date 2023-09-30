use crate::error::{Error, Result};
use serde::{ser, Serialize};
use std::io;

pub struct Serializer<W, F> {
    writer: W,
    formatter: F,
    tags: Vec<&'static str>,
}

impl<W, F> Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    fn with_formatter(writer: W, formatter: F) -> Self {
        Serializer {
            writer,
            formatter,
            tags: Vec::with_capacity(8),
        }
    }

    fn finalize(&mut self) -> Result<()> {
        // closing root element
        if !self.tags.is_empty() {
            let tag = self.tags.pop().ok_or(Error::IllegalState)?;
            self.formatter
                .render_closing_struct_tag(&mut self.writer, tag)
                .map_err(Error::Io)?;
        }

        Ok(())
    }
}

pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: Serialize,
{
    to_writer_formatted(writer, CompactFormatter, value)
}

pub fn to_writer_formatted<W, F, T>(writer: W, formatter: F, value: &T) -> Result<()>
where
    W: io::Write,
    F: Formatter,
    T: Serialize,
{
    let mut ser = Serializer::with_formatter(writer, formatter);
    value.serialize(&mut ser)?;
    ser.finalize()
}

pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    to_vec_formatted(CompactFormatter, value)
}

pub fn to_vec_formatted<F, T>(formatter: F, value: &T) -> Result<Vec<u8>>
where
    F: Formatter,
    T: Serialize,
{
    let mut buffer = Vec::with_capacity(256);
    to_writer_formatted(&mut buffer, formatter, value)?;
    Ok(buffer)
}

impl<'a, W, F> ser::Serializer for &'a mut Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.formatter
            .render_opening_field_tag(
                &mut self.writer,
                self.tags.last().ok_or(Error::IllegalState)?,
            )
            .map_err(Error::Io)?;
        self.formatter
            .render_field_value(&mut self.writer, v)
            .map_err(Error::Io)?;
        self.formatter
            .render_closing_field_tag(
                &mut self.writer,
                self.tags.last().ok_or(Error::IllegalState)?,
            )
            .map_err(Error::Io)
    }

    fn serialize_none(self) -> Result<()> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        self.tags.push(name);
        self.formatter
            .render_opening_struct_tag(&mut self.writer, name)
            .map_err(Error::Io)?;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}

impl<'a, W, F> ser::SerializeSeq for &'a mut Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W, F> ser::SerializeTuple for &'a mut Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W, F> ser::SerializeTupleStruct for &'a mut Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W, F> ser::SerializeTupleVariant for &'a mut Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W, F> ser::SerializeMap for &'a mut Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W, F> ser::SerializeStruct for &'a mut Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.tags.push(key);
        value.serialize(&mut **self)?;
        self.tags.pop();
        Ok(())
    }

    fn end(self) -> Result<()> {
        let tag = self.tags.pop().ok_or(Error::IllegalState)?;
        self.formatter
            .render_closing_struct_tag(&mut self.writer, tag)
            .map_err(Error::Io)
    }
}

impl<'a, W, F> ser::SerializeStructVariant for &'a mut Serializer<W, F>
where
    W: io::Write,
    F: Formatter,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

// ----- Formatter -----

pub trait Formatter {
    fn render_opening_struct_tag<W>(&mut self, writer: &mut W, tag: &'static str) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(b"<")?;
        writer.write_all(tag.as_bytes())?;
        writer.write_all(b">")
    }

    fn render_closing_struct_tag<W>(&mut self, writer: &mut W, tag: &'static str) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(b"</")?;
        writer.write_all(tag.as_bytes())?;
        writer.write_all(b">")
    }

    fn render_opening_field_tag<W>(&mut self, writer: &mut W, tag: &'static str) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(b"<")?;
        writer.write_all(tag.as_bytes())?;
        writer.write_all(b">")
    }

    fn render_closing_field_tag<W>(&mut self, writer: &mut W, tag: &'static str) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(b"</")?;
        writer.write_all(tag.as_bytes())?;
        writer.write_all(b">")
    }

    fn render_field_value<W>(&mut self, writer: &mut W, v: &[u8]) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(v)
    }
}

pub struct CompactFormatter;

impl Formatter for CompactFormatter {}

pub struct PrettyFormatter<'a> {
    default_formatter: CompactFormatter,
    nl: &'a [u8],
    indent: &'a [u8],
    indention_level: usize,
}

impl<'a> PrettyFormatter<'a> {
    pub fn new() -> Self {
        Self::with_indent(b"  ")
    }

    pub fn with_indent(indent: &'a [u8]) -> Self {
        PrettyFormatter {
            default_formatter: CompactFormatter,
            nl: b"\n",
            indent,
            indention_level: 0,
        }
    }

    fn inc_indent(&mut self) {
        self.indention_level += 1;
    }

    fn dec_indent(&mut self) {
        self.indention_level -= 1;
    }

    fn render_indent<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(&self.indent.repeat(self.indention_level))
    }
}

impl<'a> Formatter for PrettyFormatter<'a> {
    fn render_opening_struct_tag<W>(&mut self, writer: &mut W, tag: &'static str) -> io::Result<()>
    where
        W: io::Write,
    {
        self.render_indent(writer)?;
        self.inc_indent();
        self.default_formatter
            .render_opening_struct_tag(writer, tag)?;
        writer.write_all(&self.nl)
    }

    fn render_closing_struct_tag<W>(&mut self, writer: &mut W, tag: &'static str) -> io::Result<()>
    where
        W: io::Write,
    {
        self.dec_indent();
        self.render_indent(writer)?;
        self.default_formatter
            .render_closing_struct_tag(writer, tag)?;
        writer.write_all(&self.nl)
    }

    fn render_opening_field_tag<W>(&mut self, writer: &mut W, tag: &'static str) -> io::Result<()>
    where
        W: io::Write,
    {
        self.render_indent(writer)?;
        self.default_formatter.render_opening_field_tag(writer, tag)
    }

    fn render_closing_field_tag<W>(&mut self, writer: &mut W, tag: &'static str) -> io::Result<()>
    where
        W: io::Write,
    {
        self.default_formatter
            .render_closing_field_tag(writer, tag)?;
        writer.write_all(&self.nl)
    }
}

// ----- Tests -----

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use serde_repr::Serialize_repr;

    #[test]
    fn serializes_structs() {
        #[derive(Serialize)]
        #[serde(rename = "inner")]
        struct InnerDTO {
            b: i32,
        }

        #[derive(Serialize)]
        #[serde(rename = "outer")]
        struct OuterDTO {
            a: u32,
            inner: InnerDTO,
        }

        let test = OuterDTO {
            a: 47,
            inner: InnerDTO { b: -11 },
        };

        let expected = "<outer><a>47</a><inner><b>-11</b></inner></outer>";
        assert_eq!(
            String::from_utf8(super::to_vec(&test).unwrap()).unwrap(),
            expected
        );
    }

    #[test]
    fn serializes_empty_struct() {
        #[derive(Serialize)]
        #[serde(rename = "iam_empty")]
        struct IamEmpty {}

        let test = IamEmpty {};

        // also valid would be <iam_empty/>, see https://boinc.berkeley.edu/trac/wiki/GuiRpcProtocol (Sep 2023):
        // "Self-closing tags must not have a space before the slash, [..]"
        let expected = "<iam_empty></iam_empty>";
        assert_eq!(
            String::from_utf8(super::to_vec(&test).unwrap()).unwrap(),
            expected
        );
    }

    #[test]
    fn serializes_enums() {
        #[derive(Serialize_repr)]
        #[repr(u8)]
        enum Color {
            Red = 5,
            Blue = 7,
        }

        #[derive(Serialize)]
        #[serde(rename = "dto")]
        struct Dto {
            a_color: Color,
            another_color: Color,
        }

        let test = Dto {
            a_color: Color::Red,
            another_color: Color::Blue,
        };

        let expected = "<dto><a_color>5</a_color><another_color>7</another_color></dto>";
        assert_eq!(
            String::from_utf8(super::to_vec(&test).unwrap()).unwrap(),
            expected
        );
    }

    #[test]
    fn serializes_sequences() {
        #[derive(Serialize)]
        #[serde(rename = "dto")]
        struct Dto {
            number: Vec<i32>,
        }

        let test = Dto {
            number: vec![2, 3, 5],
        };

        let expected = "<dto><number>2</number><number>3</number><number>5</number></dto>";
        assert_eq!(
            String::from_utf8(super::to_vec(&test).unwrap()).unwrap(),
            expected
        );
    }

    #[test]
    fn serializes_sequences2() {
        #[derive(Serialize)]
        #[serde(rename = "msg")]
        struct Msg {
            id: u32,
            body: String,
        }

        #[derive(Serialize)]
        #[serde(rename = "msgs")]
        struct Msgs {
            msg: Vec<Msg>,
        }

        let test = Msgs {
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

        let expected = "<msgs><msg><id>2</id><body>foo</body></msg><msg><id>5</id><body>bar</body></msg></msgs>";
        assert_eq!(
            String::from_utf8(super::to_vec(&test).unwrap()).unwrap(),
            expected
        );
    }

    #[test]
    fn prints_pretty() {
        use crate::ser::PrettyFormatter;

        #[derive(Serialize)]
        #[serde(rename = "inner")]
        struct InnerDTO {
            b: i32,
        }

        #[derive(Serialize)]
        #[serde(rename = "outer")]
        struct OuterDTO {
            a: u32,
            inner: InnerDTO,
        }

        let test = OuterDTO {
            a: 47,
            inner: InnerDTO { b: -11 },
        };

        // yep, I know, it has a redundant newline
        let expected = "\
<outer>
  <a>47</a>
  <inner>
    <b>-11</b>
  </inner>
</outer>
";
        assert_eq!(
            String::from_utf8(
                super::to_vec_formatted(PrettyFormatter::with_indent(b"  "), &test).unwrap()
            )
            .unwrap(),
            expected
        );
    }
}
