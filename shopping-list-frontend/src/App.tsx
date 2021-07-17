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
  const [doneItems, setDoneItems] = useState<Item[]>();
  const [showDoneItems, setShowDoneItems] = useState(false);

  const handleResponse = (resp: ItemsResponse) => {
    setOpenItems(resp.open);
    if (resp.done) {
      setDoneItems(resp.done);
    }
  };

  useEffect(
    () => { backend.getItems(showDoneItems).then(handleResponse); },
    [showDoneItems, backend]
  );

  const addItem = (newItem: Item) => { backend.addItem(newItem, showDoneItems).then(handleResponse); };
  const finishItem = (item: Item) => { backend.finishItem(item, showDoneItems).then(handleResponse); };
  const undoItem = (item: Item) => { backend.undoItem(item, showDoneItems).then(handleResponse); };
  const editItem = (item: Item) => { backend.editItem(item, showDoneItems).then(handleResponse); };

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
            editItem={editItem}
          />
        }
        <DoneItemSection
          items={doneItems}
          undoItem={undoItem}
          showDoneItems={showDoneItems}
          setShowDoneItems={it => setShowDoneItems(it)}
        />
      </header>
    </div>
  );
}

export default App;
