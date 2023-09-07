<script lang="ts">
	import { AppBar } from '@skeletonlabs/skeleton';
	import { page } from '$app/stores';

	let chat_active = false;
	let chat_hover = false;
	let model_active = false;
	let model_hover = false;

	page.subscribe((p) => {
		console.log(p.url.pathname);
		model_active = p.url.pathname.startsWith('/model');
		chat_active = p.url.pathname.startsWith('/chat');
	});
</script>

<!-- border="border-b border-black" -->
<AppBar
	class="justify-center w-full h-16"
	background="bg-surface-50-900-token"
	shadow="shadow"
	gap="gap-10"
>
	<!-- svelte-ignore a11y-mouse-events-have-key-events -->
	<a
		href="/chat"
		class:variant-filled-primary={chat_active}
		class:variant-soft-surface={chat_hover && !chat_active}
		class="w-20 p-2 font-bold rounded-md btn"
		on:mouseenter={() => (chat_hover = true)}
		on:mouseleave={() => (chat_hover = false)}
	>
		Chat
	</a>

	<!-- svelte-ignore a11y-mouse-events-have-key-events -->
	<a
		href="/model"
		class:variant-filled-primary={model_active}
		class:variant-soft-surface={model_hover && !model_active}
		class="w-20 p-2 font-bold rounded-md btn"
		on:mouseover={() => (model_hover = true)}
		on:mouseout={() => (model_hover = false)}
	>
		Models
	</a>
</AppBar>
