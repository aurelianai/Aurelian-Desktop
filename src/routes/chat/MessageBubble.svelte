<script lang="ts">
	import SvelteMarkdown from 'svelte-markdown';
	import { Avatar } from '@skeletonlabs/skeleton';
	import type { Message } from './types';
	export let msg: Message;
</script>

{#if msg.role === 'USER'}
	<div
		class="flex justify-center max-w-2xl p-6 mx-auto space-x-5 rounded-md shadow-xl card"
	>
		<div class="w-8 h-8">
			<Avatar
				initials={'AU'}
				width="w-8 h-8"
				rounded="rounded-md"
				fill="fill-white"
				background="bg-primary-500"
			/>
		</div>

		<div class="flex flex-col w-full">
			<p class="break-all">{msg.content}</p>
		</div>

		<div class="w-8" />
	</div>
{:else}
	<div
		class="flex justify-center max-w-2xl p-6 mx-auto space-x-5 rounded-md shadow-xl model-bg card"
	>
		<div class="w-8 h-8">
			<Avatar
				src="/logo.svg"
				width="w-8 h-8"
				rounded="rounded-md"
				background="bg-inherit"
			/>
		</div>

		<div class="flex flex-col w-full space-y-5">
			<SvelteMarkdown source={msg.content} />
		</div>

		<div class="w-8" />
	</div>
{/if}

<style lang="postcss">
	.model-bg {
		@apply bg-secondary-300;
	}
	:is(.dark .model-bg) {
		@apply bg-secondary-500;
	}
</style>
