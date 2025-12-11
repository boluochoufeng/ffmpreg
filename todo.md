io → container → codec → transform → codec → container → io

Todo

- Minimal WAV pipeline

  > End-to-end pipeline, audible audio.
  - [x] Create project and basic folders (core, containers/wav, codecs/pcm, cli)
  - [x] Implement Packet, Frame, Timebase
  - [x] Read WAV and produce Packets (containers/wav/read.rs)
  - [x] Write Packets back (containers/wav/write.rs)
  - [x] PCM passthrough codec (decode → encode)
  - [x] Connect pipeline: read → decode → encode → write
  - [x] Minimal CLI: ffmpreg -i input.wav -o output.wav
  - [x] Test with a simple WAV file

- Frame inspection / Media info

  > Show internal frame info, minimal ffprobe alternative.
  - [x] Add CLI option --show
  - [x] Iterate over Packets → Frames
  - [x] Display pts, sample count, channels, sample rate
  - [x] Test output with example WAV

- Basic transform

  > Apply simple operation on frames (e.g., gain)
  - [x] Create transforms/gain.rs
  - [x] Implement trait Transform<T>
  - [x] Integrate pipeline: read → decode → transform → encode → write
  - [x] CLI: ffmpreg --apply gain=2.0
  - [x] Test amplified audio

- Multi-file / batch

  > Process multiple files using the same pipeline
  - [x] CLI accepts multiple files or wildcard (folder/\*.wav)
  - [x] Iterate files → pipeline
  - [x] Create separate output for each file
  - [x] Test with 2-3 WAV files

- More containers

  > Add raw video support (Y4M)
  - [x] Create containers/y4m/read.rs and write.rs
  - [x] Parse Y4M header (width, height, framerate, colorspace, aspect ratio)
  - [x] Produce Packets/Frames
  - [x] Minimal pipeline: decode → encode → write
  - [x] CLI: ffmpreg -i input.y4m -o output.y4m
  - [x] Test with a Y4M file (lossless passthrough verified)

- More codecs

  > ADPCM, multi-channel PCM
  - [x] Add ADPCM codec
  - [x] Support multi-channel PCM
  - [x] Pipeline: decode → transform → encode → write
  - [x] Roundtrip tests for each codec

- Chained filters
  > Apply multiple transforms in sequence
  - [x] CLI: ffmpreg --apply gain=2.0 --apply normalize
  - [x] Create transforms/normalize.rs
  - [x] Pipeline applies filters in sequence
  - [x] Test audio with two chained filters
