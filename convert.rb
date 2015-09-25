require 'ffi'

module Convert
  extend FFI::Library
  ffi_lib './target/debug/libconvert.dylib'

  # TODO: figure out why I need a no_op call for ffi functions to work
  attach_function :no_op, [], :void

  attach_function :string_to_libc_char_test, [], :string
  attach_function :i32_to_libc_char_test, [], :string
  attach_function :libc_char_to_libc_char_test, [], :string
end
