<script lang="ts">
	import MessageBubble from './MessageBubble.svelte';
	import Input from './Input.svelte';
	import Suggestions from './Suggestions.svelte';
	import type { Chat, Message } from './types';
	import { ChatStore, newMessage, complete, newChat } from '.';
	import { goto } from '$app/navigation';

	export let messages: Message[] = [];
	let chat: Chat | null = null;
	let generating: boolean = false;
	let signal: AbortSignal;

	const handle_message_send = async (event: any) => {
		generating = true;

		chat = await newChat(event.detail.message_content.slice(0, 22));
		$ChatStore = [chat, ...$ChatStore];

		messages = [
			...messages,
			await newMessage(chat.id, 'USER', event.detail.message_content)
		];

		let modelResponse: Message = { role: 'MODEL', content: '' };
		messages = [...messages, modelResponse];

		const controller = new AbortController();
		signal = controller.signal;

		for await (const update of complete(chat.id, signal)) {
			if (update.err) {
				console.log('Error ocurred during generation:', update.err);
				break;
			}
			modelResponse.content += update.delta;
			messages[-1] = modelResponse;
		}

		generating = false;
		goto(`/chat/${chat.id}`);
	};
</script>

<div class="flex flex-col h-full">
	{#if messages.length !== 0}
		<div class="w-full p-5 space-y-3">
			{#each messages as message}
				<MessageBubble msg={message} />
			{/each}
			<div class="w-full h-16" />
		</div>
	{:else}
		<div class="flex flex-col justify-center h-full">
			<Suggestions />
		</div>
	{/if}
</div>

<div class="fixed right-0 flex justify-center flex-grow left-64 bottom-4">
	<Input on:send_message={handle_message_send} disabled={generating} />
</div>
