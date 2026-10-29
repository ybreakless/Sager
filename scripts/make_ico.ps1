Add-Type -AssemblyName System.Drawing

$src = [System.Drawing.Bitmap]::FromFile("c:\Mein Projekts\Sager\assets\icon.png")

# Sizes to generate
$sizes = @(16, 24, 32, 48, 64, 128, 256)
$pngStreams = @()

foreach ($size in $sizes) {
    $bmp = New-Object System.Drawing.Bitmap($size, $size, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
    $g.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
    $g.Clear([System.Drawing.Color]::FromArgb(0, 0, 0, 0))

    # Add a subtle glowing rounded background badge so the black padlock is clearly visible on any dark taskbar
    # Or draw a sleek squircle badge
    $badgeRect = New-Object System.Drawing.Rectangle(0, 0, $size, $size)
    $path = New-Object System.Drawing.Drawing2D.GraphicsPath
    $radius = [float]($size * 0.22)
    $diameter = $radius * 2.0
    $rect = New-Object System.Drawing.RectangleF(0.5, 0.5, ($size - 1.0), ($size - 1.0))
    $path.AddArc($rect.X, $rect.Y, $diameter, $diameter, 180, 90)
    $path.AddArc($rect.Right - $diameter, $rect.Y, $diameter, $diameter, 270, 90)
    $path.AddArc($rect.Right - $diameter, $rect.Bottom - $diameter, $diameter, $diameter, 0, 90)
    $path.AddArc($rect.X, $rect.Bottom - $diameter, $diameter, $diameter, 90, 90)
    $path.CloseFigure()

    # Dark slate background with subtle gradient
    $brush = New-Object System.Drawing.Drawing2D.LinearGradientBrush(
        (New-Object System.Drawing.PointF(0, 0)),
        (New-Object System.Drawing.PointF(0, $size)),
        [System.Drawing.Color]::FromArgb(250, 16, 24, 40),
        [System.Drawing.Color]::FromArgb(250, 9, 13, 22)
    )
    $g.FillPath($brush, $path)
    $brush.Dispose()

    # Glowing cyan border
    $pen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(220, 56, 189, 248), [Math]::Max(1.0, [float]($size * 0.04)))
    $g.DrawPath($pen, $path)
    $pen.Dispose()
    $path.Dispose()

    # Now draw the padlock inside with white/cyan lock body and dark star so it pops brilliantly
    # Padlock destination rectangle
    $pad = [int]($size * 0.15)
    $iconDest = New-Object System.Drawing.Rectangle($pad, $pad, ($size - 2 * $pad), ($size - 2 * $pad))

    # We can draw the icon: invert lock to white/cyan or draw the icon
    # Let's create an inverted/white version of the icon for dark background
    $whiteIcon = New-Object System.Drawing.Bitmap($src.Width, $src.Height, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    for ($y = 0; $y -lt $src.Height; $y++) {
        for ($x = 0; $x -lt $src.Width; $x++) {
            $p = $src.GetPixel($x, $y)
            if ($p.A -gt 20) {
                if ($p.R -lt 100 -and $p.G -lt 100 -and $p.B -lt 100) {
                    # Lock body in crisp white / light cyan
                    $whiteIcon.SetPixel($x, $y, [System.Drawing.Color]::FromArgb($p.A, 245, 250, 255))
                } else {
                    # Star cutout in golden amber (#fbbf24) or deep background for contrast
                    $whiteIcon.SetPixel($x, $y, [System.Drawing.Color]::FromArgb($p.A, 251, 191, 36))
                }
            } else {
                $whiteIcon.SetPixel($x, $y, [System.Drawing.Color]::FromArgb(0, 0, 0, 0))
            }
        }
    }

    $g.DrawImage($whiteIcon, $iconDest)
    $whiteIcon.Dispose()
    $g.Dispose()

    $ms = New-Object System.IO.MemoryStream
    $bmp.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
    $bmp.Dispose()
    $pngStreams += $ms
}

$src.Dispose()

# Now write multi-size PNG-compressed Windows ICO file
$icoBytes = [System.Collections.Generic.List[byte]]::new()
$count = $sizes.Count

# ICONDIR (6 bytes)
$icoBytes.AddRange([byte[]]@(0, 0, 1, 0))
$icoBytes.AddRange([System.BitConverter]::GetBytes([uint16]$count))

$offset = 6 + ($count * 16)

# First pass: write ICONDIRENTRY (16 bytes each)
for ($i = 0; $i -lt $count; $i++) {
    $size = $sizes[$i]
    $pngData = $pngStreams[$i].ToArray()
    $len = $pngData.Length

    $wByte = if ($size -ge 256) { [byte]0 } else { [byte]$size }
    $hByte = if ($size -ge 256) { [byte]0 } else { [byte]$size }

    $icoBytes.Add($wByte)
    $icoBytes.Add($hByte)
    $icoBytes.Add(0) # colors
    $icoBytes.Add(0) # reserved
    $icoBytes.AddRange([System.BitConverter]::GetBytes([uint16]1)) # planes
    $icoBytes.AddRange([System.BitConverter]::GetBytes([uint16]32)) # bitcount
    $icoBytes.AddRange([System.BitConverter]::GetBytes([uint32]$len)) # bytes in res
    $icoBytes.AddRange([System.BitConverter]::GetBytes([uint32]$offset)) # offset

    $offset += $len
}

# Second pass: append PNG image bytes
for ($i = 0; $i -lt $count; $i++) {
    $pngData = $pngStreams[$i].ToArray()
    $icoBytes.AddRange($pngData)
    $pngStreams[$i].Dispose()
}

$icoOutPath = "c:\Mein Projekts\Sager\assets\sager.ico"
[System.IO.File]::WriteAllBytes($icoOutPath, $icoBytes.ToArray())
Write-Host "Generated multi-resolution assets\sager.ico ($($icoBytes.Count) bytes, sizes: $($sizes -join ', '))"
