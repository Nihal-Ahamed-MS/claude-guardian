class ClaudeGuardian < Formula
  desc "Local observer daemon that intercepts Claude Code hooks and masks PII"
  homepage "https://github.com/Nihal-Ahamed-MS/claude-guardian"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/Nihal-Ahamed-MS/claude-guardian/releases/download/v#{version}/claude-guardian-aarch64-apple-darwin"
      sha256 "ARM_PLACEHOLDER"
    else
      url "https://github.com/Nihal-Ahamed-MS/claude-guardian/releases/download/v#{version}/claude-guardian-x86_64-apple-darwin"
      sha256 "X86_PLACEHOLDER"
    end
  end

  def install
    bin.install stable.url.split("/").last => "claude-guardian"
  end

  def post_install
    system "#{bin}/claude-guardian", "start"
  rescue
    # Daemon start is best-effort at install time.
  end

  def caveats
    <<~EOS
      claude-guardian has been installed and started as a background daemon.

      It intercepts Claude Code hooks on port 7421 and serves the monitoring
      UI at http://localhost:7422

      Useful commands:
        claude-guardian start   # install hooks and start daemon
        claude-guardian stop    # remove hooks and stop daemon
        claude-guardian logs    # open the web UI
    EOS
  end

  test do
    assert_match "claude-guardian", shell_output("#{bin}/claude-guardian --version")
  end
end
