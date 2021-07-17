type Props = {
    openDoneItems: () => void
}

export default function OpenDoneItemsButton({ openDoneItems }: Props) {
    return <button onClick={openDoneItems}>[v]</button>
}
