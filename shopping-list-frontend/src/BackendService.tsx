import { Item } from "./types"
import axios from 'axios'

type BackendService = {
    getItems: (doneItemsCollapsed: boolean) => Promise<ItemsResponse>
    addItem: (item: Item, doneItemsCollapsed: boolean) => Promise<ItemsResponse>
    finishItem: (item: Item, doneItemsCollapsed: boolean) => Promise<ItemsResponse>
    undoItem: (item: Item, doneItemsCollapsed: boolean) => Promise<ItemsResponse>
    editItem: (item: Item, doneItemsCollapsed: boolean) => Promise<ItemsResponse>
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
        const response = (doneItemsCollapsed: boolean) => {
            const done = doneItemsCollapsed ? [] : doneItems
            const resp = { open: openItems, done: done }
            console.log("response", resp)
            return resp
        }
        const getItems = async (doneItemsCollapsed: boolean) => {
            console.log('getItems called')
            return response(doneItemsCollapsed)
        }
        const addItem = async (item: Item, doneItemsCollapsed: boolean) => {
            console.log('addItem called')
            item.id = id
            id += 1
            openItems = openItems.concat(item)
            return response(doneItemsCollapsed)
        }
        const finishItem = async (item: Item, doneItemsCollapsed: boolean) => {
            console.log('finishItem called')
            item.doneAt = new Date()
            openItems = openItems.filter(i => i.id !== item.id)
            doneItems = doneItems.concat(item)
            return response(doneItemsCollapsed)
        }
        const undoItem = async (item: Item, doneItemsCollapsed: boolean) => {
            console.log('undoItem called')
            item.doneAt = undefined
            doneItems = doneItems.filter(i => i.id !== item.id)
            openItems.push(item)
            return response(doneItemsCollapsed)
        }
        const editItem = async (item: Item, doneItemsCollapsed: boolean) => {
            console.log('editItem called')
            openItems = openItems.map(i => i.id === item.id ? item : i)
            return response(doneItemsCollapsed)
        }
        return {
            getItems: getItems,
            addItem: addItem,
            finishItem: finishItem,
            undoItem: undoItem,
            editItem: editItem
        }
    } else {
        const params = (doneItemsCollapsed: boolean) => {
            return {
                params: {
                    done_items_collapsed: doneItemsCollapsed
                }
            }
        }
        return {
            getItems: async (doneItemsCollapsed: boolean) => {
                const resp = await axios.get('/items', {
                    params: {
                        done_items_collapsed: doneItemsCollapsed
                    }
                })
                // FIXME handle errors
                // .catch(e => alert(e))
                console.log("getOpenItems response", resp)
                return resp.data
            },
            addItem: async (item: Item, doneItemsCollapsed: boolean) => {
                console.log("creating item", item)
                const resp = await axios.post(
                    '/items',
                    item,
                    params(doneItemsCollapsed)
                )
                console.log("createItem response", resp)
                return resp.data
            },
            finishItem: async (item: Item, doneItemsCollapsed: boolean) => {
                console.log("finishing item", item)
                const resp = await axios.put(
                    `/items/${item.id}/complete`,
                    null,
                    params(doneItemsCollapsed)
                )
                console.log("finishItem response", resp)
                return resp.data
            },
            undoItem: async (item: Item, doneItemsCollapsed: boolean) => {
                console.log("undoing item", item)
                const resp = await axios.put(
                    `/items/${item.id}/undo`,
                    null,
                    params(doneItemsCollapsed)
                )
                console.log("undoItem response", resp)
                return resp.data
            },
            editItem: async (item: Item, doneItemsCollapsed: boolean) => {
                console.log("editing item", item)
                const resp = await axios.put(
                    `/items/${item.id}`,
                    null,
                    params(doneItemsCollapsed))
                console.log("undoItem response", resp)
                return resp.data
            }
        }
    }
}

export default backendService
export type { BackendService, ItemsResponse }
