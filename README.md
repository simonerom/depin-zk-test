#Benchmark RISC Zero zkVM for DePIN

Welcome!
This code is based on the `hello-world` example for the RISC Zero [zkVM]

For a step-by-step guide to building your first zkVM application, we recommend
[this tutorial][tutorial].

The intended use is to benchmark RISC0 performances on some basic logic applied to numerous input messages. It's been tested on runpod.io on an NVIDIA RTX 4090 and H100 PCIe using CUDA acceleration and compared with the same execution on a Macbook Pro M1 16GB. 

# Usage
On a Macbook Pro, just run 
```sh
cargo run -r -F metal
```

On runpod.io, make sure you start an instance from a template that includes CUDA.
I've selected the following:
```text
RunPod Pytorch 2.2.10
runpod/pytorch:2.2.0-py3.10-cuda12.1.1-devel-ubuntu22.04
```


run the following to install Oh My Zsh shell (this is just my own preference!)
```sh
./instshell
```
then install all required packages

```sh
./podstart
```

finally, run
```sh
cargo run -r -F cuda
```

The first compilation may take >5 minutes, please be patient.
The first execution may result too long, just run the second one for a more accurate execution time.



