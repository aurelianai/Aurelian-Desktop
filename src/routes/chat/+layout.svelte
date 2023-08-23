<script lang="ts">
	import Sidebar from './Sidebar.svelte';
</script>

<div class="flex h-full">
	<div
		class="fixed w-64 h-full overflow-y-hidden shadow-md hover:overflow-y-auto"
	>
		<Sidebar />
	</div>

	<div
		class="fixed top-0 bottom-0 right-0 overflow-y-auto left-64 bg-[url('/chat-background.svg')] bg-tertiary-50-200-token"
	>
		<slot />
	</div>
</div>
