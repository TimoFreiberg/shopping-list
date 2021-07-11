type Props = {
    openDoneItems: () => void
}
const OpenDoneItemsButton = ({ openDoneItems }: Props) => <button onClick={ openDoneItems}>[v]</button>

export default OpenDoneItemsButton
