CLI DX

- Minimal WAV pipeline

  > End-to-end pipeline, audible audio
  - Command:
    $ ffmpreg -i input.wav -o output.wav
  - Output: WAV written, playable immediately

- Frame inspection / Media info

  > Show frame details, minimal ffprobe
  - Command:
    $ ffmpreg -i input.wav --show
  - Output:
    Frame 0: pts=0, samples=1024, channels=2, rate=44100
    Frame 1: pts=1024, samples=1024, channels=2, rate=44100

- Basic transform

  > Apply simple transform (gain)
  - Command:
    $ ffmpreg -i input.wav -o output.wav --apply gain=2.0
  - Output: audio amplified x2

- Multi-file / batch

  > Process multiple files in one command
  - Command:
    $ ffmpreg --input folder/\*.wav --output out/
  - Output: each file processed into out/

- More containers

  > Support raw video (Y4M)
  - Command:
    $ ffmpreg -i input.y4m -o output.y4m
  - Output: decoded/encoded video frames

- More codecs

  > Encode/decode multiple codecs
  - Command:
    $ ffmpreg -i input.adpcm -o output.wav --codec adpcm
    $ ffmpreg -i input.wav -o output.adpcm --codec adpcm
  - Output: roundtrip decode/encode working

- Chained filters
  > Apply multiple transforms in sequence
  - Command:
    $ ffmpreg -i input.wav -o output.wav --apply gain=2.0 --apply normalize
  - Output: audio amplified and normalized
