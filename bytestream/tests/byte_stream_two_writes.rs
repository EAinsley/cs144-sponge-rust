use bytestream::ByteStream;

#[test]
fn write_write_end_pop_pop() {
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

  assert_eq!(stream.write("tac"), 3);

  assert_eq!(stream.input_ended(), false);
  assert_eq!(stream.buffer_empty(), false);
  assert_eq!(stream.eof(), false);
  assert_eq!(stream.bytes_read(), 0);
  assert_eq!(stream.bytes_written(), 6);
  assert_eq!(stream.remaining_capacity(), 9);
  assert_eq!(stream.buffer_size(), 6);
  assert_eq!(stream.peek_output(6), "cattac");

  stream.end_input();

  assert_eq!(stream.input_ended(), true);
  assert_eq!(stream.buffer_empty(), false);
  assert_eq!(stream.eof(), false);
  assert_eq!(stream.bytes_read(), 0);
  assert_eq!(stream.bytes_written(), 6);
  assert_eq!(stream.remaining_capacity(), 9);
  assert_eq!(stream.buffer_size(), 6);
  assert_eq!(stream.peek_output(6), "cattac");

  stream.pop_output(2);

  assert_eq!(stream.input_ended(), true);
  assert_eq!(stream.buffer_empty(), false);
  assert_eq!(stream.eof(), false);
  assert_eq!(stream.bytes_read(), 2);
  assert_eq!(stream.bytes_written(), 6);
  assert_eq!(stream.remaining_capacity(), 11);
  assert_eq!(stream.buffer_size(), 4);
  assert_eq!(stream.peek_output(4), "ttac");

  stream.pop_output(4);

  assert_eq!(stream.input_ended(), true);
  assert_eq!(stream.buffer_empty(), true);
  assert_eq!(stream.eof(), true);
  assert_eq!(stream.bytes_read(), 6);
  assert_eq!(stream.bytes_written(), 6);
  assert_eq!(stream.remaining_capacity(), 15);
  assert_eq!(stream.buffer_size(), 0);
}
#[test]
fn write_pop_write_end_pop() {
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

  stream.pop_output(2);

  assert_eq!(stream.input_ended(), false);
  assert_eq!(stream.buffer_empty(), false);
  assert_eq!(stream.eof(), false);
  assert_eq!(stream.bytes_read(), 2);
  assert_eq!(stream.bytes_written(), 3);
  assert_eq!(stream.remaining_capacity(), 14);
  assert_eq!(stream.buffer_size(), 1);
  assert_eq!(stream.peek_output(1), "t");

  assert_eq!(stream.write("tac"), 3);

  assert_eq!(stream.input_ended(), false);
  assert_eq!(stream.buffer_empty(), false);
  assert_eq!(stream.eof(), false);
  assert_eq!(stream.bytes_read(), 2);
  assert_eq!(stream.bytes_written(), 6);
  assert_eq!(stream.remaining_capacity(), 11);
  assert_eq!(stream.buffer_size(), 4);
  assert_eq!(stream.peek_output(4), "ttac");

  stream.end_input();

  assert_eq!(stream.input_ended(), true);
  assert_eq!(stream.buffer_empty(), false);
  assert_eq!(stream.eof(), false);
  assert_eq!(stream.bytes_read(), 2);
  assert_eq!(stream.bytes_written(), 6);
  assert_eq!(stream.remaining_capacity(), 11);
  assert_eq!(stream.buffer_size(), 4);
  assert_eq!(stream.peek_output(4), "ttac");

  stream.pop_output(4);

  assert_eq!(stream.input_ended(), true);
  assert_eq!(stream.buffer_empty(), true);
  assert_eq!(stream.eof(), true);
  assert_eq!(stream.bytes_read(), 6);
  assert_eq!(stream.bytes_written(), 6);
  assert_eq!(stream.remaining_capacity(), 15);
  assert_eq!(stream.buffer_size(), 0);
}
