# Object detection with Tract and Tensorflow MobileNet v2

This sample is derived from the [tensorflow-mobilenet-v2](https://github.com/sonos/tract/tree/main/examples/tensorflow-mobilenet-v2) example in the [sonos/tract project](https://github.com/sonos/tract).

## Important: Before compiling, obtaining the model

💡 This sample requires some model files to be downloaded before you can build it. For the sake of ergonomics, however, you can do `just build` or `just run` instead of running cargo directly, which will automatically take care of the models for you.

MobileNet is a response to the ImageNet challenge. The goal is to categorize
images and associate them with one of 1000 labels. In other words, recognize a
dog, a cat, a rabbit, or a military uniform.

See https://github.com/tensorflow/models/tree/master/research/slim/nets/mobilenet for more information.

You will need to download the models with the following command:

```sh
just get-models
```

This dumps a half-dozen files in the directory. The only files we care about are `imagenet_slim_labels.txt` and `mobilenet_v2_1.4_224_frozen.pb`, both of which will be automatically inlined into the tract-object-detection binary when it is compiled.

## Running object detection locally

This app will run object detection on any image given to it over stdin. There are a handful of example images in this repository:

```sh
just run < examples/grace_hopper.jpg
```

This should print:

```
military uniform (33.37%)
suit (12.55%)
bulletproof vest (9.98%)
academic gown (5.73%)
```

## Compiling to WASM and running on Serval

`release` builds are noticeably faster than `debug` builds:

```sh
just build --target wasm32-wasi --release
```

Then, from the `serval-mesh` project (assuming this repository is checked out as a sibling directory):

```sh
just run --bin pounce -- run ../wasm-samples/tract-object-detection/target/wasm32-wasi/release/tract-object-detection.wasm ../wasm-samples/tract-object-detection/examples/grace_hopper.jpg
```
