# TODO

* Create stream variants of `@reader` handlers in `byte_layout!{}`, requiring only stream that implements trait `std::io::Read`
* Create stream variant of `parse_bytes::<I,E>(&[u8])` as `parse_bytes_stream::<I,E>(std::io::Read)`
* Design flush handler for chunk when buffer full or SIGINT/SIGTERM received
* Design cache flush to chunk in store mechanism
* Finish design document
