import { useEffect, useState } from 'react';
import './App.css';
import Header from './components/Header'
import OpenItemSection from './components/OpenItemSection'
import DoneItemSection from './components/DoneItemSection'
import backendService, { BackendService, ItemsResponse } from './BackendService';
import type { Item } from './types';

function App() {
  const backend = backendService()
  return <AppInternal backend={backend} />
}

type Props = {
  backend: BackendService
}
function AppInternal({ backend }: Props) {
  const [openItems, setOpenItems] = useState<Item[]>();
  const [doneItems, setDoneItems] = useState<Item[]>([]);
  const [doneItemsCollapsed, setDoneItemsCollapsed] = useState(true);

  const handleResponse = (resp: ItemsResponse) => {
    setOpenItems(resp.open);
    if (resp.done) {
      setDoneItems(resp.done);
    }
  };

  useEffect(
    () => { backend.getItems(doneItemsCollapsed).then(handleResponse); },
    [doneItemsCollapsed, backend]
  );

  const addItem = (newItem: Item) => { backend.addItem(newItem, doneItemsCollapsed).then(handleResponse); };
  const finishItem = (item: Item) => { backend.finishItem(item, doneItemsCollapsed).then(handleResponse); };
  const undoItem = (item: Item) => { backend.undoItem(item, doneItemsCollapsed).then(handleResponse); };
  const editItem = (item: Item) => { backend.editItem(item, doneItemsCollapsed).then(handleResponse); };

  return (
    <div className="App">
      <header className="App-header">
        <Header />
        {
          openItems &&
          <OpenItemSection
            items={openItems}
            addItem={addItem}
            finishItem={finishItem}
            editItem={editItem} />
        }
        <DoneItemSection
          items={doneItems}
          undoItem={undoItem}
          doneItemsCollapsed={doneItemsCollapsed}
          setDoneItemsCollapsed={it => setDoneItemsCollapsed(it)} />
      </header>
    </div>
  );
}

export default App;
