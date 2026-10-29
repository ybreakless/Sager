Add-Type -AssemblyName System.Drawing

$src = [System.Drawing.Bitmap]::FromFile("c:\Mein Projekts\Sager\assets\logo.png")
$w = $src.Width
$h = $src.Height

# 1. Generate logo_transparent.png (1024x1024 with transparent background)
# Flood fill from (0,0) or check: pixels with R > 240, G > 240, B > 240 connected to outside become transparent.
# The star inside the lock has X around 512, Y around 460.
# The hole inside the lock shackle has X around 512, Y around 300.
# Both outside and shackle hole should be transparent. The star inside the lock body can stay white or transparent.
$transLogo = New-Object System.Drawing.Bitmap($w, $h, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)

# Simple thresholding: Lock body and text have brightness < 100.
# Outside background is > 240. Star inside lock has brightness > 240 and is within Y [400, 520] and X [440, 580].
for ($y = 0; $y -lt $h; $y++) {
    for ($x = 0; $x -lt $w; $x++) {
        $c = $src.GetPixel($x, $y)
        $isDark = ($c.R -lt 128 -and $c.G -lt 128 -and $c.B -lt 128)
        $inStar = ($x -ge 440 -and $x -le 584 -and $y -ge 400 -and $y -le 520)

        if ($isDark) {
            # Black pixel of lock or text
            $transLogo.SetPixel($x, $y, [System.Drawing.Color]::FromArgb(255, $c.R, $c.G, $c.B))
        } elseif ($inStar) {
            # White star inside lock body
            $transLogo.SetPixel($x, $y, [System.Drawing.Color]::FromArgb(255, 255, 255, 255))
        } else {
            # Transparent background
            $transLogo.SetPixel($x, $y, [System.Drawing.Color]::FromArgb(0, 0, 0, 0))
        }
    }
}
$transLogo.Save("c:\Mein Projekts\Sager\assets\logo_transparent.png", [System.Drawing.Imaging.ImageFormat]::Png)
$transLogo.Dispose()

# 2. Crop the padlock emblem (with star) to 512x512
# Padlock bounds are X=[404, 620] (center ~512), Y=[251, 571] (height 321, center ~411)
$emblemCrop = New-Object System.Drawing.Bitmap(512, 512, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
$g = [System.Drawing.Graphics]::FromImage($emblemCrop)
$g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
$g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
$g.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
$g.Clear([System.Drawing.Color]::FromArgb(0, 0, 0, 0))

# Crop rect from source: X=[312, 712] (width 400), Y=[211, 611] (height 400)
$srcCropRect = New-Object System.Drawing.Rectangle(312, 211, 400, 400)
$destRect = New-Object System.Drawing.Rectangle(20, 20, 472, 472)

# Load the transparent logo to draw into emblemCrop
$transLogoImg = [System.Drawing.Bitmap]::FromFile("c:\Mein Projekts\Sager\assets\logo_transparent.png")
$g.DrawImage($transLogoImg, $destRect, $srcCropRect, [System.Drawing.GraphicsUnit]::Pixel)
$g.Dispose()
$transLogoImg.Dispose()
$src.Dispose()

$emblemCrop.Save("c:\Mein Projekts\Sager\assets\icon.png", [System.Drawing.Imaging.ImageFormat]::Png)
$emblemCrop.Dispose()

Write-Host "Generated assets\logo_transparent.png and assets\icon.png"
