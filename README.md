# nlp-rs
## Summary
NLP inferencing SPA written in Rust, with pretrained transformers implemented in actix-web server. Frontend based on [Dioxus](https://dioxuslabs.com/) framework with TailwindCSS styling. Made for educational purposes with the aim to produce a web app leveraging pretrained BERT models for a range of NLP tasks (e.g. summarization, sentiment analysis etc.). The app is intended to yield high throughput in inferencing through concurrency. Current implementation relies on calls to REST API backend, introducing latency. Future implementation aims to support in-browser inferencing via WebAssembly.

Current functionality is limited to translation, with support for 4 languages: English, French, Spanish and German. However, the T5 transformer powering translation supports up to 100 languages. The [rust_bert](https://github.com/guillaume-be/rust-bert) library powering inferencing also provides a variety of transformers of different sizes. CUDA-enabled inferencing is supported but untested, with the local server utilizing only an Intel i7 CPU.

## Usage
The complete service can be run by executing the following commands in the frontend and backend directories:

```shell
cd ./dioxus-actix-mlops/frontend
dioxus serve
 ```
```shell
cd ./dioxus-actix-mlops/frontend
cargo run
```
Note that models will be downloaded on first run, therefore the app should be restarted before use.

## Example
![example](https://github.com/amorx1/dioxus-actix-mlops/blob/master/public/example.png?raw=true "Example")
