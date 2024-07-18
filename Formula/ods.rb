class Ods < Formula
  version "0.0.1"
  desc "CLI for Open Data Schema"
  homepage "https://github.com/open-data-schema/ods"
  license "MIT"

  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/open-data-schema/ods/releases/download/v#{version}/ods-v#{version}-aarch64-apple-darwin.zip"
      sha256 "10b9dc321187bf6782bf7ddee45fc5364dc0e9f18ee82c32da75736af43f5d96"
    else
      url "https://github.com/open-data-schema/ods/releases/download/v#{version}/ods-v#{version}-x86_64-apple-darwin.zip"
      sha256 "8c17e07759c6b950d23d8fde9279b176689fc645c250e734c9c32b1f35730351"
    end
  elsif OS.linux?
     url "https://github.com/open-data-schema/ods/releases/download/v#{version}/ods-v#{version}-x86_64-unknown-linux-gnu.zip"
     sha256 "2ca58310e5bf04400a4749f9e30293d510e1ee27be373c1d99d5c274086fbc61"
  end

  def install
    bin.install "ods"
  end

  test do
    system "#{bin}/ods --version"
  end
end
