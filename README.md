# kornia-paligemma

Rust implementation of Google Paligemma with Candle

## Usage

```bash
cargo run --example inference --release -- -i ./data/gangsters.png -p "cap en" --sample-length 100
```
or with cuda pass the flag `--features cuda`


```bash
Usage: paligemma -i <image-path> -p <text-prompt> [--sample-length <sample-length>]

Generate a description of an image using Google Paligemma

Options:
  -i, --image-path  path to an input image
  -p, --text-prompt prompt to ask the model
  --sample-length   the length of the generated text
  --help, help      display usage information
```

![xucli_sepas](https://github.com/user-attachments/assets/388ea039-d024-4a19-8462-f658856043b9)

```bash
cap enTwo men are sitting under an umbrella, the left man is wearing sunglasses.
16 tokens generated (26.15 token/s)
```

### Inference as a session

For multiple questions about the same image:

```bash
cargo run --example session -- -i ./data/gangsters.png
```
