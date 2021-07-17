import type { Item } from '../types'
import DoneItems from './DoneItems'

interface Props {
    items?: Item[]
    undoItem: (item: Item) => void
    showDoneItems: boolean
    setShowDoneItems: (showDoneItems: boolean) => void
}

export default function DoneItemSection({ items, undoItem, showDoneItems, setShowDoneItems }: Props) {
    return (
        <div>
            <label htmlFor='toggleDoneItems'>Show done items</label>
            <input
                id='toggleDoneItems'
                type='checkbox'
                checked={showDoneItems}
                onChange={() => setShowDoneItems(!showDoneItems)} />
            {showDoneItems && items && <DoneItems undoItem={undoItem} items={items} />}
        </div>
    )
}
