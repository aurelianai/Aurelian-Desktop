<script lang="ts">
	import MessageBubble from '../MessageBubble.svelte';
	import Input from '../Input.svelte';
	import type { PageData } from './$types';
	import type { Message } from '../types';
	import { complete, newMessage } from '..';

	export let data: PageData;
	let generating = false;
	let signal: AbortSignal;

	const handle_message_send = async (event: any) => {
		data.messages = [
			...data.messages,
			await newMessage(data.chatid, 'USER', event.detail.message_content)
		];
		generating = true;

		let modelResponse: Message = { role: 'MODEL', content: '' };
		data.messages = [...data.messages, modelResponse];

		const controller = new AbortController();
		signal = controller.signal;

		for await (const update of complete(data.chatid, signal)) {
			// TODO check error
			modelResponse.content += update.delta;
			data.messages[-1] = modelResponse;
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
	<Input on:send_message={handle_message_send} disabled={generating} />
</div>
