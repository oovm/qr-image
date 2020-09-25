QR Image Embed
==============

![](https://i.loli.net/2020/11/24/cUrPKjnv7upsFZq.png)

## Web

- Online preview: https://galaster.github.io/qr-image

## CLI

```yaml
QR Image Embed 0.1.0

USAGE:
    qr-image.exe [OPTIONS] <INPUT> <Text>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --ec <ec>              Set EC level
    -e, --enhance <enhance>    Set enhanced mode
        --qr <qr>              Set QR Version
    -s, --size <size>          Set output image size

ARGS:
    <INPUT>    Sets the input image file path
    <Text>     Sets the qr text for encoding
```

## Algorithm

- [Halftone QR Codes](http://vecg.cs.ucl.ac.uk/Projects/SmartGeometry/halftone_QR/paper_docs/halftoneQR_sigga13.pdf)