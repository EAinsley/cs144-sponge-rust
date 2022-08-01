use bytestream::ByteStream;

#[test]
fn byte_stream_construction() {
    let stream = ByteStream::new(15);
    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), true);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), 0);
    assert_eq!(stream.remaining_capacity(), 15);
    assert_eq!(stream.buffer_size(), 0);
}
#[test]
fn byte_stream_construction_end() {
    let mut stream = ByteStream::new(15);
    stream.end_input();
    assert_eq!(stream.input_ended(), true);
    assert_eq!(stream.buffer_empty(), true);
    assert_eq!(stream.eof(), true);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), 0);
    assert_eq!(stream.remaining_capacity(), 15);
    assert_eq!(stream.buffer_size(), 0);
}
