# libjoincserde

The serde library of project joinc implementing (de)serializing
for the GUI RPC protocol used to communicate with the BOINC clients.

It's based on the [serde](https://serde.rs/) framework
implementing trait [Serializer](https://docs.rs/serde/latest/serde/trait.Serializer.html)
and using [quick-xml](https://docs.rs/quick-xml/latest/quick_xml/de/index.html) as deserializer.

## dependencies

- [quick-xml](https://docs.rs/quick-xml/)
- [serde](https://serde.rs/)
