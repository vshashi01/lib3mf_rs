# PowerShell script to download and extract lib3mf release, and copy headers/binaries

param(
    [string]$Version = "2.4.1",
    [string]$Platform = "Windows"
)

$repo = "3MFConsortium/lib3mf"
$zipUrl = "https://github.com/$repo/releases/download/v$Version/lib3mf-$Version-$Platform.zip"
$zipFile = "lib3mf.zip"
$extractDir = "lib3mf_release"

Write-Host "Downloading lib3mf $Version for $Platform..."
Invoke-WebRequest -Uri $zipUrl -OutFile $zipFile

# Ensure the includes directory exists
if (!(Test-Path "includes")) {
    New-Item -ItemType Directory -Path "includes" | Out-Null
}


# Ensure the lib3mf directory exists
if (!(Test-Path "lib3mf")) {
    New-Item -ItemType Directory -Path "lib3mf" | Out-Null
}

Write-Host "Extracting archive..."
Expand-Archive -Path $zipFile -DestinationPath $extractDir -Force

Write-Host "Copying headers to includes/..."
Copy-Item "$extractDir/lib3mf-$Version-$Platform/include/Bindings/C/*" "includes/" -Recurse -Force

Write-Host "Copying Lib to lib3mf/..."
Copy-Item "$extractDir/lib3mf-$Version-$Platform/lib/lib3mf.lib" "lib3mf/" -Recurse -Force

Write-Host "Copying DLL to lib3mf/..."
Copy-Item "$extractDir/lib3mf-$Version-$Platform/bin/*" "lib3mf/" -Recurse -Force

Write-Host "Cleaning up..."
Remove-Item $zipFile
Remove-Item $extractDir -Recurse

Write-Host "Done!"