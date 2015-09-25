require 'ffi'

module Convert
  extend FFI::Library
  ffi_lib './target/debug/libconvert.dylib'

  attach_function :concat, [:string, :string], :string
  attach_function :sum_to_s, [:int32, :int32], :string
  attach_function :concat_nums, [:int32, :int32], :string
end
