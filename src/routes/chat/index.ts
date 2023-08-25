import type { Chat, Message } from './types'
import { writable } from 'svelte/store'
import { invoke } from '@tauri-apps/api/tauri'
import { type Event, emit, listen } from '@tauri-apps/api/event'
import { Bucket } from './bucket'

export const ChatStore = writable<Chat[]>()

export const getChats = async (): Promise<Chat[]> => {
   return await invoke("get_chats")
}

export const newChat = async (title: string): Promise<Chat> => {
   let id: number = await invoke("insert_chat", {
      new_chat: { title }
   })
   return { id, title }
}

export const updateChat = async (id: number, new_name: string) => {
   return await invoke("update_chat", {
      c: {
         id,
         title: new_name,
      }
   })
}

export const deleteChat = async (id: number) => {
   return await invoke("delete_chat", {
      pkey: id
   })
}

export const getMessages = async (c_id: number): Promise<Message[]> => {
   return await invoke("get_messages", { c_id })
}

export const newMessage = async (chat_id: number, role: "USER" | "MODEL", content: string): Promise<Message> => {
   try {
      return await invoke("insert_message", {
         new_message: {
            role,
            content,
            chat_id,
         },
      })
   } catch (e) {
      console.log(e)
   }

   return {
      role,
      content,
   }
}

export type InferenceUpdate = {
   delta: string,
   err: string | null,
}

export async function* complete(id: number, sig: AbortSignal): AsyncGenerator<InferenceUpdate> {
   let messages = await getMessages(id)
   console.log("Got Messages", JSON.stringify(messages))

   await invoke("load_default_model")

   const events = new Bucket<InferenceUpdate>();

   const unlisten = await listen('click', (event: Event<InferenceUpdate>) => {
      events.push(event.payload)
   })

   await invoke("complete", {
      msgs: messages
   })

   let message = ""

   for await (const ev of events) {
      message += ev.delta
      yield ev
   }

   await newMessage(id, "MODEL", message)

   unlisten()
}