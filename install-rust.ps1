param([string]$channel=${env:channel}, [string]$target=${env:target})

$downloadUrl = "https://static.rust-lang.org/dist/"
$manifest = "channel-rust-${channel}"
$localManifest = "${env:Temp}\${manifest}"
Start-FileDownload "${downloadUrl}${manifest}" -FileName "${localManifest}"

$match = Get-Content "${localManifest}" | Select-String -pattern "${target}.exe" -simplematch
$installer = $match.line
$localInstaller = "${env:Temp}\${installer}"
Start-FileDownload "${downloadUrl}${installer}" -FileName "${localInstaller}"

$installDir = "C:\Rust"
&"${localInstaller}" /VERYSILENT /NORESTART /DIR="${installDir}" | Write-Output
$env:Path += ";${installDir}\bin"

rustc -V
cargo -V
