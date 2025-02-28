class Gpiv2 < Formula
    desc "Visual programming environment for scientific computation"
    homepage "https://github.com/gpilab/gpi-v2"
    url "https://github.com/gpilab/gpi-v2.git",
       revision: "0f45882b0f769b3263458cbf0d1bf3f93bdaf600"
    sha256 "7f1578dac8f0a8d620f39352924f0085932404141516c4780a9bee7efa4c8b0a"
  
    depends_on "rust" => :build
  
    def install
      system "cargo", "install", *std_cargo_args
    end
  
    # test do
    #   assert_match /^gpi /, shell_output("#{bin}/gpi --version")
    # end
  end
