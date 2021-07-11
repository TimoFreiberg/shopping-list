import { Item } from '../Types'
import AddItem from './AddItem'
import OpenItem from './OpenItem'

type Props = {
    items: Item[]
    setItems: (items: Item[]) => void
    addItem: (item: Item) => void
    finishItem: (item: Item) => void
}
const OpenItemSection = ({ items, setItems, addItem, finishItem }: Props) => {
    const editItem = (pos: number, id: number, newName: string) => {
        let item = items[pos]
        if (item.id !== id) {
            console.log("unexpected id in editItem, id: ", id, ", pos: ", pos, ", item at pos: ", item)
            return
        }
        const updated: Item = { ...item, name: newName }
        var newItems: Item[] = []
        if (pos > 1) {
            newItems.push(...items.slice(0, pos - 1))
        }
        newItems.push(updated)
        if (pos < items.length) {
            newItems.push(...items.slice(pos + 1, items.length))
        }
        setItems(newItems)
    }
    const deleteItem = (pos: number, id: number) => {
        let item = items[pos]
        if (item.id !== id) {
            console.log("unexpected id in deleteItem, id: ", id, ", pos: ", pos, ", item at pos: ", item)
            return
        }
        var newItems: Item[] = []
        if (pos > 1) {
            newItems.push(...items.slice(0, pos - 1))
        }
        if (pos < items.length) {
            newItems.push(...items.slice(pos + 1, items.length))
        }
        setItems(newItems)
    }
    console.log("open items", items)
    return <div>
        <AddItem addItem={addItem} />
        {items.map((i, pos) =>
            <OpenItem
                key={i.id}
                item={i}
                finishItem={() => finishItem(i)}
                editItem={(id, newName) => editItem(pos, id, newName)}
                deleteItem={(id) => deleteItem(pos, id)}
            />)}
    </div>
}

export default OpenItemSection
