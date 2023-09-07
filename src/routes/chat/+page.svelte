<script lang="ts">
	import MessageBubble from '$lib/chat/components/MessageBubble.svelte';
	import Input from '$lib/chat/components/Input.svelte';
	import Suggestions from '$lib/chat/components/Suggestions.svelte';
	import type { Chat, Message } from '../../lib/chat/ts/types';
	import {
		ChatStore,
		newChat,
		newMessage,
		updateMessage,
		complete
	} from '../../lib/chat/ts';
	import { goto } from '$app/navigation';
	import { Icon, Stop } from 'svelte-hero-icons';
	import { emit } from '@tauri-apps/api/event';

	export let messages: Message[] = [];
	let chat: Chat | null = null;
	let generating = false;
	let controller: AbortController;
	let signal: AbortSignal;

	const handle_message_send = async (event: any) => {
		generating = true;

		chat = await newChat(event.detail.message_content.slice(0, 22));
		$ChatStore = [chat, ...$ChatStore];

		messages = [
			await newMessage(chat.id, 'USER', event.detail.message_content),
			await newMessage(chat.id, 'MODEL', '')
		];

		controller = new AbortController();
		signal = controller.signal;

		for await (const update of complete(chat.id, signal)) {
			if (update.err) {
				// TODO throw error here
				break;
			}
			if (signal.aborted) {
				await emit('cancel-generation');
				break;
			}
			await updateMessage(update.delta, messages[1].id);
			messages[1].content += update.delta;
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
