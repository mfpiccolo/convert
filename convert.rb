require 'ffi'

module Convert
  extend FFI::Library
  ffi_lib './target/debug/libconvert.dylib'

  attach_function :run, [:string], :string
end

puts Convert.run("something")
