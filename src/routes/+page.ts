import type { PageLoad } from './$types'
import { goto } from '$app/navigation'

export const load = (async () => {
   await goto("/chat")
}) satisfies PageLoad