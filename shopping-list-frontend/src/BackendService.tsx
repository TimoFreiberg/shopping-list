import { Item } from "./types"
import axios from 'axios'

type BackendService = {
    getItems: (showDoneItems: boolean) => Promise<ItemsResponse>
    addItem: (item: Item, showDoneItems: boolean) => Promise<ItemsResponse>
    finishItem: (item: Item, showDoneItems: boolean) => Promise<ItemsResponse>
    undoItem: (item: Item, showDoneItems: boolean) => Promise<ItemsResponse>
    editItem: (item: Item, showDoneItems: boolean) => Promise<ItemsResponse>
}

type ItemsResponse = {
    open: Item[],
    done?: Item[]
}

const backendService: () => BackendService = () => {
    if (process.env.NODE_ENV === 'development') {
        console.log("Using in-memory store")
        var id = 0
        var openItems: Item[] = []
        var doneItems: Item[] = []
        const response = (showDoneItems: boolean) => {
            const done = showDoneItems ? doneItems : []
            const resp = { open: openItems, done: done }
            console.log("response", resp)
            return resp
        }
        const getItems = async (showDoneItems: boolean) => {
            console.log('getItems called')
            return response(showDoneItems)
        }
        const addItem = async (item: Item, showDoneItems: boolean) => {
            console.log('addItem called')
            item.id = id
            id += 1
            openItems = openItems.concat(item)
            return response(showDoneItems)
        }
        const finishItem = async (item: Item, showDoneItems: boolean) => {
            console.log('finishItem called')
            item.doneAt = new Date()
            openItems = openItems.filter(i => i.id !== item.id)
            doneItems = doneItems.concat(item)
            return response(showDoneItems)
        }
        const undoItem = async (item: Item, showDoneItems: boolean) => {
            console.log('undoItem called')
            item.doneAt = undefined
            doneItems = doneItems.filter(i => i.id !== item.id)
            openItems.push(item)
            return response(showDoneItems)
        }
        const editItem = async (item: Item, showDoneItems: boolean) => {
            console.log('editItem called')
            openItems = openItems.map(i => i.id === item.id ? item : i)
            return response(showDoneItems)
        }
        return {
            getItems: getItems,
            addItem: addItem,
            finishItem: finishItem,
            undoItem: undoItem,
            editItem: editItem
        }
    } else {
        const params = (showDoneItems: boolean) => {
            return {
                params: {
                    show_done_items/*  */: showDoneItems
                }
            }
        }
        return {
            getItems: async (showDoneItems: boolean) => {
                const resp = await axios.get('/items', {
                    params: {
                        show_done_items: showDoneItems
                    }
                })
                // FIXME handle errors
                // .catch(e => alert(e))
                console.log("getOpenItems response", resp)
                return resp.data
            },
            addItem: async (item: Item, showDoneItems: boolean) => {
                console.log("adding item", item)
                const resp = await axios.post(
                    '/items',
                    item,
                    params(showDoneItems)
                )
                console.log("addItem response", resp)
                return resp.data
            },
            finishItem: async (item: Item, showDoneItems: boolean) => {
                console.log("finishing item", item)
                const resp = await axios.put(
                    `/items/${item.id}/complete`,
                    null,
                    params(showDoneItems)
                )
                console.log("finishItem response", resp)
                return resp.data
            },
            undoItem: async (item: Item, showDoneItems: boolean) => {
                console.log("undoing item", item)
                const resp = await axios.put(
                    `/items/${item.id}/undo`,
                    null,
                    params(showDoneItems)
                )
                console.log("undoItem response", resp)
                return resp.data
            },
            editItem: async (item: Item, showDoneItems: boolean) => {
                console.log("editing item", item)
                const resp = await axios.put(
                    `/items/${item.id}`,
                    null,
                    params(showDoneItems))
                console.log("undoItem response", resp)
                return resp.data
            }
        }
    }
}

export default backendService
export type { BackendService, ItemsResponse }
