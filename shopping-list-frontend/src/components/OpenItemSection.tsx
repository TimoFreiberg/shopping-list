import { useEffect } from 'react'
import type { Item } from '../types'
import AddItem from './AddItem'
import OpenItem from './OpenItem'

type Props = {
    items: Item[]
    addItem: (item: Item) => void
    finishItem: (item: Item) => void
    editItem: (item: Item) => void
}

export default function OpenItemSection({ items, addItem, finishItem, editItem }: Props) {
    useEffect(() => {
        console.log("rendering **************************************************************", items)
    }, [items])
    return (
        <div>
            <AddItem addItem={addItem} />
            {
                items.map((item, ix) => <OpenItem
                    key={ix}
                    item={item}
                    finishItem={() => finishItem(item)}
                    editItem={(newName) => editItem({ ...item, name: newName })}
                />
                )
            }
        </div>
    )
}
