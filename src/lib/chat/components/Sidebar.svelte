<script lang="ts">
	import { LightSwitch, popup } from '@skeletonlabs/skeleton';
	import { Icon, ArrowLeftOnRectangle, Cog6Tooth } from 'svelte-hero-icons';
	import SidebarItem from './SidebarItem.svelte';
	import type { PopupSettings } from '@skeletonlabs/skeleton';
	import { ChatStore, deleteChat } from '$lib/chat/ts';
	import { goto } from '$app/navigation';

	const delete_event_handler = async (event: any) => {
		await deleteChat(event.detail.id);
		$ChatStore = $ChatStore.filter((item) => {
			return item.id !== event.detail.id;
		});
		goto('/chat');
	};

	const userPopupBox: PopupSettings = {
		event: 'click',
		target: 'userPopupBox',
		placement: 'top',
		closeQuery: ''
	};
</script>

<div class="fixed top-0 left-0 w-64 px-3 pt-4 menu-button-bg">
	<button
		class="w-full h-10 font-bold rounded-md btn text-md variant-filled-primary"
		on:click={async () => {
			goto('/chat');
		}}
	>
		New Chat
	</button>
</div>

<div class="px-3 pt-2">
	<nav>
		<ul class="space-y-1">
			<div class="w-full h-16" />
			{#each $ChatStore as chat}
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<li>
					<SidebarItem {chat} on:delete={delete_event_handler} />
				</li>
			{/each}
			<div class="w-full h-16" />
		</ul>
	</nav>
</div>

<div class="fixed bottom-0 left-0 w-64 px-3 pb-2 space-y-3 menu-button-bg">
	<hr class="opacity-100" />
	<div class="flex items-center justify-around w-full space-x-3">
		<button
			class="flex items-center justify-center p-2 space-x-2 rounded-md hover:variant-soft-surface"
			use:popup={userPopupBox}
		>
			<Icon src={Cog6Tooth} class="w-7" />
			<p class="w-16 h-6 font-semibold truncate">Settings</p>
		</button>

		<div class="p-2"><LightSwitch rounded="rounded-md" /></div>
	</div>
</div>

<!-- Settings Popup -->
<button class="w-40 space-y-2 rounded-md card" data-popup="userPopupBox">
	<div
		class="flex items-center w-full p-2 space-x-2 rounded-md justify-left hover:variant-soft-surface"
	>
		<Icon src={ArrowLeftOnRectangle} class="w-5" />
		<div class="font-medium">TODO</div>
	</div>
</button>

<style>
	.menu-button-bg {
		background-color: #dfe0e2;
	}
	:is(.dark .menu-button-bg) {
		background-color: #16171e;
	}
</style>
