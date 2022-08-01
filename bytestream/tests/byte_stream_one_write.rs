use bytestream::ByteStream;

#[test]
fn write_end_pop() {
    let mut stream = ByteStream::new(15);

    assert_eq!(stream.write("cat"), 3);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 12);
    assert_eq!(stream.buffer_size(), 3);
    assert_eq!(stream.peek_output(3), "cat");

    stream.end_input();

    assert_eq!(stream.input_ended(), true);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 12);
    assert_eq!(stream.buffer_size(), 3);
    assert_eq!(stream.peek_output(3), "cat");

    stream.pop_output(3);

    assert_eq!(stream.input_ended(), true);
    assert_eq!(stream.buffer_empty(), true);
    assert_eq!(stream.eof(), true);
    assert_eq!(stream.bytes_read(), 3);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 15);
    assert_eq!(stream.buffer_size(), 0);
}

#[test]
fn write_pop_end() {
    let mut stream = ByteStream::new(15);

    assert_eq!(stream.write("cat"), 3);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 12);
    assert_eq!(stream.buffer_size(), 3);
    assert_eq!(stream.peek_output(3), "cat");

    stream.pop_output(3);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), true);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 3);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 15);
    assert_eq!(stream.buffer_size(), 0);

    stream.end_input();

    assert_eq!(stream.input_ended(), true);
    assert_eq!(stream.buffer_empty(), true);
    assert_eq!(stream.eof(), true);
    assert_eq!(stream.bytes_read(), 3);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 15);
    assert_eq!(stream.buffer_size(), 0);
}

#[test]
fn write_pop2_end() {
    let mut stream = ByteStream::new(15);

    assert_eq!(stream.write("cat"), 3);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 0);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 12);
    assert_eq!(stream.buffer_size(), 3);
    assert_eq!(stream.peek_output(3), "cat");

    stream.pop_output(1);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), false);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 1);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 13);
    assert_eq!(stream.buffer_size(), 2);
    assert_eq!(stream.peek_output(2), "at");

    stream.pop_output(2);

    assert_eq!(stream.input_ended(), false);
    assert_eq!(stream.buffer_empty(), true);
    assert_eq!(stream.eof(), false);
    assert_eq!(stream.bytes_read(), 3);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 15);
    assert_eq!(stream.buffer_size(), 0);

    stream.end_input();

    assert_eq!(stream.input_ended(), true);
    assert_eq!(stream.buffer_empty(), true);
    assert_eq!(stream.eof(), true);
    assert_eq!(stream.bytes_read(), 3);
    assert_eq!(stream.bytes_written(), 3);
    assert_eq!(stream.remaining_capacity(), 15);
    assert_eq!(stream.buffer_size(), 0);
}
