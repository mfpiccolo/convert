require "minitest/autorun"
require "./convert"

describe Convert do
  describe "#concat" do
    it "concatinates two strings" do
      Convert.concat('toge', 'ther').must_equal 'together'
    end
  end

  describe "#sum_to_s" do
    it "concatinates two strings" do
      Convert.sum_to_s(55, 45).must_equal "100"
    end
  end

  describe "#concat_nums" do
    it "concatinates two strings" do
      Convert.concat_nums(55, 45).must_equal "5545"
    end
  end
end
