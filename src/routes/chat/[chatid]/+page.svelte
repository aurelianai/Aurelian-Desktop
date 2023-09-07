<script lang="ts">
	import MessageBubble from '$lib/chat/components/MessageBubble.svelte';
	import Input from '$lib/chat/components/Input.svelte';
	import type { PageData } from './$types';
	import { newMessage, updateMessage, complete } from '$lib/chat/ts';
	import { Icon, Stop } from 'svelte-hero-icons';
	import { emit } from '@tauri-apps/api/event';

	export let data: PageData;
	let generating = false;
	let controller: AbortController;
	let signal: AbortSignal;

	const handle_message_send = async (event: any) => {
		generating = true;

		data.messages = [
			...data.messages,
			await newMessage(data.chatid, 'USER', event.detail.message_content),
			await newMessage(data.chatid, 'MODEL', '')
		];

		controller = new AbortController();
		signal = controller.signal;

		for await (const update of complete(data.chatid, signal)) {
			if (update.err) {
				// TODO throw error here
				break;
			}
			if (signal.aborted) {
				await emit('cancel-generation');
			}
			await updateMessage(
				update.delta,
				data.messages[data.messages.length - 1].id
			);
			data.messages[data.messages.length - 1].content += update.delta;
		}

		generating = false;
	};
</script>

<div class="flex flex-col h-full">
	<div class="w-full p-5 space-y-3">
		{#each data.messages as message}
			<MessageBubble msg={message} />
		{/each}
		<div class="w-full h-16" />
	</div>
</div>

<div class="fixed right-0 flex justify-center flex-grow left-64 bottom-4">
	{#if !generating}
		<Input on:send_message={handle_message_send} />
	{:else}
		<button
			class="btn variant-filled-primary px-2 py-1 space-x-2 rounded-md"
			on:click={() => controller.abort()}
		>
			<span><Icon src={Stop} class="w-8 h-8" /></span>
			<span class="font-semibold">Stop</span>
		</button>
	{/if}
</div>
