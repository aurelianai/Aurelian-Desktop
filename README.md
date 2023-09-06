# Aurelian Desktop

Aurelian Desktop is a cross platform application that powers local inference of LLMs. It's inspired by [gpt4all](gpt4all.io) hoping to improve on inference performance and UI.

It uses [tauri](tauri.app) and [SvelteKit](kit.svelte.dev) for the desktop UI. 

[Rustformers/llm](github.com/rustformers/llm) powers inference through a binding to [ggml](https://github.com/ggerganov/ggml).

# Roadmap

- [X] Chat Interface through tauri commands / events.
- [ ] Model downloads through GUI
  - [ ] Register deep-link to allow downloads from HF card.
  - Something like this: [aurelian://model_name=TheBloke/<model_name>](aurelian://model_name=TheBloke/<model_name>) would initiate a download model popup.
- [ ] Automatic Settings Optimization
  - [ ] If GPU but all layers can't be fit, add as many layers as possible.
  - [ ] If it can all fit in the GPU, completely offload through [candle](https://github.com/huggingface/candle) or [burn](https://github.com/burn-rs/burn).
    - Burn allows for torch backend allowing hf models to be loaded directly.
    - This would require a change in [llm](github.com/rustformers/llm), the inf backend for the app.

# Development

```
   npm i
   npm run tauri dev
```

The desktop should launch watching for changes in the `src/` and `src-tauri/`.

First build will take decades but following reloads should be pretty quick.

# Releases

Only way to use in through a build. For now, follow steps in **Development**

