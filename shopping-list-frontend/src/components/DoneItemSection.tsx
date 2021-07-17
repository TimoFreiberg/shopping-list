import type { Item } from '../types'
import DoneItems from './DoneItems'

interface Props {
    items: Item[]
    undoItem: (item: Item) => void
    doneItemsCollapsed: boolean
    setDoneItemsCollapsed: (collapsed: boolean) => void
}

export default function DoneItemSection({ items, undoItem, doneItemsCollapsed, setDoneItemsCollapsed }: Props) {
    return (
        <div>
            <label htmlFor='toggleDoneItems'>Show done items</label>
            <input
                id='toggleDoneItems'
                type='checkbox'
                // FIXME rename to showDoneItems
                checked={!doneItemsCollapsed}
                onChange={() => setDoneItemsCollapsed(!doneItemsCollapsed)} />
            {!doneItemsCollapsed && <DoneItems undoItem={undoItem} items={items} />}
        </div>
    )
}
