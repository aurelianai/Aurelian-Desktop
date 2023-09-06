import { error } from '@sveltejs/kit'
import type { LayoutLoad } from './$types'
import { ChatStore, getChats } from '.'

export const load = (async () => {
   console.log("setting chat store to", JSON.stringify(await getChats()))
   try {
      ChatStore.set(await getChats())
   } catch (err) {
      throw error(500, {
         message: `Error occured loading your chats: ${JSON.stringify(err)}`
      })
   }
}) satisfies LayoutLoad