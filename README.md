# Aurelian Desktop

Aurelian Desktop is a cross platform application that powers local inference of LLMs. It's inspired by [gpt4all](gpt4all.io) hoping to improve on inference performance and UI.

It uses [tauri](tauri.app) and [SvelteKit](kit.svelte.dev) for the desktop UI. 

[Rustformers](github.com/rustformers/llm) powers inference through a binding to [ggml](https://github.com/ggerganov/ggml).

# Roadmap

- [ ] Chat Interface through tauri commands / events.
- [ ] Install model through link on HF 
- [ ] Automatic GPU offloading

# Development

```
   npm i
   npm run tauri dev
```

The desktop should launch watching for changes in the `src/` and `src-tauri/`.

First build will take decades but following reloads should be pretty quick.

# Releases

Only way to use in through a build. For now, follow steps in **Development**

