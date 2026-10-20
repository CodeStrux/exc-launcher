# VHS tape that records the exc README animation.
# Render with: demo/record-demo.sh (runs vhs from inside demo/)
# Docs: https://github.com/charmbracelet/vhs

Output ./exc-launcher-demo.gif

Set Shell "bash"
Set FontSize 16
Set Width 1100
Set Height 640
Set Padding 20
Set Theme "Catppuccin Mocha"
Set TypingSpeed 70ms
Set WindowBar Colorful

# Setup, hidden from the recording: short prompt and the freshly built binary.
Hide
Type `export PS1='$ ' PATH="$PWD/../target/release:$PATH"` Enter
Type "clear" Enter
Show

# 1. Open the picker (starts on the System profile).
Type "exc --config demo-config.toml" Sleep 400ms Enter
Sleep 2s

# 2. Live filter: type to narrow the current profile's grid, Ctrl-U to clear.
Type "proc"
Sleep 1500ms
Ctrl+U
Sleep 800ms

# 3. Switch profiles with Tab (System -> Git -> Network -> Deploy -> System).
Tab
Sleep 900ms
Tab
Sleep 900ms
Tab
Sleep 900ms
Tab
Sleep 900ms

# 4. Move around the grid with the arrow keys.
Down
Sleep 400ms
Up
Sleep 400ms
Right
Sleep 800ms

# 5. Jump straight to an item by its global number.
Type "2"
Sleep 1200ms
Ctrl+U
Sleep 400ms

# 6. Filtering is per profile, so move to Network first, then run ping-host,
#    which prompts for a host (Enter accepts the default 127.0.0.1).
Tab
Sleep 600ms
Tab
Sleep 600ms
Type "ping"
Sleep 800ms
Enter
Sleep 1500ms
Enter
Sleep 4s

# 7. Run directly by name, with a masked secret prompt.
Type "exc --config demo-config.toml deploy-app" Sleep 400ms Enter
Sleep 1s
Type "production" Enter
Sleep 500ms
Type "demo-token-123" Sleep 800ms Enter
Sleep 3s
