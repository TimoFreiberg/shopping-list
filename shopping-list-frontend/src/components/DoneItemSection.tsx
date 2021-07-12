import { Item } from '../Types'
import CloseDoneItemsButton from './CloseDoneItemsButton'
import DoneItems from './DoneItems'
import OpenDoneItemsButton from './OpenDoneItemsButton'

type Props = {
    items: Item[]
    undoItem: (item: Item) => void
    doneItemsCollapsed: boolean
    setDoneItemsCollapsed: (collapsed: boolean) => void
}

const DoneItemSection = ({ items, undoItem, doneItemsCollapsed, setDoneItemsCollapsed }: Props) => {
    if (doneItemsCollapsed) {
        const openDoneItems = () => setDoneItemsCollapsed(false)
        return <div>
            <OpenDoneItemsButton openDoneItems={openDoneItems} />
            show completed
        </div>
    }
    const closeDoneItems = () => setDoneItemsCollapsed(true)
    return <>
        <div>
            <CloseDoneItemsButton closeDoneItems={closeDoneItems} />
            hide completed
        </div>
        <DoneItems undoItem={undoItem} items={items} />
    </>
}

export default DoneItemSection
