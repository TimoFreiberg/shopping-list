import { useState } from 'react'
import { Item } from '../Types'
import CloseDoneItemsButton from './CloseDoneItemsButton'
import DoneItems from './DoneItems'
import OpenDoneItemsButton from './OpenDoneItemsButton'

type Props = {
    items: Item[]
    undoItem: (item: Item) => void
}

const DoneItemSection = ({ items, undoItem }: Props) => {
    const [doneItemsCollapsed, setDoneItemsCollapsed] = useState(true)
    const itemCount = <>{items.length} completed</>
    if (doneItemsCollapsed) {
        const openDoneItems = () => setDoneItemsCollapsed(false)
        return <div>
            <OpenDoneItemsButton openDoneItems={openDoneItems} />
            {itemCount}
        </div>
    }
    const closeDoneItems = () => setDoneItemsCollapsed(true)
    return <>
        <div>
            <CloseDoneItemsButton closeDoneItems={closeDoneItems} />
            {itemCount}
        </div>
        <DoneItems undoItem={undoItem} items={items} />
    </>
}

export default DoneItemSection
