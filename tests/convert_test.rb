require "minitest/autorun"
require 'pry-byebug'
require "./convert"

describe Convert do
  before do
    # TODO: figure out why I need a no_op call for ffi functions to work
    Convert.no_op
  end

  describe "rust string conversion" do
    describe "#string_to_libc_char_test" do
      it "returns a string" do
        Convert.string_to_libc_char_test.must_equal 'string'
      end
    end
  end

  describe "i32 conversion" do
    describe "#i32_to_libc_char_test" do
      it "returns a string of an integer" do
        Convert.i32_to_libc_char_test.must_equal "1"
      end
    end

    describe "#i32_to_string_test" do
      it "returns a string of an integer" do
        Convert.i32_convert_to_string_test.must_equal "1"
      end
    end
  end

  describe "libc_char conversion" do
    describe "#libc_char_to_libc_char_test" do
      it "returns the string" do
        Convert.libc_char_to_libc_char_test.must_equal "a string"
      end
    end
  end
end
