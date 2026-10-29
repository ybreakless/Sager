$dir = "c:\Mein Projekts\Sager\assets"
if (!(Test-Path $dir)) { New-Item -ItemType Directory -Path $dir | Out-Null }

$bytes = [System.Collections.Generic.List[byte]]::new()

# ICONDIR (6 bytes)
$bytes.AddRange([byte[]]@(0, 0, 1, 0, 1, 0))

$imgSize = 40 + 4096 + 128
# ICONDIRENTRY (16 bytes)
$bytes.Add(32) # width
$bytes.Add(32) # height
$bytes.Add(0)  # colors
$bytes.Add(0)  # reserved
$bytes.AddRange([System.BitConverter]::GetBytes([uint16]1)) # planes
$bytes.AddRange([System.BitConverter]::GetBytes([uint16]32)) # bitcount
$bytes.AddRange([System.BitConverter]::GetBytes([uint32]$imgSize)) # image size
$bytes.AddRange([System.BitConverter]::GetBytes([uint32]22)) # offset

# BITMAPINFOHEADER (40 bytes)
$bytes.AddRange([System.BitConverter]::GetBytes([uint32]40)) # biSize
$bytes.AddRange([System.BitConverter]::GetBytes([int32]32))  # biWidth
$bytes.AddRange([System.BitConverter]::GetBytes([int32]64))  # biHeight (doubled for icon)
$bytes.AddRange([System.BitConverter]::GetBytes([uint16]1))  # biPlanes
$bytes.AddRange([System.BitConverter]::GetBytes([uint16]32)) # biBitCount
$bytes.AddRange([System.BitConverter]::GetBytes([uint32]0))  # biCompression
$bytes.AddRange([System.BitConverter]::GetBytes([uint32]4096)) # biSizeImage
$bytes.AddRange([System.BitConverter]::GetBytes([int32]0))
$bytes.AddRange([System.BitConverter]::GetBytes([int32]0))
$bytes.AddRange([System.BitConverter]::GetBytes([uint32]0))
$bytes.AddRange([System.BitConverter]::GetBytes([uint32]0))

# DIB color pixels: bottom to top (y = 31 down to 0)
for ($y = 31; $y -ge 0; $y--) {
    for ($x = 0; $x -lt 32; $x++) {
        $dx = [Math]::Abs($x - 15.5)
        $dy = [Math]::Abs($y - 15.5)
        $dist = [Math]::Pow([Math]::Pow($dx, 4) + [Math]::Pow($dy, 4), 0.25)

        if ($dist -gt 14.5) {
            $bytes.AddRange([byte[]]@(0, 0, 0, 0))
            continue
        }

        if ($dist -ge 13.0) {
            $bytes.AddRange([byte[]]@(233, 165, 14, 255))
            continue
        }

        $is_s = $false
        if ($y -ge 7 -and $y -le 9 -and $x -ge 10 -and $x -le 22) { $is_s = $true }
        if ($y -ge 10 -and $y -le 13 -and $x -ge 10 -and $x -le 14) { $is_s = $true }
        if ($y -ge 14 -and $y -le 16 -and $x -ge 11 -and $x -le 21) { $is_s = $true }
        if ($y -ge 17 -and $y -le 20 -and $x -ge 18 -and $x -le 22) { $is_s = $true }
        if ($y -ge 21 -and $y -le 23 -and $x -ge 10 -and $x -le 22) { $is_s = $true }

        if (($x -eq 10 -and $y -eq 7) -or ($x -eq 22 -and $y -eq 7) -or ($x -eq 10 -and $y -eq 23) -or ($x -eq 22 -and $y -eq 23)) {
            $is_s = $false
        }

        if ($is_s) {
            $t = ($y - 7.0) / 16.0
            if ($t -lt 0.5) {
                $f = $t * 2.0
                $r = [byte](56.0 + (99.0 - 56.0) * $f)
                $g = [byte](189.0 + (102.0 - 189.0) * $f)
                $b = [byte](248.0 + (241.0 - 248.0) * $f)
            } else {
                $f = ($t - 0.5) * 2.0
                $r = [byte](99.0 + (251.0 - 99.0) * $f)
                $g = [byte](102.0 + (191.0 - 102.0) * $f)
                $b = [byte](241.0 + (36.0 - 241.0) * $f)
            }
            $bytes.AddRange([byte[]]@($b, $g, $r, 255))
        } else {
            $bytes.AddRange([byte[]]@(42, 23, 15, 245))
        }
    }
}

for ($i = 0; $i -lt 128; $i++) { $bytes.Add(0) }

$outPath = Join-Path $dir "sager.ico"
[System.IO.File]::WriteAllBytes($outPath, $bytes.ToArray())
Write-Host "Generated assets\sager.ico ($($bytes.Count) bytes)"
